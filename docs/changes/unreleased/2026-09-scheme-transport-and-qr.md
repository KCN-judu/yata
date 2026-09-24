# Scheme codes can be decoded, compared, re-encoded, and turned into QR codes

- Date: 2026-09-24
- Area: domain
- Affected: developers
- Related: ADR-0009

## What changed

- `yata-daemon scheme dump <code>` prints a scheme code's payload as hex;
  `<code>` is a PNG image holding one QR code or a text file holding the Base64
  text.
- `yata-daemon scheme diff <code-a> <code-b>` lists each byte that differs, with
  its XOR and the changed bits as absolute bit offsets.
- `yata-daemon scheme decode <code> <payload.bin>` writes the raw payload;
  `yata-daemon scheme encode <payload.bin> [<qr.png>]` prints the scheme text
  for a payload and writes a scannable QR code.
- Nothing inside the payload is interpreted yet.

## Compatibility and migration

New commands only. A scheme text this encoder produces decodes to the same
payload but is not byte-identical to the game's text for the same payload.

## Evidence

[../../evidence/testing.md](../../evidence/testing.md), § Scheme-code transport
and § QR codes and research commands.
