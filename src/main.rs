mod farlib;

use std::{env, fs};
use std::fs::File;
use std::io::Write;

fn print_help(args : Vec<String>) {
    println!("Usage: {} <command> <archive name> [FILES...]", args[0]);
    println!("Commands:");
    println!("  help - show this help message");
    println!("  test - test if file is a valid archive");
    println!("  list - list files in archive");
    println!("  extract - extract files from archive to current directory (will make a new directory)");
    return;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        print_help(args);
        return;
    }
    let command = &args[1];
    let archive_name = &args[2];
    match command.as_ref() {
        "help" => {
            print_help(args);
        },
        "test" => {
            let mut file = fs::read(archive_name).expect("Failed to read file");
            let test = farlib::test(&file);
            match test {
                Ok(archive_obj) => {
                    println!("{} is a valid archive", archive_name);
                    println!("Archive version: {}", archive_obj.version);
                },
                Err(e) => {
                    println!("{} is not a valid archive: {}", archive_name, e);
                }
            }
        },
        "list" => {
            let file = fs::read(archive_name).expect("Failed to read file");
            let test = farlib::test(&file);
            match test {
                Ok(archive_obj) => {
                    let files = archive_obj.file_list;
                    for file in files {
                        println!("{} ({} bytes)", file.name, file.size);
                    }
                },
                Err(e) => {
                    println!("{} is not a valid archive: {}", archive_name, e);
                }
            }
        },
        "extract" => {
            let file = fs::read(archive_name).expect("Failed to read file");
            let test = farlib::test(&file);
            match test {
                Ok(archive_obj) => {
                    let archive_with_data = archive_obj.load_file_data(&file);
                    // make a new directory
                    let dir_path = format!("{}/{}", env::current_dir().unwrap().to_str().unwrap(), archive_name.split(".").next().unwrap());
                    fs::create_dir_all(dir_path.clone()).expect("Failed to create directory");
                    let files = archive_with_data.file_data;
                    for file in files {
                        let mut file_path = format!("{}/{}", dir_path.clone(), file.name);
                        let mut file_data = file.data;
                        let mut file_handle = File::create(file_path).expect("Failed to create file");
                        file_handle.write_all(&file_data).expect("Failed to write file");
                    }
                },
                Err(e) => {
                    println!("{} is not a valid archive: {}", archive_name, e);
                }
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}
