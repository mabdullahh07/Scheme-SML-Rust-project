fun hitchhiker [] = false
  | hitchhiker (x::xs) = x = 42 orelse hitchhiker xs;

fun everyother (_::y::rest) = y :: everyother rest
  | everyother _ = [];

fun isPrimeHelper (n, d) =
  if d * d > n then true
  else if n mod d = 0 then false
  else isPrimeHelper (n, d + 1);

fun isPrime n = n >= 2 andalso isPrimeHelper (n, 2);

fun findPair (n, p) =
  if isPrime p andalso isPrime (n - p) then p
  else findPair (n, p + 1);

fun goldbach n =
  if n <= 2 orelse n mod 2 <> 0 then
    print ("Error: " ^ Int.toString n ^ " must be an even integer greater than 2\n")
  else
    let val p = findPair (n, 2)
    in print (Int.toString p ^ " + " ^ Int.toString (n - p) ^ " = " ^ Int.toString n ^ "\n")
    end;

hitchhiker [1, 2, 42, 7];
hitchhiker [1, 2, 3];
everyother [3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43];
goldbach 28;
goldbach 7;
