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

    match file_type[1] {
       "txt" => {
        println!("This is a txt file")
       }, 
       _ => {
        println!("Unsupported file type")
       }
    }

    // todo: add path buf and experiment reading a file buffer from a format and write in another format.

}
