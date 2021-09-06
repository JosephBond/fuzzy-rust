use std::collections::HashMap;
use std::hash::Hash;
use std::collections::HashSet;

struct Predicate<'a, T> {
    free_vars: HashSet<&'a str>,
    app: &'a mut dyn FnMut(HashMap<&str, T>) -> bool
}

enum LogicOp {
    And,
    Or
}

pub fn combine_preds<'a, T>(operad_1: Predicate<&'a T>, operad_2: Predicate<&'a T>, op: LogicOp) -> bool {
    let x = operad_2.free_vars.iter().cloned().collect::<HashSet<&str>>();
    let y = operad_1.free_vars.iter().cloned().collect::<HashSet<&str>>().union(&x);
    let z = y.collect::<HashSet<&str>>();
    // Predicate {
    //     free_vars: y,
    //     app: &mut { |args| (operad_1.app)(args) && (operad_2.app)(args) },
    // }
    true
}
fn key_subset<T>(keys: Vec<&str>, map: HashMap<&str, T>) -> bool {
    let mut res = true;

    for key in keys {
        res = res && map.contains_key(key);
    }
    res
}

fn main() {

}