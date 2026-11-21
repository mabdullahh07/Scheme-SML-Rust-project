fun toBool [] = []
  | toBool (0::xs) = false :: toBool xs
  | toBool (_::xs) = true :: toBool xs;

fun toInt [] = []
  | toInt (false::xs) = 0 :: toInt xs
  | toInt (true::xs) = 1 :: toInt xs;

(* (bit a, bit b, carry in) -> (sum bit, carry out) *)
fun fullAdder (false, false, false) = (false, false)
  | fullAdder (true,  false, false) = (true,  false)
  | fullAdder (false, true,  false) = (true,  false)
  | fullAdder (false, false, true)  = (true,  false)
  | fullAdder (true,  true,  false) = (false, true)
  | fullAdder (true,  false, true)  = (false, true)
  | fullAdder (false, true,  true)  = (false, true)
  | fullAdder (true,  true,  true)  = (true,  true);

fun finishBinaryAdd ([], false) = []
  | finishBinaryAdd ([], true) = [true]
  | finishBinaryAdd (x::xs, c) =
      let val (s, c2) = fullAdder (x, false, c)
      in s :: finishBinaryAdd (xs, c2) end;

fun doBinaryAddition ([], ys, c) = finishBinaryAdd (ys, c)
  | doBinaryAddition (xs, [], c) = finishBinaryAdd (xs, c)
  | doBinaryAddition (x::xs, y::ys, c) =
      let val (s, c2) = fullAdder (x, y, c)
      in s :: doBinaryAddition (xs, ys, c2) end;

fun binaryAddition (a, b) =
  toInt (rev (doBinaryAddition (rev (toBool a), rev (toBool b), false)));

fun invert [] = []
  | invert (x::xs) = (not x) :: invert xs;

fun padTo ([], ys) = ys
  | padTo (_::xs, []) = false :: padTo (xs, [])
  | padTo (_::xs, y::ys) = y :: padTo (xs, ys);

fun takeLike (_::xs, y::ys) = y :: takeLike (xs, ys)
  | takeLike _ = [];

fun stripZeros (0::(rest as _::_)) = stripZeros rest
  | stripZeros xs = xs;

(* a - b, assuming a >= b *)
fun binarySubtraction (a, b) =
  let
    val ra = rev (toBool a)
    val rb = rev (toBool b)
    val twos = doBinaryAddition (invert (padTo (ra, rb)), [true], false)
    val sum = takeLike (ra, doBinaryAddition (ra, twos, false))
  in
    stripZeros (toInt (rev sum))
  end;

binaryAddition ([1,1,1,1,0], [1,0,1,1]);
binaryAddition ([1,0,0,0,1,1,1], [1,0,1,1,0]);
binarySubtraction ([1,0,1,0,0,1], [1,0,1,1]);
binarySubtraction ([1,0,1], [1,0,1]);
