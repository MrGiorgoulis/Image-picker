use std::io;
use std::env;
use std::fs;
use std::path::Path;

fn move_file(file_path: &Path, target_path: &Path) -> io::Result<bool> {
    println!("{:?}, {:?}", file_path, target_path);
    match fs::copy(file_path, target_path) {
        Ok(a) => {
            println!("File copied successfully {:?} {}",file_path.file_stem().unwrap(), a);
            Ok(true)
        },
        Err(e) => {
            println!("There was an error when trying to copy file {}", e);
            Ok(false)
        },
    }
}

fn main() {

    let mut file_count = 0;

    //Argument Collection
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        eprintln!("Wrong command format!\
        The correct format is: \
        file_type path_to_jpeg_directory path_to_raw_directory");
        return;
    }

    let target_file_type = &args[0];

    let source_path = &args[1];
    let source_path: &Path = Path::new(source_path);

    let target_path = &args[2];
    let target_path = Path::new(target_path); 


    if !source_path.is_dir() {
        eprintln!("The path provided is not a directory: {}", source_path.display());
        return;
    }

    if !target_path.is_dir() {
        eprintln!("The path provided is not a directory: {}", source_path.display());
        return;
    }
    
    match fs::read_dir(source_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) =>{
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {

                                    let target_type_file_name 
                                                        = path
                                                        .file_stem()
                                                        .unwrap()
                                                        .to_str()
                                                        .unwrap();

                                    let final_file_name = format!("{}.{}", target_type_file_name, target_file_type);

                                    let mut path_buf = target_path.to_path_buf();
                                    path_buf.push(final_file_name.clone());
                                    let new_path = path_buf.as_path();

                                    let mut path_buf = source_path.to_path_buf();
                                    path_buf.push(final_file_name.clone());
                                    let new_source_path = path_buf.as_path();

                                    
                                    match move_file(new_path , new_source_path) {
                                        Ok(success) =>  
                                            if success == true {file_count += 1}

                                        Err(e) => 
                                            println!("Error: {} while trying to move file: {}", e, target_type_file_name)
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => println!("Error reading entry {}", e)
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e)
    }
}
