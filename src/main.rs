use std::env;

pub mod cat;
pub mod ls;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        show_help();
    } else {
        let arg = args[1].as_str();
        match arg {
            "cat" => cat::cat::show_cat(&args[1..].to_vec()),
            "ls" => ls::ls::list(&args[1..].to_vec()),
            _ => {
                println!("Given command `{arg}` is not supported");
                show_help()
            }
        }
    }
}

fn show_help() {
    println!("Supported commands \n");
    cat::cat::show_help();
    println!("\n");
    ls::ls::show_help();
}
