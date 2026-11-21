fn to_bool(list: &[i32]) -> Vec<bool> {
    match list {
        [] => vec![],
        [0, rest @ ..] => [vec![false], to_bool(rest)].concat(),
        [_, rest @ ..] => [vec![true], to_bool(rest)].concat(),
    }
}

fn to_int(list: &[bool]) -> Vec<i32> {
    match list {
        [] => vec![],
        [false, rest @ ..] => [vec![0], to_int(rest)].concat(),
        [true, rest @ ..] => [vec![1], to_int(rest)].concat(),
    }
}

fn reverse(list: &[bool]) -> Vec<bool> {
    match list {
        [] => vec![],
        [first, rest @ ..] => [reverse(rest), vec![*first]].concat(),
    }
}

// (bit a, bit b, carry in) -> (sum bit, carry out)
fn full_adder(a: bool, b: bool, c: bool) -> (bool, bool) {
    match (a, b, c) {
        (false, false, false) => (false, false),
        (true, false, false) | (false, true, false) | (false, false, true) => (true, false),
        (true, true, false) | (true, false, true) | (false, true, true) => (false, true),
        (true, true, true) => (true, true),
    }
}

fn finish_binary_add(list: &[bool], carry: bool) -> Vec<bool> {
    match (list, carry) {
        ([], false) => vec![],
        ([], true) => vec![true],
        ([x, rest @ ..], c) => {
            let (sum, new_carry) = full_adder(*x, false, c);
            [vec![sum], finish_binary_add(rest, new_carry)].concat()
        }
    }
}

fn do_binary_addition(a: &[bool], b: &[bool], carry: bool) -> Vec<bool> {
    match (a, b) {
        ([], rest) | (rest, []) => finish_binary_add(rest, carry),
        ([x, xs @ ..], [y, ys @ ..]) => {
            let (sum, new_carry) = full_adder(*x, *y, carry);
            [vec![sum], do_binary_addition(xs, ys, new_carry)].concat()
        }
    }
}

fn binary_addition(a: &[i32], b: &[i32]) -> Vec<i32> {
    let ra = reverse(&to_bool(a));
    let rb = reverse(&to_bool(b));
    to_int(&reverse(&do_binary_addition(&ra, &rb, false)))
}

fn invert(list: &[bool]) -> Vec<bool> {
    match list {
        [] => vec![],
        [x, rest @ ..] => [vec![!*x], invert(rest)].concat(),
    }
}

// pad `b` with false (leading zeros, since lists are reversed) until it is as long as `a`
fn pad_to(a: &[bool], b: &[bool]) -> Vec<bool> {
    match (a, b) {
        ([], rest) => rest.to_vec(),
        ([_, xs @ ..], []) => [vec![false], pad_to(xs, &[])].concat(),
        ([_, xs @ ..], [y, ys @ ..]) => [vec![*y], pad_to(xs, ys)].concat(),
    }
}

// keep only as many bits as `a` has (drops the overflow carry)
fn take_like(a: &[bool], b: &[bool]) -> Vec<bool> {
    match (a, b) {
        ([_, xs @ ..], [y, ys @ ..]) => [vec![*y], take_like(xs, ys)].concat(),
        _ => vec![],
    }
}

fn strip_zeros(list: &[i32]) -> Vec<i32> {
    match list {
        [0, rest @ ..] if !rest.is_empty() => strip_zeros(rest),
        _ => list.to_vec(),
    }
}

// a - b, assuming a >= b
fn binary_subtraction(a: &[i32], b: &[i32]) -> Vec<i32> {
    let ra = reverse(&to_bool(a));
    let rb = reverse(&to_bool(b));
    let twos = do_binary_addition(&invert(&pad_to(&ra, &rb)), &[true], false);
    let sum = take_like(&ra, &do_binary_addition(&ra, &twos, false));
    strip_zeros(&to_int(&reverse(&sum)))
}

fn main() {
    println!("{:?}", binary_addition(&[1, 1, 1, 1, 0], &[1, 0, 1, 1]));       // 101001
    println!("{:?}", binary_addition(&[1, 0, 0, 1, 1, 0, 1], &[0, 0, 1, 0, 0, 1, 0])); // 1011111
    println!("{:?}", binary_addition(&[1, 0, 0, 1, 0, 0, 1], &[0, 0, 1, 1, 0, 0, 1])); // 1100010
    println!("{:?}", binary_addition(&[1, 0, 0, 0, 1, 1, 1], &[1, 0, 1, 1, 0]));       // 1011101
    println!("{:?}", binary_subtraction(&[1, 0, 1, 0, 0, 1], &[1, 0, 1, 1]));          // 41-11=30 -> 11110
    println!("{:?}", binary_subtraction(&[1, 1, 0, 0, 0, 1, 0], &[0, 0, 1, 1, 0, 0, 1])); // 98-25=73 -> 1001001
    println!("{:?}", binary_subtraction(&[1, 0, 1], &[1, 0, 1]));                      // 0
}
