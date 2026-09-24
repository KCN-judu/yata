use std::fmt;

use super::transport::{MAX_PAYLOAD_LEN, TransportError};

/// The game's scheme payload after the transport layers are removed: the exact decompressed
/// bytes, in order.
///
/// Holding one says nothing about what the bytes mean. No part of it is interpreted here, and
/// nothing normalizes, trims, or rewrites it: two payloads are equal only if every byte is.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RawSchemePayload(Vec<u8>);

impl RawSchemePayload {
    /// A payload from bytes: rejected if empty (no scheme is empty) or above
    /// [`MAX_PAYLOAD_LEN`].
    pub fn new(bytes: Vec<u8>) -> Result<RawSchemePayload, TransportError> {
        if bytes.is_empty() {
            return Err(TransportError::EmptyPayload);
        }
        if bytes.len() > MAX_PAYLOAD_LEN {
            return Err(TransportError::PayloadTooLarge {
                limit: MAX_PAYLOAD_LEN,
            });
        }
        Ok(RawSchemePayload(bytes))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always false: an empty payload cannot be constructed.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.0
    }
}

/// The length and every byte in hex, so a failing test shows the exact payload.
impl fmt::Debug for RawSchemePayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RawSchemePayload({} bytes: ", self.0.len())?;
        for b in &self.0 {
            write!(f, "{b:02x}")?;
        }
        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_payload_cannot_be_made() {
        assert_eq!(
            RawSchemePayload::new(vec![]),
            Err(TransportError::EmptyPayload)
        );
    }

    #[test]
    fn the_largest_accepted_payload_is_the_limit() {
        assert!(RawSchemePayload::new(vec![0; MAX_PAYLOAD_LEN]).is_ok());
        assert_eq!(
            RawSchemePayload::new(vec![0; MAX_PAYLOAD_LEN + 1]),
            Err(TransportError::PayloadTooLarge {
                limit: MAX_PAYLOAD_LEN
            })
        );
    }

    #[test]
    fn payloads_are_equal_only_byte_for_byte() {
        let a = RawSchemePayload::new(vec![0x45, 0x53, 0x00]).expect("valid");
        let b = RawSchemePayload::new(vec![0x45, 0x53, 0x00]).expect("valid");
        let c = RawSchemePayload::new(vec![0x45, 0x53, 0x01]).expect("valid");
        let longer = RawSchemePayload::new(vec![0x45, 0x53, 0x00, 0x00]).expect("valid");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, longer);
    }

    #[test]
    fn debug_shows_every_byte() {
        let p = RawSchemePayload::new(vec![0x45, 0x0a]).expect("valid");
        assert_eq!(format!("{p:?}"), "RawSchemePayload(2 bytes: 450a)");
    }
}
