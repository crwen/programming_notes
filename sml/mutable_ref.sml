(* use ref e to create a reference with initial contente *)
(* int ref *)
val x = ref 42
val y = ref 42
val z = x
(* use := to update contents *)
val _ = x := 43
(* use ! to retrieve contents *)
val w = (!y) + (!z) (* 85 *)
(* x + 1 does not type-check *)
