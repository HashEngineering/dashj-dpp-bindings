package org.dashj.dpp

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test
import uniffi.kotlin_dpp.latestProtocolVersion

class ProtocolVersionTest {

    fun startUp() {
        System.loadLibrary("dpp")
        println("jna:" + System.getProperty("jna.library.path"))
        println("java:" + System.getProperty("java.library.path"))
    }
    @Test
    fun latestProtocolVersionTest() {
        startUp()
        println("My latest verison" + latestProtocolVersion())
        assertEquals(1, latestProtocolVersion().toInt())
    }
}
