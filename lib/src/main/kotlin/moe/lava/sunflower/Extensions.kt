package moe.lava.sunflower

inline val DaveSession.protocolVersion get() = this.getProtocolVersion()
inline val DaveSession.userId get() = this.getUserId()
inline val DaveSession.channelId get() = this.getChannelId()
inline val DaveSession.epoch get() = this.getEpoch()
inline val DaveSession.ownLeafIndex get() = this.getOwnLeafIndex()
inline val DaveSession.ciphersuite get() = this.getCiphersuite()
inline val DaveSession.status get() = this.getStatus()
inline val DaveSession.ready get() = this.isReady()
inline val DaveSession.epochAuthenticator get() = this.getEpochAuthenticator()
inline val DaveSession.voicePrivacyCode get() = this.getVoicePrivacyCode()
inline val DaveSession.serializedKeyPackage get() = this.getSerializedKeyPackage()

@Suppress("NOTHING_TO_INLINE")
inline operator fun DaveSession.Companion.invoke(
    protocolVersion: UShort,
    userId: ULong,
    channelId: ULong,
    keyPair: SigningKeyPair? = null,
): DaveSession {
    if (protocolVersion == 0.toUShort()) {
        throw IllegalArgumentException("Protocol version must not be zero")
    }
    return DaveSession(
        protocolVersion,
        userId,
        channelId,
        keyPair,
    )
}