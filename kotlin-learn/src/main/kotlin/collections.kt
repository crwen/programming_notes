
fun main() {
    listCollections()
    setCollections()
    mapCollections()
}

fun listCollections() {
    // Lists store items in the order that they are added
    // allow for duplicate items

    // read-only list -> List -> listOf()
    val readOnlyShapes = listOf("triangle", "square", "circle") // type infer
    val readOnlyShapes2 = listOf<String>("triangle", "square", "circle") // type infer

    // mutable list -> MutableList -> mutableListOf()
    val shapes = mutableListOf("triangle", "square", "circle")
    println(shapes)

    shapes.add("star")

    println("shapes numbers: ${shapes.count()} $shapes")

    // obtain read-only views of mutable lists
    val shapesLocked: List<String> = shapes
    // shapesLocked.add("...") // compile error

    // use indexed access operator `[]`
    println("The first item in the list is: ${readOnlyShapes[0]}")
    // use `first()` and `last()` to get the first and last item
    println("The first item in the list is: ${readOnlyShapes.first()}, last is ${readOnlyShapes.last()}")

    // check that an item is in a list using `in`
    println("circle in shapes ${"circle" in shapes}")

    // use `add` or ` remove` to add or remove from a mutable list
    shapes.add("pentagon")
    shapes.remove("star")
    println("shapes numbers: ${shapes.count()} $shapes")

}

fun setCollections() {

    // To create a read-only set (Set), use the setOf() function.
    val readOnlyFruit = setOf("apple", "banana", "cherry", "cherry")
    // To create a mutable set (MutableSet), use the mutableSetOf() function.
    val fruit: MutableSet<String> = mutableSetOf("apple", "banana", "cherry", "cherry")

    // obtain read-only views of mutable set
    val fruitLocked: Set<String> = fruit

    println("This set has ${readOnlyFruit.count()} items")
    println("banana in readOnlyFruit ${"banana" in readOnlyFruit}")

    fruit.add("dragonfruit")    // Add "dragonfruit" to the set
    println(fruit)              // [apple, banana, cherry, dragonfruit]

    fruit.remove("dragonfruit") // Remove "dragonfruit" from the set
    println(fruit)              // [apple, banana, cherry]
}

fun mapCollections() {

    // To create a read-only map (Map), use the mapOf() function.
    val readOnlyJuiceMenu = mapOf("apple" to 100, "kiwi" to 190, "orange" to 100)
    println(readOnlyJuiceMenu)
    // To create a mutable map (MutableMap), use the mutableMapOf() function.
    val juiceMenu: MutableMap<String, Int> = mutableMapOf("apple" to 100, "kiwi" to 190, "orange" to 100)
    println(juiceMenu)
    // obtain read-only views of mutable map
    val juiceMenuLocked: Map<String, Int> = juiceMenu

    println("This map has ${readOnlyJuiceMenu.count()} key-value pairs")

    juiceMenu.put("coconut", 150) // Add key "coconut" with value 150 to the map
    println(juiceMenu)

    juiceMenu.remove("orange")    // Remove key "orange" from the map
    println(juiceMenu)

    println(readOnlyJuiceMenu.keys)
    println(readOnlyJuiceMenu.values)

    println("apple in juiceMenuLocked ${"apple" in juiceMenuLocked}")

}