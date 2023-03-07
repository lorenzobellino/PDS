fn make_box(a: i32) -> Box<(i32, i32)> {
    let b = Box::new((a, 1));
    return b;
}
fn main() {
    let b = make_box(5);
    let c = b.0 + b.1;
    println!("{}", c);
}
