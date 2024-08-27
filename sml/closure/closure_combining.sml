(* Combining functions *)

(* val compose = fn : ('a * 'b -> 'c) * 'a -> 'b -> 'c *)
fun compose (f, g) = fn x => f (g, x)

(* int -> real *)
fun sqrt_of_abs x = Math.sqrt (Real.fromInt (abs x))

(* unless function wrapping *)
(* int -> real *)
fun sqrt_of_abs x = (Math.sqrt o Real.fromInt o abs) x

(* int -> real *)
val sqrt_of_abs = Math.sqrt o Real.fromInt o abs

infix !>

fun x !> f = f x

fun sqrt_of_abs x = x !> abs !> Real.fromInt !> Math.sqrt

(* ('a -> 'b option) * ('a -> 'b) -> 'a -> 'b *)
fun backup1 (f, g) = fn x => case f x of 
                                  NONE => g x
                                | SOME y => y
                                
(* ('a -> 'b) * ('a -> 'b) -> 'a -> 'b *)
fun backup2 (f, g) = fn x => f x handle _ => g x 

val res1 = backup1(fn x => SOME(x + 1), fn x => x + 10) 1 (* 2 *)
val res2 = backup2(fn x => SOME(x + 1), fn x => SOME(x + 10)) 1 (* SOME(2) *)

val res1_2 = backup1(fn x => NONE, fn x => x + 10) 1 (* 11 *)
val res2_2 = backup2(fn x => NONE, fn x => SOME(x + 10)) 1 (* NONE *)
