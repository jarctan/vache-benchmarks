open Z

let partition (array: Z.t array) (start: Z.t) (end': Z.t) =
  let pivot = Array.get array (Z.to_int end') in
  let i = ref (start - Z.one) in
  let j = ref start in
  while !j < end' do
    if Array.get array (Z.to_int !j) <= pivot then begin
      i := succ !i;
      let tmp = Array.get array (Z.to_int !i) in
      Array.set array (Z.to_int !i) (Array.get array (Z.to_int !j));
      Array.set array (Z.to_int !j) tmp;
    end;
    j := succ !j;
  done;
  let tmp = Array.get array (Z.to_int (!i + Z.one)) in
  Array.set array (Z.to_int (!i + Z.one)) (Array.get array (Z.to_int end'));
  Array.set array (Z.to_int end') tmp;
  succ !i

let rec _quick_sort (array: Z.t array) (start: Z.t) (end': Z.t) =
  if start < end' then begin
    let pivot = partition array start end' in
    _quick_sort array start (pivot - Z.one);
    _quick_sort array (succ pivot) end';
  end

let quick_sort (array: Z.t array) =
  let start = zero in
  let end' = Z.of_int (Array.length array) - Z.one in
  _quick_sort array start end'

let () =
  let n = Z.of_int 100000 in
  let array = Array.make (Z.to_int n) Z.zero in
  Random.self_init ();
  for j = 0 to Z.to_int (n - Z.one) do
    Array.set array j (Z.of_int (Random.int 100));
  done;
  quick_sort array;
  Printf.printf "%s\n" (Z.to_string (Array.get array 99999))

