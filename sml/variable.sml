(* This is a comment. This our first program *)

val x = 34;
(* static environment: int *)
(* dynamic environment: x --> 34 *)

val y = 17;
(* static environment: int, y: int *)
(* dynamic environment: x --> 34, y --> 17 *)

val z = (x + y) + (y + 2);
(* static environment: int, y: int, z: int *)
(* dynamic environment: x --> 34, y --> 17, z --> 70 *)

val q = z + 1;
(* static environment: int, y: int, z: int, q: int *)
(* dynamic environment: x --> 34, y --> 17, z --> 70, q --> 71 *)

val abs_of_z = if z < 0 then 0 - z else z; (* bool *) (* int *)
(* static environment: int, y: int, z: int, q: int, abs_of_z: int *)
(* dynamic environment: x --> 34, y --> 17, z --> 70, q --> 71, abs_of_z --> 70*)

val abs_of_z_simpler = abs z;

val a = x div (y - 1)
