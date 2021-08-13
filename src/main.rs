// import from = mod
mod print; 
mod vars;
mod types;

fn main() {
    println!("Hello, world!");
    print::run();
    vars::vars();
    types::types();

    
}
