fn lookup(word: &str, chinese: &[&str], english: &[&str], n: i32) -> Option<i32> {
    match (chinese, english) {
        ([c, cs @ ..], [e, es @ ..]) => {
            if *c == word || *e == word {
                Some(n)
            } else {
                lookup(word, cs, es, n + 1)
            }
        }
        _ => None,
    }
}

fn join(nums: &[i32], sep: &str) -> String {
    nums.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(sep)
}

fn go(input: &[&str]) {
    let chinese: Vec<&str> = vec!["ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu", "shi"];
    let english: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

    let nums: Vec<i32> = input
        .iter()
        .filter_map(|w| lookup(w, &chinese, &english, 0))
        .collect();

    let sum: i32 = nums.iter().fold(0, |acc, x| acc + x);
    let product: i32 = nums.iter().fold(1, |acc, x| acc * x);

    println!("Translation: {}", join(&nums, " "));
    println!("Addition: {} = {}", join(&nums, " + "), sum);
    println!("Multiplication: {} = {}", join(&nums, " * "), product);
}

fn main() {
    let input = vec!["yi", "nine", "six", "ba"];
    go(&input);
    println!();
    let input = vec!["yi", "josh", "three", "si"];
    go(&input);
}
