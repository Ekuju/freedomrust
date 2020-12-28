fn main() {
    let mut a = 1;
    let b = &mut a;
    let c = &mut a;

    *b += 1;
    *c *= 2;

    println!("a {:?}", a);
}