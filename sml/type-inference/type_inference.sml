(* from the course of Programming languages *)
(*
   sum : T1 -> T2 [must be a function; all functions take one argument]
   xs : T1 [must have type of f's argument]

   x : T3 [pattern match against T3 list]
   xs' : T3 list [pattern match against T3 list]

   T1 = T3 list [else pattern-match on xs doesn't type-check]
   0 : int, so case-expresssion : int, so body : int, so T2=int
   T3 = int [because x : T3 and is argument to addition]
   T2 = int [because result of recursive call is argument to addition]
   sum xs' type-checks because xs' has type T3 list and T1 = T3 list
   case-expression type-checks because both branches have type int

   from T1 = T3 list and T3 = int, we know T1 = int list
   from that and T2 = int, we know f : int list -> int
*)
fun sum xs =
   case xs of
     [] => 0
   | x::xs' => x + (sum xs')

(* first line is not polymorphic so next two lines do not type-check *)
(* the value restriction *)
val r = ref NONE 
(* OK *)
val r2: int option ref  = ref NONE 
(* OK *)
val r3 = ref (SOME 1) 

(*
val _ = r := SOME "hi" 

val i = 1 + valOf (!r) (* does not type-check *)
*)

type 'a foo = 'a ref
val f : 'a -> 'a foo = ref 
val r2 = f NONE (* also need value restriction here *)

(* where the value restriction arises despite no mutation *)
val pairWithOne = List.map (fn x => (x,1))

(* a workaround *)
fun pairWithOne2 xs = List.map (fn x => (x,1)) xs
