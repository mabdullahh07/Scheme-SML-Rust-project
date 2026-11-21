const STORES: [&str; 4] = ["The Leftorium", "Sprawl-Mart", "Try-N-Save", "King Toots"];
const GIFTS: [&str; 4] = ["green dress (Marge)", "saxophone book (Lisa)", "slingshot (Bart)", "pacifier (Maggie)"];

// all orderings of the given items
fn permutations(items: &[&'static str]) -> Vec<Vec<&'static str>> {
    if items.is_empty() {
        return vec![vec![]];
    }
    let mut result = Vec::new();
    for i in 0..items.len() {
        let mut rest = items.to_vec();
        let first = rest.remove(i);
        for mut p in permutations(&rest) {
            p.insert(0, first);
            result.push(p);
        }
    }
    result
}

fn pos(list: &[&str], item: &str) -> usize {
    list.iter().position(|x| *x == item).unwrap()
}

// stores[i] and gifts[i] are what happened at stop i+1
fn satisfies(stores: &[&str], gifts: &[&str]) -> bool {
    let sax = pos(gifts, "saxophone book (Lisa)");
    let sling = pos(gifts, "slingshot (Bart)");
    let paci = pos(gifts, "pacifier (Maggie)");
    let try_n_save = pos(stores, "Try-N-Save");

    let clue1 = stores[sax] == "King Toots";
    let clue2 = sling + 1 < 4 && stores[sling + 1] != "Sprawl-Mart";
    let clue3 = stores[1] == "The Leftorium";
    let clue4 = paci == try_n_save + 2;
    clue1 && clue2 && clue3 && clue4
}

fn main() {
    for stores in permutations(&STORES) {
        for gifts in permutations(&GIFTS) {
            if satisfies(&stores, &gifts) {
                println!("Solution:");
                for i in 0..4 {
                    println!("  Stop {}: {} -> {}", i + 1, stores[i], gifts[i]);
                }
            }
        }
    }
}
