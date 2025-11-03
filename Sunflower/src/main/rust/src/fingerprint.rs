use davey::errors::{DisplayableCodeError, GenerateKeyFingerprintError, GeneratePairwiseFingerprintError, GetPairwiseFingerprintError};
use scrypt::errors::InvalidOutputLen;

#[uniffi::remote(Enum)]
pub enum GenerateKeyFingerprintError {
    UnsupportedFormatVersion,
    KeyIsEmpty,
}

#[uniffi::remote(Record)]
pub struct InvalidOutputLen;

#[uniffi::remote(Enum)]
pub enum DisplayableCodeError {
  DataLessThanDesiredLength,
  DesiredLengthNotMultipleOfGroupSize,
  GroupSizeGreaterThanMaxGroupSize,
  OutOfBoundsDataIndex,
}

#[uniffi::remote(Enum)]
pub enum GeneratePairwiseFingerprintError {
    GenerateKeyFingerprint(#[from] GenerateKeyFingerprintError),
    HashingFailed(InvalidOutputLen),
}

#[uniffi::remote(Enum)]
pub enum GetPairwiseFingerprintError {
    NoEstablishedGroup,
    UserNotInGroup,
    GeneratingPairwiseFingerprint(#[from] GeneratePairwiseFingerprintError),
    GeneratingKeyFingerprint(#[from] GenerateKeyFingerprintError),
}

/// Generate a key fingerprint.
/// @see https://daveprotocol.com/#verification-fingerprint
/// @param version The version of the fingerprint
/// @param key The key to fingerprint
/// @param userId The user ID of this fingerprint
#[uniffi::export]
pub fn generate_key_fingerprint(
    version: u16,
    key: &[u8],
    user_id: u64,
) -> Result<Vec<u8>, GenerateKeyFingerprintError> {
    davey::generate_key_fingerprint(version, &key, user_id)
}

/// Get a pairwise fingerprint.
/// @see https://daveprotocol.com/#verification-fingerprint
/// @param first The first fingerprint
/// @param second The second fingerprint
#[uniffi::export]
pub fn get_pairwise_fingerprint(
    first: Vec<u8>,
    second: Vec<u8>,
) -> Result<Vec<u8>, GeneratePairwiseFingerprintError> {
    davey::pairwise_fingerprints_internal([first, second])
}

/// Get a session code.
/// @see https://daveprotocol.com/#verification-fingerprint
/// @param fingerprint The pairwise fingerprint
#[uniffi::export]
pub fn get_session_code(
    fingerprint: &[u8],
) -> Result<String, DisplayableCodeError> {
    davey::generate_displayable_code(fingerprint, 45, 5)
}
