package org.dashj.dpp

import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Converters
import org.junit.jupiter.api.Test

class DPPTest {
    @Test
    fun createIdentityTest() {
        println(DPP.createIdentityCbor().toHex())

        DPP.convertMap(mapOf("integer" to 11, "float" to 11.0, "map" to mapOf("double" to 1.000)))

        val rawIdentity = mapOf(
            "protocolVersion" to 1,
            "id" to Identifier.from(
                byteArrayOf(
                    198.toByte(), 23, 40, 120, 58, 93, 0, 165.toByte(),
                    27, 49, 4, 117, 107, 204.toByte(), 67, 46, 164.toByte(), 216.toByte(), 230.toByte(), 135.toByte(),
                    201.toByte(), 92, 31, 155.toByte(), 62, 131.toByte(), 211.toByte(), 177.toByte(), 139.toByte(),
                    175.toByte(), 163.toByte(), 237.toByte()
                )
            ),
            "publicKeys" to listOf(
                mapOf(
                    "id" to 0,
                    "type" to 0,
                    "purpose" to 0,
                    "securityLevel" to 0,
                    "data" to Converters.fromBase64("AuryIuMtRrl/VviQuyLD1l4nmxi9ogPzC9LT7tdpo0di"),
                    "readOnly" to false
                ),
                mapOf(
                    "id" to 1,
                    "type" to 0,
                    "purpose" to 1,
                    "securityLevel" to 3,
                    "data" to Converters.fromBase64("A8AK95PYMVX5VQKzOhcVQRCUbc9pyg3RiL7jttEMDU+L"),
                    "readOnly" to false
                )
            ),
            "balance" to 10,
            "revision" to 0
        );

        val kotlinIdentityCbor = Identity(rawIdentity).toBuffer()

        val rsDPPcbor = DPP.createIdentityFromRawObject(rawIdentity).toHex();
        val kotlinDPPcbor = kotlinIdentityCbor.toHex()

        println("rs-dpp:     $rsDPPcbor")
        println("kotlin-dpp: $kotlinDPPcbor")
        println("matching: ${rsDPPcbor == kotlinDPPcbor}")
    }
}