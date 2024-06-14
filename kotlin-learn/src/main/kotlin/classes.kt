import kotlin.contracts.contract

fun main() {
    val contact = Contact(1, "mary@gmail.com")
    val contact2 = Contact(1, "mary@gmail.com")
    println(contact.email)
    contact.printId()
    println(contact.equals(contact2)) // false


    val user = User("kotlin", 2)
    val user2 = User("kotlin", 2)
    println(user)
    println(user.equals(user2)) // true
    println(user == user2) // true

    val user3 = user.copy()
    println(user.equals(user3)) // true
    val user4 = user.copy(name = "Java")
    println(user.equals(user4)) // false
    println(user4) // User(name=Java, id=2)




}


// declare a class
class Customer

// declare properties for a class
// Within parentheses () after the class name.
// The content contained within parentheses () is called the class header.
class Contact(
    val id: Int,
    var email: String
) {
    // member functions must be declared within the class body.
    fun printId() {
        println(id)
    }

}

// Within the class body defined by curly braces {}.
class Contact2(val id: Int, var email: String) {
    val category: String = ""

    fun printId() {
        println(id)
    }

}



// declare properties without val or var within parentheses
// these properties are not accessible after an instance has been created.
class Contact3(id: Int, email: String)

// class properties can have default values
class Contact4(val id: Int, var email: String = "example@gmail.com") {
    val category: String = "work"
}

// data classes
// come automatically with additional member functions.
// These member functions allow you to easily
// - `toString()`: print the instance to readable output,
// - `equals()`, `==`: compare instances of a class,
// - `copy()`: copy instances,
// - and more.
// useful for storing data

data class User(val name: String, val id: Int)