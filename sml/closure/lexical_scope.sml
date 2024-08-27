
(* Lexical scope: use environment where functino is defined. now, makes much more sense*)
(* Dynamic scope: use environment where functino is called. Convenient in some situations*)

val x = 1
(* y maps to 2  *)

(* 
 * function has two parts:
 * - The code
 * - The environment that was current when the function was defined
 *)
fun f y = x + y
(* f maps to a function that adds 1 to its argument *)

val x = 2
(* x maps to 2  *)

val y = 3
(* y maps to 2  *)

val z = f (x + y) (* z = 6 *)


(* return a fucntion from a function *)
fun f2 y = 
  let
    val x = y + 1
  in
    fn z => x + y + z (* take z and return 2y + 1 + z *)
  end

val x = 3 (* irrelevant *)

val g = f2 4 (* return a function that adds 9 to its argument *)

val y = 5 (* irrelevant *)

val z = g 6 (* get 15 *)


(* receive a function *)
fun f3 g = 
  let 
    val x = 3 (* irrelevant *)
  in
    g 2
  end

val x = 4

fun h y = x + y (* add 4 to its argument *)

val z = f3 h (* gets 6 *)
