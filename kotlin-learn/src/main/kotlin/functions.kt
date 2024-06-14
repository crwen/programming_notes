
fun main() {

    println(add(1, 2))
    println(add_omit_return_type(1, 2))
    namedArg(prefix = "Log", message = "Hello")
    defaultParameterValues("Hello")

    lambda_expressions()
}

fun add(x: Int, y: Int): Int {
    return x + y
}

fun add_omit_return_type(x: Int, y: Int) = x + y

fun namedArg(message: String, prefix: String) {
    println("[$prefix] $message")
}

fun defaultParameterValues(message: String, prefix: String = "Info") {
    println("[$prefix] $message")
}

fun lambda_expressions() {

    println({ x: Int, y: Int -> add(x, y) }(1, 2))

    // Assign to variable
    val upperCaseString = { string: String -> string.uppercase() }
    println(upperCaseString("hello"))

    // Pass to another function
    // If a lambda expression is the only function parameter,
    // you can drop the function parentheses ()
    val numbers = listOf(1, -2, 3, -4, 5, -6)
    // public inline fun <T> Iterable<T>.filter(
    //     predicate: (T) -> Boolean
    // ): List<T>

    val positives = numbers.filter ({ x -> x > 0 })
    val negatives = numbers.filter { x -> x < 0 }
    println(positives)
    println(negatives)

    val doubled = numbers.map { x -> x * 2 }
    val tripled = numbers.map { x -> x * 3 }
    println(doubled)
    println(tripled)

    val upperCaseString2: (String) -> String = { string -> string.uppercase() }
    println(upperCaseString2("hello"))

    // return from a function
    val timesInMinutes = listOf(2, 10, 15, 1)
    val min2sec = toSeconds("minute")
    println("60 minutes to seconds is ${min2sec(60)}s")
    val totalTimeInSeconds = timesInMinutes.map(min2sec).sum()
    println("Total time is $totalTimeInSeconds secs")

    // invoke separately
    // invoke on their own by add parentheses
    println({ string: String -> string.uppercase() }("hello"))

    // Trailing lambdas
    // If a lambda expression is passed as the last parameter of a function,
    // then the expression can be written outside the function parentheses ()
    println(listOf(1, 2, 3).fold(0, { x, item -> x + item })) // 6
    println(listOf(1, 2, 3).fold(0) { x, item -> x + item })  // 6

}

fun toSeconds(time: String): (Int) -> Int = when (time) {
    "hour" -> { value -> value * 60 * 60 }
    "minute" -> { value -> value * 60 }
    "second" -> { value -> value }
    else -> { value -> value }
}

