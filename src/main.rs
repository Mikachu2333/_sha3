use arboard::Clipboard;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //let mut args: Vec<String> = vec![String::from("aaa")];
    //args.push(r#".\\Program Files"#.to_string());

    if args.len() == 1 {
        println!(
            r#"USAGE OF "_sha3"

This program is used to calc value(s) of SHA3_256 for given file path(s).

Please note that once you give it a DIR path(s), it will calc the values of all files in the directory (and files in subdirectory).

If you give the only value of a file (not a dir), it will automatically copy the calced value with the following style to your clipboard.
"<FILENAME>" SHA3_256=<VALUE>
"#
        );
        return;
    }

    for i in args[1..].iter().map(PathBuf::from) {
        let to_check_path = i
            .file_name()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_str()
            .unwrap();

        let checked_dir = i.parent().unwrap_or(std::path::Path::new(""));
        let checked_name = rm_special(to_check_path);

        let checked_path = checked_dir.join(checked_name);
        //println!("{}", checked_path.to_string_lossy());

        if checked_path.exists() {
            if checked_path.is_dir() {
                for (path, dir) in dir_enum(&checked_path) {
                    if !dir {
                        let _ = packed_info(path);
                    }
                }
            } else {
                let tmp_info = packed_info(checked_path);
                if args.len() == 2 {
                    set_clipboard(&tmp_info);
                }
            }
        } else {
            println!("NOT EXIST: {}", checked_path.to_string_lossy());
        }
    }
}

fn rm_special(name: &str) -> String {
    let mut temp = name.replace(r#"\"#, r#"/"#).replace(r#"//"#, r#"/"#);
    
    while temp.ends_with(r#"""#) {
        temp = temp[0..temp.len() - 2].to_string();
    }
    while name.ends_with(r#"\"#) {
        temp = temp[0..temp.len() - 2].to_string();
    }
    while name.starts_with(r#"""#) {
        temp = temp[1..temp.len() - 1].to_string();
    }
    temp
}

fn dir_enum(dir_path: &std::path::Path) -> Vec<(PathBuf, bool)> {
    let mut path_collected: Vec<(PathBuf, bool)> = Vec::new();
    match dir_path.read_dir() {
        Ok(dir_info) => {
            for each in dir_info {
                match each {
                    Ok(each_path) => {
                        let tmp_path = each_path.path();
                        let tmp_path_isdir = tmp_path.is_dir();
                        path_collected.push((tmp_path, tmp_path_isdir))
                    }
                    Err(err_msg) => panic!("{}", err_msg),
                }
            }
        }
        Err(err_msg) => panic!("{}", err_msg),
    }
    path_collected
}

fn packed_info(path: PathBuf) -> String {
    let sha3_256 = calc_sha3_256(&path);
    let name = path.file_name().unwrap().to_string_lossy();
    let formatted_info = format!("<{}> SHA3_256={}", name, sha3_256);
    println!("{}", formatted_info);
    formatted_info
}

fn calc_sha3_256(file_path: &PathBuf) -> String {
    let mut file = File::open(file_path).expect("Error Open File.");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Error Read File.");

    let mut hasher = Sha3_256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:X}", result)
}

fn set_clipboard(information: &String) {
    let mut clipboard = Clipboard::new().expect("Error Read Clipboard.");
    match clipboard.set_text(&**information) {
        Ok(_) => println!("Success Write Clipboard!"),
        Err(err_msg) => panic!("{}", err_msg),
    }
}
