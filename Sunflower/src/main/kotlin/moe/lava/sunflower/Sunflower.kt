package moe.lava.sunflower

import uniffi.sunflower.generateKeyFingerprint
import uniffi.sunflower.getPairwiseFingerprint
import uniffi.sunflower.getSessionCode

object Sunflower {
    fun hi(): String {
        val fp = getPairwiseFingerprint(
            generateKeyFingerprint(0u, byteArrayOf(0x20), 20uL),
            generateKeyFingerprint(0u, byteArrayOf(0x21), 20uL),
        )
        val code = getSessionCode(fp)
        return code
    }
}