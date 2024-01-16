use std::env;
pub mod file_types;

fn main() {

    let args: Vec<String> = env::args().skip(1).collect();    
    
    let file_name = args[0].clone();
    let file_type: Vec<&str>= file_name.split(".").collect();
    // a simple error and input sanitizer
    if file_type.len() > 2 {
        println!("incorrect file type!");
        return
    }

    for i in file_type {
        println!("file type {}", i);
    }
    
}
