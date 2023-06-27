package org.dashj.dpp

object DPP {
    init {
        System.loadLibrary("dashj_dpp_bindings")
    }
    external fun createIdentityCbor(): ByteArray
    external fun convertMap(map: Map<String, Any>)
    external fun createIdentityFromRawObject(map: Map<String, Any>): ByteArray
}