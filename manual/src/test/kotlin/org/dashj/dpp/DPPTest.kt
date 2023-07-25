package org.dashj.dpp

import org.bitcoinj.core.ECKey
import org.bitcoinj.core.Sha256Hash
import org.bitcoinj.params.AbsintheDevNetParams
import org.bitcoinj.params.TestNet3Params
import org.dashj.dpp.DPP.validateIdentityCreateTransition
import org.dashj.platform.dpp.identifier.Identifier
import org.dashj.platform.dpp.identity.Identity
import org.dashj.platform.dpp.identity.IdentityCreateTransition
import org.dashj.platform.dpp.toHex
import org.dashj.platform.dpp.util.Converters
import org.junit.jupiter.api.Assertions.*
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

    @Test
    fun createIdentityCreateTransitionTest() {
        val raw_transition = mapOf<String, Any?>(
            "protocolVersion" to ProtocolVersion.latestProtocolVersion,
            "assetLockProof" to mapOf(
                "type" to 0,
                "instantLock" to Converters.fromHex("01011DBBDA5861B12D7523F20AA5E0D42F52DE3DCD2D5C2FE919BA67B59F050D206E00000000C00964FF90E9C29E60682E0FE18598DDD462F3A8EB92615CF422888C95F1DCAD2E02C76C7E57779AFD9942F983AFBFE2F1E0DD07CAB57A75A776B062DFD0C80D000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".lowercase()),
                "transaction" to Converters.fromHex("03000000018C6A321936A4DA0649AA8EEB2DA3280C10088C057A22BAF2BF987951924E8CC8000000006A473044022073ED0EE14ACDFD31AE0E3627109676B065B97651342187F7501E42525404AD2602205B7F12C535D0A7A5693390D3456C93752B15D39467A63C687B516C5AAFB93A7B0121027D916607B92BAC0A1527AA126ADACBDC24E749989ADC0CD2C62DFC0A0D4035E3FFFFFFFF0140420F0000000000166A1425EC403D0CEF684745E7741E90183CC0F128277500000000".lowercase()),
                "outputIndex" to 0
            ),
            "publicKeys" to listOf(
                mapOf(
                    "id" to 0,
                    "type" to 0,
                    "purpose" to 0,
                    "securityLevel" to 0,
                    "readOnly" to false,
                    "data" to Converters.fromHex("036E6F84B4B4525605F819FBA38388E7E088606663AFA789730413664C71D0ECC2".lowercase()),
                    "signature" to Converters.fromHex("20F9353522CE1F724B70C87F5F9454F7453DA703FCC5F4BC4F5C36E3D9DE16EB86501D7EA8B42DFBAAA384DEABF35E3165668B7A7DDE1C214E81B4504B97F3CD9B".lowercase())
                ),
                mapOf(
                    "id" to 1,
                    "type" to 0,
                    "purpose" to 0,
                    "securityLevel" to 2,
                    "readOnly" to false,
                    "data" to Converters.fromHex("025CEFCE88EE5484256814FAD4619DDF6B845E6CD2429AF0115115F1BB87169A26".lowercase()),
                    "signature" to Converters.fromHex("1F1B04469F1507CBAE51B990B6608F215E44017C43B858B0E21F874C379F9CF3A8153A02408641624DDED0568C0854817A368C2AE2CD309DF6AEFB50812B3FD051".lowercase())
                ),
            ),
            "signature" to Converters.fromHex("2058CB28B00B749EF0736363E110DD2AC2F26917DDFFD086532DD3FA3885A0993F54ABEB9E17960A2704FEB96632595F67D9CE3692A443B62E8969740C87496285".lowercase()),
            "type" to 2
        )
        val bytes = DPP.serializeIdentityCreateTransition(raw_transition)
        assertTrue(bytes.isNotEmpty())
        println(bytes.toHex())
        val expected = "0202000000000021036e6f84b4b4525605f819fba38388e7e088606663afa789730413664c71d0ecc24120f9353522ce1f724b70c87f5f9454f7453da703fcc5f4bc4f5c36e3d9de16eb86501d7ea8b42dfbaaa384deabf35e3165668b7a7dde1c214e81b4504b97f3cd9b010000020021025cefce88ee5484256814fad4619ddf6b845e6cd2429af0115115f1bb87169a26411f1b04469f1507cbae51b990b6608f215e44017c43b858b0e21f874c379f9cf3a8153a02408641624dded0568c0854817a368c2ae2cd309df6aefb50812b3fd0510000c601011dbbda5861b12d7523f20aa5e0d42f52de3dcd2d5c2fe919ba67b59f050d206e00000000c00964ff90e9c29e60682e0fe18598ddd462f3a8eb92615cf422888c95f1dcad2e02c76c7e57779afd9942f983afbfe2f1e0dd07cab57a75a776b062dfd0c80d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000bc03000000018c6a321936a4da0649aa8eeb2da3280c10088c057a22baf2bf987951924e8cc8000000006a473044022073ed0ee14acdfd31ae0e3627109676b065b97651342187f7501e42525404ad2602205b7f12c535d0a7a5693390d3456c93752b15d39467a63c687b516c5aafb93a7b0121027d916607b92bac0a1527aa126adacbdc24e749989adc0cd2c62dfc0a0d4035e3ffffffff0140420f0000000000166a1425ec403d0cef684745e7741e90183cc0f1282775000000000001412058cb28b00b749ef0736363e110dd2ac2f26917ddffd086532dd3fa3885a0993f54abeb9e17960a2704feb96632595f67d9ce3692a443b62e8969740c874962859ef3dc44553db4b808be2cab7052f20a96d5d184e77625d9c27257ecd950f9b5";
        assertEquals(expected, bytes.toHex())

        val fromKotlin = IdentityCreateTransition(TestNet3Params.get(), raw_transition)
        val bytes2 = DPP.serialize(fromKotlin);
        assertEquals(expected, bytes2.toHex())

        assertTrue(validateIdentityCreateTransition(raw_transition))
    }

    @Test
    fun privateKeyTest() {
        val privateKey1 = ECKey();
        val privateKey2 = ECKey();

        val raw_transition = mapOf<String, Any?>(
            "protocolVersion" to ProtocolVersion.latestProtocolVersion,
            "assetLockProof" to mapOf(
                "type" to 0,
                "instantLock" to Converters.fromHex("01011DBBDA5861B12D7523F20AA5E0D42F52DE3DCD2D5C2FE919BA67B59F050D206E00000000C00964FF90E9C29E60682E0FE18598DDD462F3A8EB92615CF422888C95F1DCAD2E02C76C7E57779AFD9942F983AFBFE2F1E0DD07CAB57A75A776B062DFD0C80D000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".lowercase()),
                "transaction" to Converters.fromHex("03000000018C6A321936A4DA0649AA8EEB2DA3280C10088C057A22BAF2BF987951924E8CC8000000006A473044022073ED0EE14ACDFD31AE0E3627109676B065B97651342187F7501E42525404AD2602205B7F12C535D0A7A5693390D3456C93752B15D39467A63C687B516C5AAFB93A7B0121027D916607B92BAC0A1527AA126ADACBDC24E749989ADC0CD2C62DFC0A0D4035E3FFFFFFFF0140420F0000000000166A1425EC403D0CEF684745E7741E90183CC0F128277500000000".lowercase()),
                "outputIndex" to 0
            ),
            "publicKeys" to listOf(
                mapOf(
                    "id" to 0,
                    "type" to 0,
                    "purpose" to 0,
                    "securityLevel" to 0,
                    "readOnly" to false,
                    "data" to privateKey1.pubKey,
                    "signature" to ByteArray(0)
                ),
                mapOf(
                    "id" to 1,
                    "type" to 0,
                    "purpose" to 0,
                    "securityLevel" to 2,
                    "readOnly" to false,
                    "data" to privateKey2.pubKey,
                    "signature" to ByteArray(0)
                ),
            ),
            "type" to 2,
            "signature" to ByteArray(0)
        )

        val ict = IdentityCreateTransition(AbsintheDevNetParams.get(), raw_transition)
        val bytes1 = DPP.signableBytesIdentityCreateTransition(raw_transition)
        assertArrayEquals(bytes1, DPP.signableBytesIdentityCreateTransition(ict.toObject()))

        for (i in 0 until 2) {
            //val bytes1 = DPP.signableBytesIdentityCreateTransition(raw_transition)

            //assertArrayEquals(bytes1, DPP.signableBytesIdentityCreateTransition(ict.toObject()))

            val bytes = DPP.signableBytesIdentityCreateTransition(ict.toObject())
            val sig = privateKey1.signHash(Sha256Hash.twiceOf(bytes1))
            ict.publicKeys[i].signature = sig

            println(bytes.toHex())
            println(sig.toHex())
        }
        val bytesTransition = DPP.signableBytesIdentityCreateTransition(ict.toObject())
        val sigTransition = privateKey1.signHash(Sha256Hash.twiceOf(bytesTransition))
        ict.signature = sigTransition

        assertTrue(validateIdentityCreateTransition(ict.toObject()))



        val bincode = Converters.fromHex("0120413f39f3cc096a47bb025eddbe7f8d7289d1f3323cd75a65e50f73052c75b6d9020000000000002103292da1684067c3b78ecef3d3e2c132434e7aa9930c0a520bfc4e2ad73a3edd6b0001010002000021020a066d33abcd94c00f01fc84ae3376f7103c79d8dbacba2b4b20e0eb733ec0150000000000")
        println("cbor from bincode: " + DPP.getIdentityCborFromBincode(bincode).toHex())
    }
}