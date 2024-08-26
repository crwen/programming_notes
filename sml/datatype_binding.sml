
datatype mytype = TwoInts of int * int
                | Str of string
                | Pizza

val a = Str "hi"
(* a = Str "hi" : mytype *)

val b = Str
(* val b = fn : string -> mytype *)

val c = Pizza
(* val c = Pizza : mytype *)

val d = TwoInts(1+2, 3+4)
(* val d = TwoInts (3,7) : mytype *)

val e = a
(* val e = Str "hi" : mytype *)

(* val f = fn : mytype -> int *)
fun f (x: mytype) = 
  case x of
       Pizza => 3
     | Str s => 8
     | TwoInts(i1, i2) => i1 + i2


datatype exp = Constant of int 
             | Negate of exp 
             | Add of exp * exp 
             | Multiply of exp * exp

fun eval e = 
  case e of
       Constant i => i
     | Negate e2 => ~ (eval e2)
     | Add (e1, e2) => (eval e1) + (eval e2)
     | Multiply(e1, e2) => (eval e1) * (eval e2)

fun number_of_adds e = 
  case e of
       Constant i => 0
     | Negate e2 =>  number_of_adds e2
     | Add (e1, e2) => 1 + number_of_adds e1 + number_of_adds e2
     | Multiply(e1, e2) =>  number_of_adds e1 + number_of_adds e2

fun max_constant e = 
  case e of 
       Constant i => i 
     | Negate e2 => max_constant e2
     | Add (e1, e2) => Int.max(max_constant e1, max_constant e2)
     | Multiply (e1, e2) => Int.max(max_constant e1, max_constant e2)

val example_exp = Add(Multiply (Constant 2, Constant 3), Negate (Constant 4))
val number_of_example_adds = number_of_adds(example_exp)
val max_const = max_constant(example_exp)
val result = eval(example_exp)
