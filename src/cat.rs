pub mod cat {
    use std::fs::read_to_string;

    enum ArgOptions {
        NoArgs = 0,
        ShowLineNumber = 2,
        Rev = 4,
    }

    pub fn show_cat(args: &Vec<String>) {
        if args.len() == 1 {
            eprintln!("pass file path as argument");
        } else {
            let mut cur_index = args.len() - 1;

            let mut option: u8 = ArgOptions::NoArgs as u8;
            let mut path: String = String::new();

            loop {
                let arg = args[cur_index].as_str();
                match arg {
                    "-n" => option |= ArgOptions::ShowLineNumber as u8,
                    "-rev" => option |= ArgOptions::Rev as u8,
                    "--help" => {
                        show_help();
                        return;
                    }
                    _ => {
                        path = arg.to_string();
                    }
                }

                cur_index -= 1;
                if cur_index == 0 {
                    break;
                }
            }

            if path.is_empty() {
                panic!(
                "WTF you haven't given file path and expecting to show the content you cunt!!!!"
            );
            } else {
                show_result(path, option);
            }
        }
    }

    fn show_result(file_path: String, option: u8) {
        let content = read_file_data(file_path);

        match content {
            Ok(content) => {
                let show_number =
                    option & ArgOptions::ShowLineNumber as u8 == ArgOptions::ShowLineNumber as u8;

                let is_rev = option & ArgOptions::Rev as u8 == ArgOptions::Rev as u8;

                let mut line_number: usize;

                if is_rev {
                    line_number = content.len();
                    // println!("Total content len: {}", content.len());
                } else {
                    line_number = 1;
                }

                for _ in 0..content.len() {
                    if show_number {
                        print!("{}. ", line_number);
                    }

                    let line_content: &String = &content[line_number - 1];

                    if is_rev {
                        line_number -= 1;
                    } else {
                        line_number += 1;
                    }

                    println!("{line_content}");
                }
            }
            Err(err) => println!("Error: {err}"),
        }
    }

    fn read_file_data(file_path: String) -> Result<Vec<String>, String> {
        let content = read_to_string(file_path);

        match content {
            Ok(data) => {
                let mut result: Vec<String> = data
                    .split("\n")
                    .map(|line: &str| line.to_string())
                    .collect();

                result = result[..result.len() - 1].to_vec();
                // println!("Result len : {}", result.len());

                Ok(result)
            }
            Err(err) => return Err(err.to_string()),
        }
    }

    pub fn show_help() {
        println!("cat [OPTION]... [FILE]...");
        println!("Prints file content to standard output");

        println!("Options: ");
        println!(" -n\t shows line number");
        println!(" -rev\t print content with bottom to top");
    }
}
