fn main() {
    println!("borrow 1");
    let mut a = 1;
    println!("borrow 2");
    let b = &mut a;
    println!("borrow 3");
    let c = &mut a;
    println!("borrow 4");

    *b += 1;
    println!("borrow 5");
    *c *= 2;
    println!("borrow 6");

    mul(c);

    println!("a {:?}", a);
}

fn mul(c: &mut i32) {
    *c *= 2;
}
