use std::num::NonZeroU16;
use std::sync::Arc;
use std::u16;

use davey::errors::{CreateKeyPackageError, DecryptError, EncryptError, GetPairwiseFingerprintError, GetVerificationCodeError, InitError, NoDecryptorForUser, ProcessCommitError, ProcessProposalsError, ProcessWelcomeError, ReinitError, ResetError, SetExternalSenderError};
use davey::{Codec, CommitWelcome, DecryptionStats, EncryptionStats, MediaType, ProposalsOperationType, SessionStatus, SigningKeyPair};
use parking_lot::RwLock;

#[derive(uniffi::Object)]
#[uniffi::export(Display)]
pub struct DaveSession {
    inner: Arc<RwLock<davey::DaveSession>>,
}

#[uniffi::remote(Record)]
pub struct CommitWelcome {
    pub commit: Vec<u8>,
    pub welcome: Option<Vec<u8>>,
}

#[uniffi::remote(Record)]
pub struct SigningKeyPair {
    pub private: Vec<u8>,
    pub public: Vec<u8>,
}

#[uniffi::remote(Record)]
pub struct DecryptionStats {
  /// Number of decryption successes
  pub successes: u32,
  /// Number of decryption failures
  pub failures: u32,
  /// Total decryption duration in microseconds
  pub duration: u32,
  /// Total amounts of decryption attempts
  pub attempts: u32,
  /// Total amounts of packets that passed through
  pub passthroughs: u32,
}

#[uniffi::remote(Record)]
pub struct EncryptionStats {
  /// Number of encryption successes
  pub successes: u32,
  /// Number of encryption failures
  pub failures: u32,
  /// Total encryption duration in microseconds
  pub duration: u32,
  /// Total amounts of encryption attempts
  pub attempts: u32,
  /// Maximum attempts reached at encryption
  pub max_attempts: u32,
}

#[derive(Debug, uniffi::Record)]
pub struct FingerprintPair {
    pub local: Vec<u8>,
    pub remote: Vec<u8>,
}

#[uniffi::remote(Enum)]
pub enum SessionStatus {
    INACTIVE,
    PENDING,
    AWAITING_RESPONSE,
    ACTIVE,
}

#[uniffi::remote(Enum)]
pub enum ProposalsOperationType {
    APPEND,
    REVOKE,
}

#[uniffi::remote(Enum)]
pub enum MediaType {
    AUDIO,
    VIDEO,
}

/// The type of codec being used.
#[uniffi::remote(Enum)]
pub enum Codec {
    UNKNOWN,
    OPUS,
    VP8,
    VP9,
    H264,
    H265,
    AV1,
}


fn as_nonzero(num: u16) -> Result<NonZeroU16, InitError> {
    NonZeroU16::new(num)
        .ok_or_else(|| InitError::UnsupportedProtocolVersion(
                davey::errors::UnsupportedProtocolVersion(NonZeroU16::new(u16::MAX).unwrap())
        ))
}

#[uniffi::export]
impl DaveSession {
    #[uniffi::constructor()]
    pub fn new(
        protocol_version: u16,
        user_id: u64,
        channel_id: u64,
        key_pair: Option<SigningKeyPair>,
    ) -> Result<Self, InitError> {
        let ver = as_nonzero(protocol_version)?;
        Ok(DaveSession {
            inner: Arc::new(RwLock::new(davey::DaveSession::new(ver, user_id, channel_id, key_pair.as_ref())?))
        })
    }

    pub fn reinit(
        &self,
        protocol_version: u16,
        user_id: u64,
        channel_id: u64,
        key_pair: Option<SigningKeyPair>,
    ) -> Result<(), ReinitError> {
        self.inner.write().reinit(as_nonzero(protocol_version)?, user_id, channel_id, key_pair.as_ref())?;
        Ok(())
    }

    pub fn reset(&self) -> Result<(), ResetError> {
        self.inner.write().reset()?;
        Ok(())
    }

    pub fn get_protocol_version(&self) -> u16 { self.inner.read().protocol_version().get() }
    pub fn get_user_id(&self) -> u64 { self.inner.read().user_id() }
    pub fn get_channel_id(&self) -> u64 { self.inner.read().channel_id() }
    pub fn get_epoch(&self) -> Option<u64> { self.inner.read().epoch().map(|e| e.as_u64()) }
    pub fn get_own_leaf_index(&self) -> Option<u32> { self.inner.read().own_leaf_index().map(|e| e.u32()) }
    pub fn get_ciphersuite(&self) -> u16 { self.inner.read().ciphersuite() as u16 }
    pub fn get_status(&self) -> SessionStatus { self.inner.read().status() }
    pub fn is_ready(&self) -> bool { self.inner.read().is_ready() }

    pub fn get_epoch_authenticator(&self) -> Option<Vec<u8>> {
        self.inner.read()
            .get_epoch_authenticator()
            .map(|ea| ea.as_slice().to_vec())
    }

    /// Get the voice privacy code of this session's group.
    /// The result of this is created and cached each time a new transition is executed.
    /// This is the equivalent of `generateDisplayableCode(epochAuthenticator, 30, 5)`.
    /// @returns The current voice privacy code, or an empty string if the session is not active.
    /// @see https://daveprotocol.com/#displayable-codes
    pub fn get_voice_privacy_code(&self) -> String {
        self.inner.read()
            .voice_privacy_code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "".to_string())
    }

    /// Set the external sender this session will recieve from.
    /// @param externalSenderData The serialized external sender data.
    /// @throws Will throw if the external sender is invalid, or if the group has been established already.
    /// @see https://daveprotocol.com/#dave_mls_external_sender_package-25
    pub fn set_external_sender(&self, external_sender_data: &[u8]) -> Result<(), SetExternalSenderError> {
        self.inner.write().set_external_sender(&external_sender_data)?;
        Ok(())
    }

    /// Create, store, and return the serialized key package buffer.
    /// Key packages are not meant to be reused, and will be recreated on each call of this function.
    pub fn get_serialized_key_package(&self) -> Result<Vec<u8>, CreateKeyPackageError> {
        Ok(self.inner.write().create_key_package()?)
    }

    /// Process proposals from the voice server.
    /// @param operationType The operation type of the proposals.
    /// @param proposals The vector of proposals or proposal refs of the payload. (depending on operation type)
    /// @param recognizedUserIds The recognized set of user IDs gathered from the voice gateway. Recommended to set so that incoming users are checked against.
    /// @returns A commit (if there were queued proposals) and a welcome (if a member was added) that should be used to send an [opcode 28: dave_mls_commit_welcome](https://daveprotocol.com/#dave_mls_commit_welcome-28) ONLY if a commit was returned.
    /// @see https://daveprotocol.com/#dave_mls_proposals-27
    pub fn process_proposals(
        &self,
        operation_type: ProposalsOperationType,
        proposals: &[u8],
        recognized_user_ids: Option<Vec<u64>>,
    ) -> Result<Option<CommitWelcome>, ProcessProposalsError> {
        self.inner.write()
            .process_proposals(operation_type, &proposals, recognized_user_ids.as_deref())
    }

    /// Process a welcome message.
    /// @param welcome The welcome message to process.
    /// @throws Will throw an error if the welcome is invalid. Send an [opcode 31: dave_mls_invalid_commit_welcome](https://daveprotocol.com/#dave_mls_invalid_commit_welcome-31) if this occurs.
    /// @see https://daveprotocol.com/#dave_mls_welcome-30
    pub fn process_welcome(&self, welcome: &[u8]) -> Result<(), ProcessWelcomeError> {
        self.inner.write().process_welcome(&welcome)
    }

    /// Process a commit.
    /// @param commit The commit to process.
    /// @throws Will throw an error if the commit is invalid. Send an [opcode 31: dave_mls_invalid_commit_welcome](https://daveprotocol.com/#dave_mls_invalid_commit_welcome-31) if this occurs.
    /// @see https://daveprotocol.com/#dave_mls_announce_commit_transition-29
    pub fn process_commit(&self, commit: &[u8]) -> Result<(), ProcessCommitError> {
        self.inner.write().process_commit(&commit)
    }

    /// Get the verification code of another member of the group.
    /// This is the equivalent of `generateDisplayableCode(getPairwiseFingerprint(0, userId), 45, 5)`.
    /// @see https://daveprotocol.com/#displayable-codes
    pub fn get_verification_code(&self, user_id: u64) -> Result<String, GetVerificationCodeError> {
        self.inner.write().get_verification_code(user_id)
    }

    /// Create a pairwise fingerprint of you and another member.
    /// @see https://daveprotocol.com/#verification-fingerprint
    pub fn get_pairwise_fingerprint(
        &self,
        version: u16,
        user_id: u64,
    ) -> Result<FingerprintPair, GetPairwiseFingerprintError> {
        self.inner.read()
            .get_key_fingerprint_pair(version, user_id)
            .map(|fps| {
                let [local, remote] = fps;
                FingerprintPair { local, remote }
            })
    }

    /// End-to-end encrypt a packet.
    /// @param mediaType The type of media to encrypt
    /// @param codec The codec of the packet
    /// @param packet The packet to encrypt
    pub fn encrypt(
        &self,
        media_type: MediaType,
        codec: Codec,
        packet: &[u8],
    ) -> Result<Vec<u8>, EncryptError> {
        self.inner.write()
            .encrypt(media_type, codec, &packet)
            .map(|sl| sl.to_vec())
    }

    /// End-to-end encrypt an opus packet.
    /// This is the shorthand for `encrypt(MediaType.AUDIO, Codec.OPUS, packet)`
    /// @param packet The packet to encrypt
    pub fn encrypt_opus(&self, packet: &[u8]) -> Result<Vec<u8>, EncryptError> {
        self.encrypt(MediaType::AUDIO, Codec::OPUS, packet)
    }

    /// Get encryption stats.
    /// @param [mediaType=MediaType.AUDIO] The media type, defaults to `MediaType.AUDIO`
    pub fn get_encryption_stats(&self, media_type: Option<MediaType>) -> Option<EncryptionStats> {
        self.inner.read()
            .get_encryption_stats(media_type)
            .map(|s| s.to_owned())
    }

    /// Decrypt an end-to-end encrypted packet.
    /// @param userId The user ID of the packet
    /// @param mediaType The type of media to decrypt
    /// @param packet The packet to decrypt
    pub fn decrypt(
        &self,
        user_id: u64,
        media_type: MediaType,
        packet: &[u8],
    ) -> Result<Vec<u8>, DecryptError> {
        self.inner.write()
            .decrypt(user_id, media_type, &packet)
    }

    /// Get decryption stats.
    /// @param userId The user ID
    /// @param [mediaType=MediaType.AUDIO] The media type, defaults to `MediaType.AUDIO`
    pub fn get_decryption_stats(
        &self,
        user_id: u64,
        media_type: Option<MediaType>,
    ) -> Result<Option<DecryptionStats>, NoDecryptorForUser> {
        self.inner.read()
            .get_decryption_stats(user_id, media_type.unwrap_or(MediaType::AUDIO))
            .map(|r| r.map(|o| o.to_owned()))
    }

    /// Get the IDs of the users in the current group.
    /// @returns An array of user IDs, or an empty array if there is no group.
    pub fn get_user_ids(&self) -> Vec<u64> {
        self.inner.read().get_user_ids().unwrap_or_default()
    }

    /// Check whether this user's decryptor is in passthrough mode.
    /// If passthrough mode is enabled, then unencrypted packets are allowed to be passed through the decryptor.
    /// @param userId The user ID
    pub fn can_passthrough(&self, user_id: u64) -> bool {
        self.inner.read().can_passthrough(user_id)
    }

    /// Set whether passthrough mode is enabled on all decryptors.
    /// @param passthroughMode Whether to enable passthrough mode
    /// @param [transitionExpiry=10] The transition expiry (in seconds) to use when disabling passthrough mode, defaults to 10 seconds
    pub fn set_passthrough_mode(&self, passthrough_mode: bool, transition_expiry: Option<u32>) {
        self.inner.write()
            .set_passthrough_mode(passthrough_mode, transition_expiry);
    }
}

impl std::fmt::Display for DaveSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "DAVESession {{ protocolVersion: {}, userId: {}, channelId: {}, ready: {}, status: {:?} }}",
            self.get_protocol_version(),
            self.get_user_id(),
            self.get_channel_id(),
            self.is_ready(),
            self.get_status()
        )
    }
}
