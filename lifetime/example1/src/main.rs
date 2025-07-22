#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share<'a>(&'a mut self) -> &'a Self {
        &*self
    }
    fn share<'a>(&'a self) {}
}
fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share();
    println!("{:?}",loan);
}
