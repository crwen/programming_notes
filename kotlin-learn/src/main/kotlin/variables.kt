import kotlin.reflect.typeOf

const val TOP_LEVEL = 100

fun main() {
    readOnlyVariables()
    mutableVariables()
    stringTemplates()
    println("top level variables is $TOP_LEVEL")
}

fun readOnlyVariables() {
    val customers = 10

    // customers = 8 // compile error
    println(customers)
}

fun mutableVariables() {
    var customers = 10
     customers = 8 // ok

    println(customers) // 8
}

fun stringTemplates() {
    // template expressions
    val customer = 10
    println("There are $customer customers")

    println("There are ${customer + 1} customers")

    val letters = listOf("a", "b", "c", "d", "e")
    println("Letters: $letters")
    println("""
        insert ${'$'} in a multiline string.
        backslash escaping(\$) is not supported.
    """)

}