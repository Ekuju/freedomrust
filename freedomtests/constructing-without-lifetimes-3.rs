// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed {
    x: &i32,
}

// Annotate lifetimes to impl.
impl Default for Borrowed {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}