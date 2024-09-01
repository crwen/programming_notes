About the course

- Many essential concepts relevant in any programming language
- Use ML, Racket, and Ruby
- Big focus on functional programming

### Bindings

An ML program is a sequence of _bindings_. Each binding gets _type-checked_ and then (assuming it type-checks) _evaluated_. What type (if any) a binding has depends on a _static environment_(context), which is roughly the types of the preceding bindings in the file. How a binding is evaluated depends on a _dynamic environment_, which is roughly the values of the preceding bindings in the file

```sml
(* variable binding *)
val x = e;

(* function binding *)
fun pow (x: int, y: int) = (* correct only for y >= 0 *)
  if y = 0
  then 1
  else x * pow(x, y - 1)

(* datatype binding *)
datatype mytype = TwoInts of int * int | Pizza

(* type synonym *)
type foo = int
```

- Syntax: how to write it
- Semantic: how it type-checks and evaluates
  - Type-checking(before program runs): Type-check expression and extend **static environment**
  - Evaluation(as program runs): Evaluate expression and extend **dynamic environment**

### Polymorphic Datatypes

Polymorphic Datatypes can be used for ayn type, it is very useful for building "generic" data structures.

```sml
datatype 'a option = NONE | SOME of 'a
```

if a `t` is a type, then `t option` is type

ML type system is unsound, meaning that it would accept programs that when run could have values of the wrong types. The problem results from a combination of polymorphic types and mutable references, and the fix is a special restriction to the type system called the **the value restriction**

```sml
val r = ref NONE        (* 'a option ref *)
val _ := SOME "hi"      (* instantiate 'a with string *)
val i := 1 + valOf(!r)  (* instantiate 'a with int *)

```

### Pattern-Matching

- Each-Of Type
- One-Of Type

Each branch has the form `p => e` where p is a _pattern_ and _e_ is an exppression. Each pattern use a different constructor and pattern-matching picks the branch with the "right one"

```sml
case a of
    SOME x => 1
  | NONE => 0
```

Acutally, val bindings is a kind of Pattern-Matching

```sml
fun full_name (r: {first: string, middle: string, last: string}) =
  let val {first = x, middle = y, last = z} = r
  in
    x ^ " " ^ y ^ " " ^ z
  end

fun full_name2 {first=x, middle=y, last=z} =
    x ^ " " ^ y ^ " " ^ z
```

multiple cases in function binding

```sml
fun eval (Constant i) = i
  | eval (Negate e2) = ~(eval e2)
  | eval (Add e1, e2) = (eval e1) + (eval e2)

```

### Exception

### Function

- _first-class functions_: functions can be computed, passed, stored, etc.
- _functions closures_: refer to functions that use variables defined outside of them
- _higher-order function_: refers to a function that takes or returns other function

The term of _functional programming_ often used imprecisely to refer serveral distinct concepts:

- Not using mutable data in most or all cases
- Using functions as values
- Encourages recuursion
- Closer to traditional mathematical definitions of function
- ...

**Anonymous functions**

```sml
fun triple_n_times (n, x) = n_times((fn y => 3 * y), n, x)

fun increment x = x + 1
(* use val bindings instead of a fun binding *)
val increment = fn x => x + 1
```

#### Lexical Scope

We can use any bindings that are in scope, and the body of a function is evaluated in the environment where the function is **defined**, not the environment where the function is **called**

```sml
val x = 1
fun f y = x + y
val x = 2 (* irrelevant *)
val y = 3 (* irrelevant *)
val z = f (x + y)  (* => f (2 + 3),  get 6 *)
```

The argument was evaluated in the current environment, but the function body was evaluated in the "old" environment.

This semantics is called _lexical scope_. The alternate, inferior semantics where you use the current environment is called _dynamic scope_

```sml
fun f g =
  let
    val x = 3 (* irrelevant *)
  in
    g 2
  end

val x = 4

fun h y = x + y (* add 4 to its argument *)

val z = f h (* gets 6 *)
```

#### Closures

function has two parts:

- code for the function
- environment that was current when we created the function

the code can have _free variables_(not bound inside the code), but the closure carries with it an environment that provides all these bindings. So the closure overall is "closed"(has everything it needs to produce a function result given a function argument)

```sml
fun filter (f,xs) =
  case xs of
      [] => []
      | x::xs’ => if f x then x::(filter(f,xs’)) else filter(f,xs’)

fun allGreaterThanSeven xs = filter (fn x => x > 7, xs)

fun allGreaterThan (xs,n) = filter (fn x => x > n, xs)
```

**Function composition**: functions that are just combinations of other functions

```sml

fun compose (f, g) = fn x => f (g, x)
fun sqrt_of_abs x = Math.sqrt (Real.fromInt (abs x))
val sqrt_of_abs = Math.sqrt o Real.fromInt o abs
```

**Currying**: a function take the first conceptual argument and return another function that takes the second conceptual argument and so on

```sml
fun sorted3 (x, y, z) => z >= y andalso y >= x
(* ==> currying *)
val sorted3 = fn x => fn y => fn z => z >= y andalso y >= x
(* ==> syntax suger*)
fun sorted3 x y z => z >= y andalso y >= x
```

**Partial Application**

```sml
(* Get a closure that has x in its environment *)
val f = sorted3 4
(* Get a closure that has x, y in its environment *)
val g = f 5

(* Partial application combines currying*)
fun range (i, j) = if i > j then [] else i :: range (i + 1, j)

fun curry f x y = f (x, y)

val countup = curry range 1
val xs = countup 7
```

**Callback**: a function that gets called when the event occurs.

### Type inference

Java, C, and ML are all examples of statically typed languages, meaning every binding has a type that is determined “at compile-time,” i.e., before any part of the program is run. The type-checker is a compile-time procedure that either accepts or rejects a program

By contrast, Racket, Ruby, and Python are dynamically typed languages, meaning the type of a binding is not determined ahead of time and computations

ML is a statically typed language, so type-checker must infer what the type annotations "would have been". In principle, type inference and type checking could be separate steps

ML type inference overview:

1. Determines the types of bindings in order, using the types of earlier bindings to infer the types of later ones
2. For each `val` or `fun` binding, it analyzes the binding to determine necessary facts about its type. e.g. x + 1 => x must have type `int`
3. Afterward, use **type variables**(e.g. 'a) for any unconstrained types in function argument or results
4. (Enforce the value restriction, only variables and values can have polymorphic types)
