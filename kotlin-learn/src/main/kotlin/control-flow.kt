fun main() {
    ifExpr()
    whenExpr()
    rangeExpr()
    loopExpr()
}

// no `condition ? then : else` in kotlin
fun ifExpr() {
    val check = true
    val a = 1
    val b = 2

    if (check) {
        println(if (a > b) a else b)
    }
}

fun whenExpr() {
    val obj = "Hello"

    // only the first suitable branch is executed.
    when (obj) {
        // Checks whether obj equals to "1"
        "1" -> println("One")
        // Checks whether obj equals to "Hello"
        "Hello" -> println("Greeting")
        // Default statement
        else -> println("Unknown")
    }
}

fun rangeExpr() {
    val num = 10
    val res = when  {
        num < 0 -> "less than 0"
        num in 0..10 -> "between 0 and 10"
        else -> "greater than 10"
    }
    println(res)

    var sum = 0
    for (i in 0..10)
        sum += i
    println(sum)

    for (i in 5 downTo 0)
        sum -= i
    println(sum)
}

fun loopExpr() {
    for (number in 1..5) {
        print(number)
    }
    println()
    val cakes = listOf("carrot", "cheese", "chocolate")
    var cakesEaten = 0
    for (cake in cakes) {
        println("Yummy, it's a $cake cake!")
    }
    while (cakesEaten < 3) {
        println("Eat a cake")
        cakesEaten++
    }
    var cakesBaked = 0
    do {
        println("Bake a cake")
        cakesBaked++
    } while (cakesBaked < cakesEaten)
}