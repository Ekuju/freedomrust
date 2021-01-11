#[derive(Debug)]
struct Test(pub i32);

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

    let mut test = ClassName { val: &mut a };
    println!("{:?}=4", test.val);
    test.modify();
    println!("{:?}=8", test.val);
    println!("{:?}=8", a);

    mul(c);

    println!("{:?}=16", c);
    println!("{:?}=16", a);
    println!("{:?}=16", test.val);
}

fn mul(c: &mut Test) {
    c.0 *= 2;
    println!("{:?}=16", c);
}

pub struct ClassName {
    val: &mut Test,
}

impl ClassName {
    pub fn modify(&mut self) {
        self.val.0 *= 2;
    }
}
