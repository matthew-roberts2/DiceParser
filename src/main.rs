use std::env;
use dice_roll_parser::{parse, print};

fn main() {
    let args: Vec<String> = env::args().collect();

    let expression = args.get(1).expect("No expression provided!");

    let parse_tree = parse(expression).expect("Didn't parse right!");

    println!("{}", print(&parse_tree));

    println!("{}", parse_tree.evaluate().expect("Failed to evaluate parse tree"));
}
