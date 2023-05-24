fn main() {
    let x= 10;
    let y: Option<u8> = None;

    let sum  = x + y.unwrap_or(0);
}