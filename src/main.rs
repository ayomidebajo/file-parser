use std::env;
pub mod file_types;

fn main() {
    let args = env::args().skip(1).next();    
    
    match args {
        Some(name) => {
            println!("Hello, {}", name);
        },
        None => {
            println!("Didn't find any name, going back to default");
            println!("Hello, World");
        }
    }

}
