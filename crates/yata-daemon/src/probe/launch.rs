//! The reader process and its named pipe, on Windows (ADR-0006, rules 6–11; `reader-security.md`,
//! R1, R7, R10).
//!
//! The daemon creates the pipe first, as the first instance, refusing remote clients, with a
//! security descriptor whose one entry grants access to the pipe's owner: the user the daemon runs
//! as. An elevated reader of the same user holds that SID and can connect; no one else can. If the
//! descriptor cannot be built the pipe is not created. The reader gets the pipe's name on its
//! command line and nothing else, and the connected client's process id must be the reader's.
//!
//! Elevation is detected, not assumed: the reader is started unelevated, and only when it answers
//! `probe.elevation_required` and exits with code 5 is it started again through UAC, after its
//! file's SHA-256 is checked against the one the caller expects.

use std::fs::File;
use std::io::{self, Read};
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use interprocess::os::windows::named_pipe::{
    DuplexPipeStream, PipeListenerOptions, PipeStream, pipe_mode,
};
use interprocess::os::windows::security_descriptor::SecurityDescriptor;
use sha2::Digest;
use widestring::U16CString;
use yata_protocol::probe::{HandshakeAck, Log, ProbeErrorCode, Reading, Scope};

use super::session::{Session, SessionError, Step, Target};

/// One access-allowed entry, for the object's owner, and no inheritance (`P`): R10.
pub const PIPE_SDDL: &str = "D:P(A;;GA;;;OW)";

/// How long the daemon waits for a started reader to connect.
pub const CONNECT_WAIT: Duration = Duration::from_secs(30);

/// How long the daemon waits for any frame once the session is open.
pub const FRAME_WAIT: Duration = Duration::from_secs(600);

/// `CREATE_NO_WINDOW`: the reader has no console for anyone to see.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// What the elevation helper exits with when the user declines the UAC prompt.
const DECLINED_EXIT: i32 = 23;

/// A SHA-256 digest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sha256([u8; 32]);

impl Sha256 {
    /// Exactly 64 hexadecimal digits, of either case, and nothing else.
    pub fn parse(text: &str) -> Option<Sha256> {
        let digits = text.as_bytes();
        if digits.len() != 64 || !digits.iter().all(u8::is_ascii_hexdigit) {
            return None;
        }
        let nibble = |d: u8| match d {
            b'0'..=b'9' => d - b'0',
            b'a'..=b'f' => d - b'a' + 10,
            _ => d - b'A' + 10,
        };
        let mut out = [0u8; 32];
        for (byte, pair) in out.iter_mut().zip(digits.chunks_exact(2)) {
            *byte = nibble(pair[0]) << 4 | nibble(pair[1]);
        }
        Some(Sha256(out))
    }

    /// The digest of a file's contents.
    pub fn of_file(path: &Path) -> io::Result<Sha256> {
        let mut file = File::open(path)?;
        let mut hasher = sha2::Sha256::new();
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
        Ok(Sha256(hasher.finalize().into()))
    }
}

impl std::fmt::Display for Sha256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|b| write!(f, "{b:02x}"))
    }
}

/// Whether the reader ran with the user's ordinary rights or through UAC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevation {
    Unelevated,
    Elevated,
}

/// A started reader: a child the daemon owns, or an elevated process it knows only by id.
enum Started {
    Child(Child),
    Elevated { pid: u32 },
}

impl Started {
    fn pid(&self) -> u32 {
        match self {
            Started::Child(c) => c.id(),
            Started::Elevated { pid } => *pid,
        }
    }
}

#[derive(Debug)]
pub enum LaunchError {
    /// No random bytes for the pipe name.
    Random(String),
    /// The pipe or its security descriptor could not be created; nothing was started (R10).
    Pipe(io::Error),
    /// The reader could not be started.
    Spawn(io::Error),
    /// The reader did not connect within [`CONNECT_WAIT`].
    NoConnection,
    /// A process other than the reader connected (ADR-0006, rule 7).
    WrongClient { expected: u32, connected: u32 },
    /// The reader file's hash is not the expected one, so it is not elevated (R7).
    Unverified { found: Sha256 },
    /// Elevation is needed and the caller gave no expected hash to check the reader against.
    NoExpectedHash,
    /// The user declined the UAC prompt: `import.elevation_declined`.
    ElevationDeclined,
    /// The session failed.
    Session(SessionError),
}

/// Everything a read produced.
#[derive(Debug)]
pub struct Outcome {
    pub ack: HandshakeAck,
    pub reading: Reading,
    pub logs: Vec<Log>,
    pub elevation: Elevation,
}

/// What a read is asked to do.
pub struct ReadOptions<'a> {
    pub reader: &'a Path,
    pub target: Target,
    /// Where the frames of the final attempt are recorded.
    pub record: Option<&'a Path>,
    /// The SHA-256 the reader file must have before it is elevated.
    pub expected_sha256: Option<Sha256>,
}

/// A random, per-session pipe name.
pub fn pipe_name() -> Result<String, LaunchError> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| LaunchError::Random(e.to_string()))?;
    let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
    Ok(format!(r"\\.\pipe\yata-reader-{hex}"))
}

/// Create the pipe with [`PIPE_SDDL`], as the first instance, refusing remote clients.
pub fn create_pipe(
    name: &str,
) -> Result<
    interprocess::os::windows::named_pipe::PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>,
    LaunchError,
> {
    let sddl = U16CString::from_str(PIPE_SDDL)
        .map_err(|e| LaunchError::Pipe(io::Error::other(e.to_string())))?;
    let descriptor = SecurityDescriptor::deserialize(&sddl).map_err(LaunchError::Pipe)?;
    let options = PipeListenerOptions::new()
        .path(name)
        .accept_remote(false)
        .security_descriptor(Some(descriptor));
    options
        .create_duplex::<pipe_mode::Bytes>()
        .map_err(LaunchError::Pipe)
}

/// Wait for one client, check it is `expected_pid`, and stop listening.
pub fn accept_from(
    name: &str,
    listener: interprocess::os::windows::named_pipe::PipeListener<
        pipe_mode::Bytes,
        pipe_mode::Bytes,
    >,
    expected_pid: u32,
) -> Result<DuplexPipeStream<pipe_mode::Bytes>, LaunchError> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(listener.accept());
    });
    let stream = match rx.recv_timeout(CONNECT_WAIT) {
        Ok(Ok(stream)) => stream,
        Ok(Err(e)) => return Err(LaunchError::Pipe(e)),
        Err(_) => {
            // Wake the waiting accept by connecting to it ourselves, then drop both ends.
            let _ = PipeStream::<pipe_mode::Bytes, pipe_mode::Bytes>::connect_by_path(name);
            return Err(LaunchError::NoConnection);
        }
    };
    let connected = stream.client_process_id().map_err(LaunchError::Pipe)?;
    if connected != expected_pid {
        return Err(LaunchError::WrongClient {
            expected: expected_pid,
            connected,
        });
    }
    Ok(stream)
}

/// Read the souls: unelevated first, then elevated only if the reader asks for it.
pub fn read_souls(
    options: &ReadOptions<'_>,
    observe: &mut dyn FnMut(u64, Option<u64>) -> Step,
) -> Result<Outcome, LaunchError> {
    match attempt(options, Elevation::Unelevated, observe) {
        Err(LaunchError::Session(SessionError::Failed(e)))
            if e.code == ProbeErrorCode::ElevationRequired =>
        {
            let expected = options.expected_sha256.ok_or(LaunchError::NoExpectedHash)?;
            let found = Sha256::of_file(options.reader).map_err(LaunchError::Spawn)?;
            if found != expected {
                return Err(LaunchError::Unverified { found });
            }
            attempt(options, Elevation::Elevated, observe)
        }
        other => other,
    }
}

fn attempt(
    options: &ReadOptions<'_>,
    elevation: Elevation,
    observe: &mut dyn FnMut(u64, Option<u64>) -> Step,
) -> Result<Outcome, LaunchError> {
    let name = pipe_name()?;
    let listener = create_pipe(&name)?;
    let mut started = match elevation {
        Elevation::Elevated => Started::Elevated {
            pid: start_elevated(options.reader, &name)?,
        },
        Elevation::Unelevated => Started::Child(start(options.reader, &name)?),
    };
    let stream = match accept_from(&name, listener, started.pid()) {
        Ok(s) => s,
        Err(e) => {
            if let Started::Child(c) = &mut started {
                let _ = c.kill();
                let _ = c.wait();
            }
            return Err(e);
        }
    };
    let (incoming, outgoing) = stream.split();
    let recorder = options
        .record
        .map(File::create)
        .transpose()
        .map_err(LaunchError::Spawn)?
        .map(|f| Box::new(f) as Box<dyn io::Write + Send>);
    let mut session = Session::start(incoming, outgoing, recorder).with_wait(FRAME_WAIT);
    let read = session
        .handshake(options.target)
        .and_then(|ack| session.read(Scope::Souls, &mut *observe).map(|r| (ack, r)));
    let logs = session.logs().to_vec();
    let closed = session.shutdown();
    if let Started::Child(c) = &mut started {
        // Reaped so it does not outlive the session; its message, not its exit code, says why it
        // stopped, and an elevation-required reader exits with code 5 after saying so.
        c.wait().map_err(LaunchError::Spawn)?;
    }
    let (ack, reading) = read.map_err(LaunchError::Session)?;
    closed.map_err(LaunchError::Session)?;
    Ok(Outcome {
        ack,
        reading,
        logs,
        elevation,
    })
}

fn start(reader: &Path, pipe: &str) -> Result<Child, LaunchError> {
    Command::new(reader)
        .arg("pipe")
        .arg(pipe)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(LaunchError::Spawn)
}

/// Start the reader through UAC and return its process id. The shell's `runas` verb is reached
/// through Windows PowerShell from the system directory, which prints the new process's id; a
/// declined prompt is Win32 error 1223, `ERROR_CANCELLED`.
fn start_elevated(reader: &Path, pipe: &str) -> Result<u32, LaunchError> {
    let script = format!(
        "$ErrorActionPreference = 'Stop'; try {{ \
         $p = Start-Process -FilePath '{}' -ArgumentList 'pipe','{}' -Verb RunAs \
         -WindowStyle Hidden -PassThru; [Console]::Out.Write($p.Id) }} catch {{ \
         $c = $_.Exception.NativeErrorCode; if (-not $c) {{ $c = $_.Exception.InnerException.NativeErrorCode }}; \
         if ($c -eq 1223) {{ exit {DECLINED_EXIT} }} else {{ [Console]::Error.Write($_); exit 1 }} }}",
        quoted(reader)?,
        pipe
    );
    let out = Command::new(powershell()?)
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            &script,
        ])
        .stdin(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(LaunchError::Spawn)?;
    if out.status.code() == Some(DECLINED_EXIT) {
        return Err(LaunchError::ElevationDeclined);
    }
    let text = String::from_utf8_lossy(&out.stdout);
    text.trim().parse().map_err(|_| {
        LaunchError::Spawn(io::Error::other(format!(
            "elevated start failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        )))
    })
}

/// The reader's path inside a single-quoted PowerShell string.
fn quoted(path: &Path) -> Result<String, LaunchError> {
    let s = path
        .to_str()
        .ok_or_else(|| LaunchError::Spawn(io::Error::other("the reader path is not Unicode")))?;
    Ok(s.replace('\'', "''"))
}

fn powershell() -> Result<PathBuf, LaunchError> {
    let root = std::env::var_os("SystemRoot")
        .ok_or_else(|| LaunchError::Spawn(io::Error::other("SystemRoot is not set")))?;
    Ok(PathBuf::from(root).join(r"System32\WindowsPowerShell\v1.0\powershell.exe"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn the_pipe_refuses_everyone_but_its_owner() {
        assert_eq!(PIPE_SDDL, "D:P(A;;GA;;;OW)");
    }

    #[test]
    fn a_pipe_name_is_random_and_local() {
        let a = pipe_name().expect("random");
        let b = pipe_name().expect("random");
        assert_ne!(a, b);
        assert!(a.starts_with(r"\\.\pipe\yata-reader-"));
        assert_eq!(a.len(), r"\\.\pipe\yata-reader-".len() + 32);
    }

    #[test]
    fn the_owner_connects_and_its_process_id_is_checked() {
        let name = pipe_name().expect("random");
        let listener = create_pipe(&name).expect("created");
        let client_name = name.clone();
        let client = std::thread::spawn(move || {
            let mut c = PipeStream::<pipe_mode::Bytes, pipe_mode::Bytes>::connect_by_path(
                client_name.as_str(),
            )
            .expect("the owner may connect");
            c.write_all(b"x").expect("written");
            c
        });
        let mut stream = accept_from(&name, listener, std::process::id()).expect("same process");
        let mut byte = [0u8; 1];
        stream.read_exact(&mut byte).expect("read");
        assert_eq!(&byte, b"x");
        drop(client.join());
    }

    #[test]
    fn a_client_that_is_not_the_reader_is_refused() {
        let name = pipe_name().expect("random");
        let listener = create_pipe(&name).expect("created");
        let client_name = name.clone();
        let client = std::thread::spawn(move || {
            PipeStream::<pipe_mode::Bytes, pipe_mode::Bytes>::connect_by_path(client_name.as_str())
        });
        let refused = accept_from(&name, listener, std::process::id() + 1);
        assert!(matches!(refused, Err(LaunchError::WrongClient { .. })));
        drop(client.join());
    }

    #[test]
    fn the_first_instance_cannot_be_taken_by_a_second_server() {
        let name = pipe_name().expect("random");
        let _first = create_pipe(&name).expect("created");
        assert!(matches!(create_pipe(&name), Err(LaunchError::Pipe(_))));
    }

    #[test]
    fn hashes_are_read_and_written_as_hex_and_nothing_else() {
        let text = "ab".repeat(32);
        let h = Sha256::parse(&text).expect("hex");
        assert_eq!(h.to_string(), text);
        assert_eq!(Sha256::parse(&"AB".repeat(32)), Some(h));
        assert_eq!(Sha256::parse("ab"), None);
        assert_eq!(Sha256::parse(&"zz".repeat(32)), None);
        // A sign is not a digit, though `from_str_radix` would take one.
        assert_eq!(Sha256::parse(&"+f".repeat(32)), None);
        assert_eq!(Sha256::parse(&format!(" {}", "a".repeat(63))), None);
    }

    #[test]
    fn a_file_hash_is_its_sha256() {
        let dir = std::env::temp_dir().join(format!("yata-launch-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let path = dir.join("f");
        std::fs::write(&path, b"abc").expect("written");
        assert_eq!(
            Sha256::of_file(&path).expect("hashed").to_string(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
