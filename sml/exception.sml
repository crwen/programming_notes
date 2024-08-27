(* raise an exception *)
fun hd xs =
    case xs of
        []   => raise List.Empty
      | x::_ => x

exception MyException

exception MyOtherException of int * int

fun mydiv (x,y) =
    if y=0
    then raise MyException
    else x div y 

fun maxlist (xs,ex) = (* int list * exn -> int *)
    case xs of
        [] => raise ex
      | x::[] => x
      | x::xs' => Int.max(x,maxlist(xs',ex))

val w = maxlist ([3,4,5],MyException) (* 5 *)

(* handle exception *)
val x = maxlist ([3,4,5],MyException) (* 5 *)
	handle MyException => 42

(*val y = maxlist ([],MyException)*)

val z = maxlist ([],MyException) (* 42 *)
	handle MyException => 42
