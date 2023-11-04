pub mod ls {
    use std::env;
    use colored::Colorize;
    use std::fs::{ReadDir, read_dir};
    use std::io::Error;
    use std::path::PathBuf;

    pub fn list(args: &Vec<String>) {
        let mut given_paths: Vec<PathBuf> = Vec::new();
        let mut_args = args[0..args.len()].to_vec();
        let mut cur_index = mut_args.len() - 1;

        loop {
            let arg = mut_args[cur_index].as_str();

            match arg {
                "-l" => todo!(),
                "-a" => todo!(),
                "--help" => {
                    show_help();
                    return;
                }
                "ls" => {
                    if given_paths.len() == 0 {
                        given_paths.push(env::current_dir().unwrap());
                    }
                }
                _ => {
                    given_paths.push(PathBuf::from(arg));
                }
            }

            if cur_index != 0 {
                cur_index -= 1;
            }

            if cur_index == 0 {
                break;
            }
        }

        for given_p in given_paths.iter() {
            let result = get_dir_list(&given_p);

            match result {
                Ok(files) => {
                    if given_paths.len() > 1 {
                        println!("{}:", given_p.to_str().unwrap());
                    }
                    show_in_ui(files);
                }
                Err(err) => {
                    if given_p.is_file() {
                        show_just_file_ui(given_p);
                    } else {
                        eprintln!("{:?} {}", &given_p, err)
                    }
                }
            }
        }
    }

    fn show_just_file_ui(file: &PathBuf) {
        println!("{}", file.to_str().unwrap().to_string());
    }

    fn show_in_ui(files: ReadDir) {
        for i in files {
            match i {
                Ok(result) => {
                    if let Some(file_name) = result.path().file_name() {
                        let file_name_str: String = file_name.to_str().unwrap().to_string();
                        if result.path().is_dir() {
                            print!("{} ", file_name_str.green());
                        } else {
                            print!("{} ", file_name_str);
                        }
                    }
                }
                Err(err) => eprintln!("{:?}", err),
            }
        }

        println!("\n");
    }

    fn get_dir_list(path: &PathBuf) -> Result<ReadDir, Error> {
        let cur_dir: Result<&PathBuf, Error> = Ok(&path);
        let files = cur_dir?;
        let dir_files = read_dir(files)?;

        Ok(dir_files)
    }

    pub fn show_help() {
        println!("ls [OPTION]... [FILE]...");
        println!("List information about the FILES (By default current directory");

        println!("Options: ");
        println!(" -a\t list includes hidden files");
    }
}
