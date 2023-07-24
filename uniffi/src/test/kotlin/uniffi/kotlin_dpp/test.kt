package uniffi.kotlin_dpp

import org.dashj.platform.dpp.util.Converters
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

fun ByteArray.toUByteList(): List<UByte> {
    return toList().map { it.toUByte() }
}
class Test {
    @Test
    fun myTest() {
        println("my test")
        val me = IdentifierWrapper(ByteArray(32).toUByteList())
        me.destroy()
        // println(me.toBase58())

        val bytes = Converters.fromHex("0120413f39f3cc096a47bb025eddbe7f8d7289d1f3323cd75a65e50f73052c75b6d9020000000000002103292da1684067c3b78ecef3d3e2c132434e7aa9930c0a520bfc4e2ad73a3edd6b0001010002000021020a066d33abcd94c00f01fc84ae3376f7103c79d8dbacba2b4b20e0eb733ec0150000000000")
        val you = IdentityWrapper.deserialize(bytes.toUByteList())
        assertEquals(1u, you.getProtocolVersion())
        val ipk = you.getPublicKeyById(0u)
        // val ipk;
        // val you2 = IdentityWrapper(bytes.toUByteList())
        // println(you.getId().toString())
        println(you.getId().toBase58())
        you.destroy()
    }
}
