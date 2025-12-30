use hell0_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use hello_macro_derive::MyDefault;

#[derive(HelloMacro)]
struct Sd;

#[derive(HelloMacro)]
struct Fendada;

#[derive(MyDefault)]
struct SomeData(u32, String);

#[derive(MyDefault)]
struct User {
    name: String,
    data: SomeData,
}
fn main() {
    Sd::hello_macro();
    Fendada::hello_macro();
}
