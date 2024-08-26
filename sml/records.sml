
val user = {name = "crwen", age = 18, id = 1}
val name = #name user
val vip_user = {id= #id user, user = user, discount = 0.2}

val a_record = {3="hi", 1=true, 2=2+3}
(* a_record --> (true,5,"hi") : bool * int * string *)
val bool_value = #1 a_record
val hi = #3 a_record
