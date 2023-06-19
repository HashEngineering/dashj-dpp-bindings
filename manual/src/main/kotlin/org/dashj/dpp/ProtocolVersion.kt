package org.dashj.dpp

object ProtocolVersion {
    val latestProtocolVersion: Int
        get() = getLatestProtocolVersionBinding()

    external fun getLatestProtocolVersionBinding(): Int
}