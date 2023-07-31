package org.dashj.dpp

import org.dashj.platform.dpp.identity.IdentityCreateTransition

object DPP {
    init {
        System.loadLibrary("dashj_dpp_bindings")
    }
    external fun createIdentityCbor(): ByteArray
    external fun convertMap(map: Map<String, Any>)
    external fun createIdentityFromRawObject(map: Map<String, Any>): ByteArray
    external fun serializeIdentityFromRawObject(map: Map<String, Any?>): ByteArray

    external fun serializeIdentityCreateTransition(map: Map<String, Any?>): ByteArray
    external fun signableBytesIdentityCreateTransition(map: Map<String, Any?>): ByteArray
    external fun validateIdentityCreateTransition(map: Map<String, Any?>): Boolean
    external fun getIdentityCborFromBincode(bytes: ByteArray) : ByteArray


    fun serialize(identityCreateTransition: IdentityCreateTransition) : ByteArray {
        return serializeIdentityCreateTransition(identityCreateTransition.toObject().toMap())
    }
}