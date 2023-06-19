package org.dashj.dpp

object ProtocolVersion {

    init {
        System.loadLibrary("dashj_dpp_bindings")
    }
    val latestProtocolVersion: Int
        get() = getLatestProtocolVersionBinding()

    external fun getLatestProtocolVersionBinding(): Int
}