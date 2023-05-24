fn main() {
    let five = Some(5);
    let six = plu_one(five);
    let none = plu_one(None);
}

fn plu_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1) // Should wrap it in Some, coz of return type in Option
    }
}