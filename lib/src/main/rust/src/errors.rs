use davey::errors::*;

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum DisplayableCodeError {
    DataLessThanDesiredLength,
    DesiredLengthNotMultipleOfGroupSize,
    GroupSizeGreaterThanMaxGroupSize,
    OutOfBoundsDataIndex,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum GenerateKeyFingerprintError {
    UnsupportedFormatVersion,
    KeyIsEmpty,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum GeneratePairwiseFingerprintError {
    GenerateKeyFingerprint,
    HashingFailed,
}

#[uniffi::remote(Object)]
pub struct UnsupportedProtocolVersion;

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum InitError {
    UnsupportedProtocolVersion,
    KeyPairGenerationFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ReinitError {
    Init,
    Reset,
    PendingGroup,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ResetError {
    DeletingGroupFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum SetExternalSenderError {
    AlreadyInGroup,
    DeletingGroupFailed,
    DeserializeExternalSender,
    PendingGroup,
}

#[uniffi::remote(Object)]
pub struct CreateKeyPackageError;

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum PendingGroupError {
    NoExternalSender,
    AddingExternalSenderFailed,
    CreatingGroupFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ProcessProposalsError {
    NoGroup,
    DeserializeProposalFailed,
    DeserializeMessageFailed,
    MessageNotPrivateOrPublic,
    MessageProcessingFailed,
    CredentialContentConvertFailed,
    UnexpectedUser,
    StorePendingProposalFailed,
    MessageNotProposal,
    DeserializeProposalRefFailed,
    RemovingPendingProposalFailed,
    RemovingPendingCommitFailed,
    CommitToPendingProposalsFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ProcessWelcomeError {
    AlreadyInGroup,
    NoExternalSender,
    DeserializeWelcomeFailed,
    CreatingStagedWelcomeFailed,
    ExpectedExternalSenderExtension,
    ExpectedOneExternalSender,
    UnexpectedExternalSender,
    DeletingPendingGroupFailed,
    UpdatingRatchetsFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ProcessCommitError {
    NoGroup,
    PendingGroup,
    DeserializeMessage,
    MessageNotPrivateOrPublic,
    MessageForDifferentGroup,
    MergingPendingCommitFailed,
    MergingStagedCommitFailed,
    ProcessingMessageFailed,
    ProcessedMessageNotStagedCommit,
    UpdatingRatchetsFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum GetVerificationCodeError {
    GettingPairwiseFingerprint,
    GeneratingPairwiseFingerprint,
    GeneratingDisplayableCode,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum GetPairwiseFingerprintError {
    NoEstablishedGroup,
    UserNotInGroup,
    GeneratingPairwiseFingerprint,
    GeneratingKeyFingerprint,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum UpdateRatchetsError {
    NoEstablishedGroup,
    ExportingSecretFailed,
    GeneratingDisplayableCode,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum EncryptError {
    NotReady,
    EncryptionFailed,
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum DecryptError {
    NoDecryptorForUser,
    DecryptionFailed,
}

#[uniffi::remote(Object)]
pub struct NoDecryptorForUser;

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum DecryptorDecryptError {
    UnencryptedWhenPassthroughDisabled,
    NoValidCryptorFound {
        media_type: MediaType,
        encrypted_size: usize,
        plaintext_size: usize,
        manager_count: usize,
    },
}

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum ExpiringCipherError {
    GetKey,
    CreatingCipherFailed,
}

#[uniffi::remote(Object)]
pub struct FrameTooSmall;

#[uniffi::remote(Object)]
pub struct InvalidLength;

#[uniffi::remote(Error)]
#[uniffi(flat_error)]
pub enum GetKeyError {
    KeyExpired,
    NextGenerationFailed,
}
