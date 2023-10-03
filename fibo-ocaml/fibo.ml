open Printf
open Z

let rec fibo i =
  let one = Z.one in
  if i <= one then
    i
  else
    fibo (i - one) + fibo (i - (Z.of_int 2))

let () =
  let result = fibo (Z.of_int 42) in
  printf "Fibo of 42: %s\n" (Z.to_string result)
