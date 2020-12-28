fn main() {
    println!("borrow 1");
    let mut a = 1;
    println!("borrow 2");
    let b = &mut a;
    println!("borrow 3");
    *b += 1;
    println!("borrow 4");

    let c = &mut a;
    println!("borrow 5");
    *c *= 2;
    println!("borrow 6");

    println!("a {:?}", a);
}
