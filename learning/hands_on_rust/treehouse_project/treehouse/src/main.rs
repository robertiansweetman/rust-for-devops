use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

fn what_is_your_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    name.trim().to_string()
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    // println!("Hello, {} nice to meet you!", name);

    let visitor_list = ["bob", "frank", "ferris"];

    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }
    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("You are not allowed in the Treehouse, {}", name);
    }
}
