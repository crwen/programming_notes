fun main() {

    typeInference()
    basicTypes()
    declaredWithoutInit()
}

fun typeInference() {
    var customers = 10 // customer has type: `Int`

    customers += 3

    println(customers) // 13
}

fun basicTypes() {
    // integers
    val integerByte: Byte = -128 // -128~127
    val integerShort: Short = -32768 // -32768~32767
    val integerInt: Int = 123_456 // -2^31~2^31-1
    val integerLong: Long = 100_000_000_000_000 // -2^61~2^61-1

    // unsigned integers
    val unsignedIntegerByte: UByte = 255u
    val unsignedIntegerShort: UShort = 65535u
    val unsignedIntegerInt: UInt = 123456u
    val unsignedIntegerLong: ULong = 123456u

    // floating-point numbers
    val float: Float = 255.123f
    val double: Double = 255.123

    // booleans
    var bool: Boolean = true

    // characters
    var char: Char = '➡'

    // strings
    var str: String = "Hello kotlin"
}

fun declaredWithoutInit() {
    val d: Int
    d = 3
}