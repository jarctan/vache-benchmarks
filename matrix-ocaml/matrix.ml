open Z

let () =
  let n = Z.of_int 8000 in
  (* Generate the matrix *)
  let model = Array.init (Z.to_int n) (fun j -> Z.of_int j) in
  let a = Array.init (Z.to_int n) (fun _ -> Array.copy model) in
  print_endline (Z.to_string a.(3).(0));

  (* Transpose the matrix *)
  let i = ref Z.zero in
  while !i < n do
  	let j = ref Z.zero in
    while !j < !i do
      let c = a.(Z.to_int !j).(Z.to_int !i) in
      a.(Z.to_int !j).(Z.to_int !i) <- a.(Z.to_int !i).(Z.to_int !j);
      a.(Z.to_int !i).(Z.to_int !j) <- c;
      j := succ !j;
    done;
    i := succ !i;
  done;
  print_endline (Z.to_string a.(3).(0));;

