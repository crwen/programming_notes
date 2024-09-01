
structure MyLib = 
struct
  (* bindings *)
  val pi = Math.pi 

  fun fib x = 
    if x <= 1 then x
    else fib (x - 1) + fib (x - 2)

end

val pi = MyLib.pi
val fib_5 = MyLib.fib 5
