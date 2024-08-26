
fun swap(pr: int * int) = 
  (#2 pr, #1 pr)

fun sum_two_pairs(pr1: int * int, pr2: int * int) = 
  ((#1 pr1) + (#1 pr2), (#2 pr1) + (#2 pr2))


val pair2 = swap(1, 2);
val pair = (11, 22);

val sum_of_pairs = sum_two_pairs(pair, pair2);


val x1 = (7, (true, 9)) (* int * (bool*int) *)
val x2 = #1 (#2 x1)     (* bool *)
val x3 = (#2 x1)        (* bool*int *)

(* (int*int) * ((int*int) * (int*int))  *)
val x4 = ((3, 5), ((4, 8), (0, 0)))

