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

    println!("file {:?}", file_type[1]);

    match file_type[1] {
       "txt" => {
        println!("This is a txt file")
       }, 
       _ => {
        println!("Unsupported file type")
       }
    }

}
