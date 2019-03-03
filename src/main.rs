extern crate regex;

mod transpiler;

fn main() {
    let code = "def f(x, y, z) g(name, 2) end";

    println!("\n\n");
    println!("NP: {}\n\n", code);
    println!("JS: {}", transpiler::transpile(&code));
    println!("\n\n");
}
