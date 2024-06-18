// TODO: Create an enum with frame IDs specified in the SDD
// TODO: Crate a struct for every frame that contains a payload so that it can be serialized and deserialized (for temperature, only divide by 100 when displaying the value)
// TODO: Create a constant for the starting byte delimiter in the SDD

/// Decodes a slice of bytes by removing the prepended 0xB3 and verifying the checksum.
///
/// The checksum is verified as the XOR of every byte in the input slice. If the checksum does not match, the function will return an error.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to be decoded. It must have at least 2 bytes: the prepended 0xB3 and the checksum.
///
/// # Returns
///
/// A `Result<Vec<u8>, &'static str>` containing the decoded bytes or an error message if the checksum is invalid.
///
/// # Example
///
/// ```
/// let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03];
/// let decoded = decode(&encoded).unwrap();
/// assert_eq!(decoded, vec![0x01, 0x02, 0x03]);
/// ```
enum FrameId {
	Ping = 0x00,
	AssignID = 0x01,
	RequestData = 0x02,
	SetStandby = 0x04,
	SetDischarge = 0x05,
	SetCharge = 0x06,
	AnnounceCompletion = 0x07	
}

struct Frame {

}

pub fn decode(bytes: &[u8]) -> Result<Vec<u8>, &'static str> {
	todo!()
}

/// Encodes a slice of bytes by prepending 0xB3 and appending a checksum.
///
/// The checksum is calculated as the XOR of every byte in the resulting slice, including the prepended 0xB3.
///
/// # Arguments
///
/// * `bytes` - A slice of bytes to be encoded.
///
/// # Returns
///
/// A `Vec<u8>` containing the encoded bytes.
///
/// # Example
///
/// ```
/// let data = vec![0x01, 0x02, 0x03];
/// let encoded = encode(&data);
/// assert_eq!(encoded, vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03]);
/// ```
pub fn encode(bytes: &[u8]) -> Vec<u8> {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encode() {
		let data = vec![0x01, 0x02, 0x03];
		let encoded = encode(&data);
		assert_eq!(encoded, vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03]);

		let data = vec![0x00, 0xFF, 0x55];
		let encoded = encode(&data);
		assert_eq!(encoded, vec![0xB3, 0x00, 0xFF, 0x55, 0xB3 ^ 0x00 ^ 0xFF ^ 0x55]);
	}

	#[test]
	fn test_decode() {
		let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0xB3 ^ 0x01 ^ 0x02 ^ 0x03];
		let decoded = decode(&encoded).unwrap();
		assert_eq!(decoded, vec![0x01, 0x02, 0x03]);

		let encoded = vec![0xB3, 0x00, 0xFF, 0x55, 0xB3 ^ 0x00 ^ 0xFF ^ 0x55];
		let decoded = decode(&encoded).unwrap();
		assert_eq!(decoded, vec![0x00, 0xFF, 0x55]);
	}

	#[test]
	fn test_decode_invalid_checksum() {
		let encoded = vec![0xB3, 0x01, 0x02, 0x03, 0x00];
		let result = decode(&encoded);
		assert!(result.is_err());
		assert_eq!(result.err(), Some("Invalid checksum"));
	}

	#[test]
	fn test_decode_too_short() {
		let encoded = vec![0xB3];
		let result = decode(&encoded);
		assert!(result.is_err());
		assert_eq!(result.err(), Some("Input is too short to be valid"));
	}
}