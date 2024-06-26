use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn move_file(file_path: &Path, target_path: &Path) -> io::Result<bool> {
    match fs::copy(file_path, target_path) {
        Ok(a) => {
            println!(
                "File copied successfully {:?}",
                file_path.file_name().unwrap()
            );
            Ok(true)
        }
        Err(e) => {
            println!("There was an error when trying to copy file {}", e);
            Ok(false)
        }
    }
}

fn main() {
    let mut file_count = 0;

    //Argument Collection
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        eprintln!(
            "Wrong command format!\
        The correct format is: \
        file_type path_to_jpeg_directory path_to_raw_directory"
        );
        return;
    }

    let target_files_type = &args[0];

    let target_path: &String = &args[1];
    let target_path: &Path = Path::new(target_path);

    let source_path = &args[2];
    let source_path = Path::new(source_path);

    if !target_path.is_dir() {
        eprintln!(
            "The path provided is not a directory: {}",
            target_path.display()
        );
        return;
    }

    if !target_path.is_dir() {
        eprintln!(
            "The path provided is not a directory: {}",
            target_path.display()
        );
        return;
    }

    match fs::read_dir(target_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext.eq_ignore_ascii_case("jpg")
                                    || ext.eq_ignore_ascii_case("jpeg")
                                {
                                    let file_name = path.file_stem().unwrap().to_str().unwrap();

                                    let final_file_name =
                                        format!("{}.{}", file_name, target_files_type);

                                    let mut path_buf = source_path.to_path_buf();
                                    path_buf.push(final_file_name.clone());
                                    let new_path = path_buf.as_path();

                                    let mut path_buf = target_path.to_path_buf();
                                    path_buf.push(final_file_name.clone());
                                    let new_target_path = path_buf.as_path();

                                    match move_file(new_path, new_target_path) {
                                        Ok(success) => {
                                            if success == true {
                                                file_count += 1
                                            }
                                        }

                                        Err(e) => println!(
                                            "Error: {} while trying to move file: {}",
                                            e, file_name
                                        ),
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => println!("Error reading entry {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }

    println!("{} files have been copied", file_count);
}
