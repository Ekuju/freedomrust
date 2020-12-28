#[derive(Debug)]
struct Test (pub i32);

fn main() {
    println!("borrow 1");
    let mut a = Test(1);
    println!("{:?}=1", a);
    let b = &mut a;
    let c = &mut a;

    b.0 += 1;
    println!("{:?}=2", a);
    c.0 *= 2;
    println!("{:?}=4", a);

    ownership(a);

    mul(c);

    println!("{:?}=invalid, but maybe 16", c);
}

fn ownership(mut a: Test) {
    println!("ownership 1 {:?}=4", a);
    a.0 *= 2;

    println!("ownership 2 {:?}=8", a);
}

fn mul(c: &mut Test) {
    c.0 *= 2;
    println!("{:?}=invalid, but maybe 16", c);
}