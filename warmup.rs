fn hitchhiker(list: &[i32]) -> bool {
    match list {
        [] => false,
        [42, ..] => true,
        [_, rest @ ..] => hitchhiker(rest),
    }
}

fn everyother(list: &[i32]) -> Vec<i32> {
    match list {
        [_, second, rest @ ..] => {
            let mut result = vec![*second];
            result.extend(everyother(rest));
            result
        }
        _ => vec![],
    }
}

fn is_prime_helper(n: i32, d: i32) -> bool {
    match (d * d > n, n % d == 0) {
        (true, _) => true,
        (false, true) => false,
        (false, false) => is_prime_helper(n, d + 1),
    }
}

fn is_prime(n: i32) -> bool {
    n >= 2 && is_prime_helper(n, 2)
}

fn find_pair(n: i32, p: i32) -> i32 {
    match is_prime(p) && is_prime(n - p) {
        true => p,
        false => find_pair(n, p + 1),
    }
}

fn goldbach(n: i32) {
    match n > 2 && n % 2 == 0 {
        false => println!("Error: {} must be an even integer greater than 2", n),
        true => {
            let p = find_pair(n, 2);
            println!("{} + {} = {}", p, n - p, n);
        }
    }
}

fn main() {
    println!("{}", hitchhiker(&[1, 2, 42, 7]));
    println!("{}", hitchhiker(&[1, 2, 3]));
    println!("{:?}", everyother(&[3, 5, 7, 11, 13, 17, 19, 29, 31, 41, 43]));
    goldbach(28);
    goldbach(100);
    goldbach(7);
    goldbach(2);
}
