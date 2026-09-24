//! QR codes for scheme codes: the matrix for a scheme text, and the text in a QR image
//! (ADR-0009: both ends of the QR layer are the daemon's).
//!
//! Everything here is a function of its arguments; reading and writing files is the caller's.
//! The QR codes made here are plain and standard, for testing in the game: no logo, no colour,
//! no styling. Their parameters are fixed so the same text always gives the same matrix:
//!
//! - **byte mode**, one segment: the text is sent exactly as it is, never re-segmented;
//! - **error correction M**, the level of most QR generators, which tolerates some blur when a
//!   code is photographed from a screen. The level of the game's own codes is not known;
//! - **the smallest version** that holds the text, with the level never raised and the mask
//!   chosen by the standard's penalty rule.

use std::io::Cursor;

use png::{BitDepth, ColorType, Decoder, Encoder, Limits, Transformations};
use qrcodegen::{DataTooLong, QrCode, QrCodeEcc, QrSegment, Version};

/// The widest or tallest image accepted for reading, in pixels.
pub const MAX_IMAGE_SIDE: u32 = 8192;

/// The most pixels accepted for reading: a 4K screenshot has 8.3 million.
pub const MAX_IMAGE_PIXELS: u64 = 32 * 1024 * 1024;

/// The largest image file accepted for reading.
pub const MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;

/// The quiet zone around a rendered code, in modules, as the QR standard requires.
pub const QUIET_ZONE: u32 = 4;

/// The most pixels per module a rendering uses. At this scale the largest code, version 40, is
/// under 3 000 pixels per side.
pub const MAX_RENDER_SCALE: u32 = 16;

/// A QR code as modules: `size × size`, row by row, `true` for dark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QrMatrix {
    size: u32,
    modules: Vec<bool>,
}

impl QrMatrix {
    /// Modules per side: 21 for version 1, up to 177 for version 40.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Whether the module at column `x`, row `y` is dark; outside the matrix, light.
    pub fn is_dark(&self, x: u32, y: u32) -> bool {
        x < self.size && y < self.size && self.modules[(y * self.size + x) as usize]
    }
}

/// Why a QR code could not be made or read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QrError {
    /// The text does not fit in a version 40 code at level M.
    TooLong { bits: usize, capacity_bits: usize },
    /// The image is larger than the limits allow.
    ImageTooLarge { width: u32, height: u32 },
    /// The file is larger than [`MAX_IMAGE_BYTES`].
    FileTooLarge { bytes: usize },
    /// The file is not a PNG image this reader can decode.
    NotAnImage { reason: String },
    /// The image holds no readable QR code.
    NoCode,
    /// The image holds more than one QR code, and which one is meant is not known.
    SeveralCodes { count: usize },
    /// A QR code was found but could not be decoded.
    Unreadable { reason: String },
    /// The code's content is not text.
    NotText,
}

/// The QR matrix for a text, with the fixed parameters above.
pub fn encode(text: &str) -> Result<QrMatrix, QrError> {
    let segment = QrSegment::make_bytes(text.as_bytes());
    let code = QrCode::encode_segments_advanced(
        &[segment],
        QrCodeEcc::Medium,
        Version::MIN,
        Version::MAX,
        None,
        false,
    )
    .map_err(|e| match e {
        DataTooLong::DataOverCapacity(bits, capacity_bits) => QrError::TooLong {
            bits,
            capacity_bits,
        },
        DataTooLong::SegmentTooLong => QrError::TooLong {
            bits: text.len() * 8,
            capacity_bits: 0,
        },
    })?;
    // A QR code is at most 177 modules per side, so these conversions are exact.
    let size = code.size() as u32;
    let modules = (0..code.size())
        .flat_map(|y| (0..code.size()).map(move |x| (x, y)))
        .map(|(x, y)| code.get_module(x, y))
        .collect();
    Ok(QrMatrix { size, modules })
}

/// The one QR code in a greyscale image (`0` black, `255` white), as text.
pub fn decode_luma(width: u32, height: u32, luma: &[u8]) -> Result<String, QrError> {
    check_dimensions(width, height)?;
    let (w, h) = (width as usize, height as usize);
    if luma.len() != w * h {
        return Err(QrError::NotAnImage {
            reason: format!("{} bytes for {width}×{height} pixels", luma.len()),
        });
    }
    let mut image = rqrr::PreparedImage::prepare_from_greyscale(w, h, |x, y| luma[y * w + x]);
    let grids = image.detect_grids();
    let grid = match grids.as_slice() {
        [] => return Err(QrError::NoCode),
        [one] => one,
        several => {
            return Err(QrError::SeveralCodes {
                count: several.len(),
            });
        }
    };
    let mut content = Vec::new();
    grid.decode_to(&mut content)
        .map_err(|e| QrError::Unreadable {
            reason: e.to_string(),
        })?;
    String::from_utf8(content).map_err(|_| QrError::NotText)
}

/// The one QR code in a PNG image, as text.
pub fn decode_png(bytes: &[u8]) -> Result<String, QrError> {
    if bytes.len() > MAX_IMAGE_BYTES {
        return Err(QrError::FileTooLarge { bytes: bytes.len() });
    }
    let not_png = |e: png::DecodingError| QrError::NotAnImage {
        reason: e.to_string(),
    };
    let mut decoder = Decoder::new_with_limits(
        Cursor::new(bytes),
        Limits {
            bytes: MAX_IMAGE_PIXELS as usize * 4,
        },
    );
    // Palette, low bit depths and 16-bit samples all become 8-bit samples.
    decoder.set_transformations(Transformations::EXPAND | Transformations::STRIP_16);
    let mut reader = decoder.read_info().map_err(not_png)?;
    let (width, height) = (reader.info().width, reader.info().height);
    check_dimensions(width, height)?;
    let size = reader
        .output_buffer_size()
        .ok_or(QrError::ImageTooLarge { width, height })?;
    let mut buffer = vec![0u8; size];
    let frame = reader.next_frame(&mut buffer).map_err(not_png)?;
    let channels = frame.color_type.samples();
    let pixels = &buffer[..frame.buffer_size()];
    let luma: Vec<u8> = match frame.color_type {
        ColorType::Grayscale | ColorType::GrayscaleAlpha => {
            pixels.chunks_exact(channels).map(|p| p[0]).collect()
        }
        ColorType::Rgb | ColorType::Rgba => pixels
            .chunks_exact(channels)
            .map(|p| luma_of(p[0], p[1], p[2]))
            .collect(),
        // EXPAND turns a palette into RGB before this point.
        ColorType::Indexed => {
            return Err(QrError::NotAnImage {
                reason: "an indexed image was not expanded".to_owned(),
            });
        }
    };
    decode_luma(width, height, &luma)
}

/// A rendering of the matrix as an 8-bit greyscale PNG: `scale` pixels per module (clamped to
/// `1..=MAX_RENDER_SCALE`), with the standard quiet zone, black on white.
pub fn render_png(matrix: &QrMatrix, scale: u32) -> Result<Vec<u8>, png::EncodingError> {
    let (pixels, side) = render_luma(matrix, scale);
    let mut out = Vec::new();
    let mut encoder = Encoder::new(&mut out, side, side);
    encoder.set_color(ColorType::Grayscale);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&pixels)?;
    writer.finish()?;
    Ok(out)
}

/// The greyscale pixels of a rendering, and its side in pixels.
pub fn render_luma(matrix: &QrMatrix, scale: u32) -> (Vec<u8>, u32) {
    let scale = scale.clamp(1, MAX_RENDER_SCALE);
    let side = (matrix.size() + 2 * QUIET_ZONE) * scale;
    let pixels = (0..side)
        .flat_map(|py| (0..side).map(move |px| (px, py)))
        .map(|(px, py)| {
            let module = |p: u32| (p / scale).checked_sub(QUIET_ZONE);
            match (module(px), module(py)) {
                (Some(x), Some(y)) if matrix.is_dark(x, y) => 0,
                _ => 255,
            }
        })
        .collect();
    (pixels, side)
}

fn check_dimensions(width: u32, height: u32) -> Result<(), QrError> {
    let too_large = width > MAX_IMAGE_SIDE
        || height > MAX_IMAGE_SIDE
        || u64::from(width) * u64::from(height) > MAX_IMAGE_PIXELS;
    if too_large || width == 0 || height == 0 {
        return Err(QrError::ImageTooLarge { width, height });
    }
    Ok(())
}

/// ITU-R BT.601 luma in integer arithmetic.
fn luma_of(r: u8, g: u8, b: u8) -> u8 {
    let y = (299 * u32::from(r) + 587 * u32::from(g) + 114 * u32::from(b)) / 1000;
    y as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "eNqrTCxJVChITAEADgoDBQ==";

    #[test]
    fn a_short_text_is_a_version_one_code() {
        let m = encode("yata").expect("fits");
        assert_eq!(m.size(), 21);
        // Every code has a dark top-left finder corner.
        assert!(m.is_dark(0, 0));
        assert!(!m.is_dark(21, 0));
    }

    #[test]
    fn the_same_text_always_gives_the_same_matrix() {
        assert_eq!(encode(TEXT), encode(TEXT));
    }

    #[test]
    fn a_rendered_code_reads_back_as_its_text() {
        let m = encode(TEXT).expect("fits");
        let (luma, side) = render_luma(&m, 4);
        assert_eq!(decode_luma(side, side, &luma), Ok(TEXT.to_owned()));
    }

    #[test]
    fn a_png_rendering_reads_back_as_its_text() {
        let long: String = "eNpl".repeat(213);
        let m = encode(&long).expect("fits");
        let png = render_png(&m, 3).expect("encodable");
        assert_eq!(decode_png(&png), Ok(long));
    }

    #[test]
    fn a_text_beyond_version_forty_is_refused() {
        let text = "A".repeat(2400);
        assert!(matches!(encode(&text), Err(QrError::TooLong { .. })));
    }

    #[test]
    fn an_image_without_a_code_reads_as_none() {
        assert_eq!(decode_luma(64, 64, &[255; 64 * 64]), Err(QrError::NoCode));
    }

    #[test]
    fn the_render_scale_is_bounded() {
        let m = encode("yata").expect("fits");
        let (_, side) = render_luma(&m, u32::MAX);
        assert_eq!(side, (21 + 2 * QUIET_ZONE) * MAX_RENDER_SCALE);
    }

    #[test]
    fn oversized_images_are_refused_before_reading() {
        assert!(matches!(
            decode_luma(MAX_IMAGE_SIDE + 1, 1, &[]),
            Err(QrError::ImageTooLarge { .. })
        ));
        assert!(matches!(
            decode_luma(8000, 8000, &[]),
            Err(QrError::ImageTooLarge { .. })
        ));
    }

    #[test]
    fn bytes_that_are_not_a_png_are_refused() {
        assert!(matches!(
            decode_png(b"not a png"),
            Err(QrError::NotAnImage { .. })
        ));
    }
}
