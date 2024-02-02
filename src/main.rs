use clap::Parser;
pub mod file_types;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long)]
    file_name: String,
    #[arg(short, long)]
    ext_name: String,
}

fn main() {
    let args = Cli::parse();

    let file_name = args.file_name;
    let file_type: Vec<&str> = file_name.split(".").collect();

    // a simple error and input sanitizer
    if file_type.len() > 2 {
        println!("incorrect file type!");
        return;
    }

    match file_type[1] {
        "txt" => {
            println!("This is a txt file")
        }
        _ => {
            println!("Unsupported file type")
        }
    }

    // todo: add path buf and experiment reading a file buffer from a format and write in another format.
    // make more research on paths
}
