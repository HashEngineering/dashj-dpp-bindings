package org.dashj.dpp

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ProtocolVersionTest {
    @Test
    fun latestProtocolVersionTest() {
        assertEquals(1, ProtocolVersion.latestProtocolVersion)
    }
}