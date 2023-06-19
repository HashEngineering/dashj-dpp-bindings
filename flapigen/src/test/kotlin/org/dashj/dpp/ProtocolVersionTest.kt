package org.dashj.dpp

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ProtocolVersionTest {
    init {
        System.loadLibrary("java_dpp")
    }
    @Test
    fun latestProtocolVersionTest() {
        val p = ProtocolVersion()
        assertEquals(1, p._protocol_version)
    }
}