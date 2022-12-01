// A starting point with an example I'll probably follow often in Advent of Code.
use parse_display::{Display, FromStr};
const INPUT: &'static str = "bee 20";

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a} {b}")]
struct ExampleStruct {
    a: String,
    b: i64
}

fn main() -> () {
    let parsed: ExampleStruct = INPUT.parse().unwrap();
    let (a, b) = (parsed.a, parsed.b);
    println!("{a} {b}");
}
