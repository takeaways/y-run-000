// import from = mod
mod print; 
mod vars;
mod types;
mod string;
mod array;

fn main() {
    println!("Hello, world!");
    print::run();
    vars::vars();
    types::types();
    string::string_fn();
    string::run();
    array::run();
}
