
fun fib x =
  if x <= 1 then x
  else (prev x) + (last x)
and prev x =
  fib (x - 1)
and last x =
  fib (x - 2)

datatype t1 = Foo of int | Bar of t2
and t2 = Baz of string | Quux of t1

fun no_zeros_or_empty_strings_t1 x =
    case x of
	Foo i => i <> 0
      | Bar y => no_zeros_or_empty_strings_t2 y
and no_zeros_or_empty_strings_t2 x =
    case x of
	Baz s => size s > 0
      | Quux y => no_zeros_or_empty_strings_t1 y
