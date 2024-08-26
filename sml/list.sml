
val list1 = [1, 2, 3];

9::list1;

val list2 = 7::5::list1;
(* list2 --> [7, 5, 1, 2, 3] : int list *)

(* null: fn 'a list -> bool *)
val is_empty = null list1;
val head = hd list1;
(* head --> 1 *)

val tail = tl list1;
(* tail --> [2, 3] *)

val elem = hd (tl list1);
(* elem --> 2 *)

val empty_list = tl [8];
(* empty_list: int list *)
(* empty_list --> [] *)

val empty_list2 = [];
(* empty_list2: 'a list *)

fun list_product (xs: int list) = 
  if null xs 
  then 1
  else hd xs * list_product(tl xs)

val x = list_product [];
val y = list_product [5];
val z = list_product [2,4,2];


fun countdown(x: int) = 
  if x = 0
  then []
  else x :: countdown(x - 1)

val sum = list_product( countdown(10) );

(* (int list) * (int list) -> int list *)
fun append (xs: int list, ys: int list) =
  if null xs
  then ys
  else (hd xs) :: append((tl xs), ys)

val list3 = append([1, 2, 3], [4, 5, 6]);
