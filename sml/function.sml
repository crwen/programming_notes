
(* int * int -> int *)
fun pow (x: int, y: int) = 
  if y < 0
  then 0
  else if y = 0
  then 1
  else x * pow(x, y - 1)

fun cubic(x: int) = 
  pow(x, 3)

val a = pow(2, pow(2, 2))
val b = cubic(2)

val one = pow(2, 0)
