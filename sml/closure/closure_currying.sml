(* Take one argument and return a functino that takes another argument *)

(* old way*)
fun sorted3_tupled (x, y, z) = z >= y andalso y >= x

(* new way: currying *)
fun sorted3_1 x = fn y => fn z => z >= y andalso y >= x
val sorted3_2 = fn x => fn y => fn z => z >= y andalso y >= x
fun sorted3_suger x y z = z >= y andalso y >= x

val t = sorted3_tupled(7, 9, 11)
val t1 = sorted3_1 7 9 11
val t2 = sorted3_2 7 9 11
val t3 = sorted3_2 7 9 11


fun fold f acc xs = 
  case xs of
       [] => acc
     | x::xs => fold f (f(acc, x)) xs

fun sum xs = fold (fn (x, y) => x + y) 0 xs


(* partial applicatino*)



fun range (i, j) = if i > j then [] else i :: range (i + 1, j)

(* val countup = range 1 *) (* won't work *)

(* curry wrapup *)
(* fun curry f = fn x => fn y => f (x, y) *)
(* val curry = fn f => fn x => fn y => f (x, y) *)
fun curry f x y = f (x, y)
fun uncurry f (x, y) = f x y

val countup = curry range 1

val xs = countup 7

(*
 * Does currying slow? It depend on the implementation of language
 * Both currying and tupling are constant-time operations, sot it doesn't matter
 * in most of case
 *
 * But for the small part where efficiency matters: 
 *  - SML/NJ tupling more faster
 *  - OCaml, F#, Haskell do better with currying
 * *)
