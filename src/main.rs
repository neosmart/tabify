extern crate regex;
extern crate tempfile;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut tab_width: i32 = 4;
    let mut mode: Mode = Mode::Tabify;
    let mut files = Vec::<String>::new();

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => help(),
            "-V" | "--version" => version(),
            "-u" | "--untabify" => { mode = Mode::Untabify },
            "-s" | "--spaces" => { tab_width = arg.parse().unwrap_or_else(|_| argument_error("Invalid space count provided!")) },
            f @ _ => { files.push(f.to_owned()) },
        }
    }

    for f in &files {
        let path = std::path::Path::new(f);
        process_path(path, &mode, &tab_width);
    }
}

enum Mode {
    Tabify,
    Untabify
}

fn help() {
    println!("tabify by NeoSmart Technologies - https://neosmart.net/");
    println!("");
    println!("USAGE: tabify [OPTIONS] file1..");
    println!("\t -t --tabify        Convert spaces to tabs");
    println!("\t -u --untabify      Convert tabs to spaces");
    println!("\t -s --spaces SPACES Set the tab width (default: 4)");
    println!("\t -h --help          Print this help message and exit");
    println!("\t -V --version       Display version information");
}

fn version() {
    println!("tabify 1.0 by NeoSmart Technologies - https://neosmart.net/");
    println!("Report issues at https://github.com/neosmart/tabify");
}

fn argument_error(msg: &str) -> ! {
    println!("Error: {}", msg);
    println!("See tabify --help for usage info");
    std::process::exit(-1);
}

fn process_path(path: &Path, mode: &Mode, width: &i32) -> Result<(), String> {
    let path_str = path.as_os_str().to_string_lossy();
    if !path.exists() {
        eprintln!("{}: file not found!", path_str);
    }
    if !path.is_file() {
        eprintln!("{} does not refer to a file!", path_str);
    }

    use std::fs::File;
    use std::io::{BufReader, BufWriter};
    use std::io::prelude::*;
    use tempfile::tempfile;


    let mut file = File::open(path).map_err(|e| format!("{}", e))?;
    let mut reader = BufReader::new(file);
    let mut temp = tempfile();

    let mut line: String = "".to_owned();
    match *mode {
        Mode::Tabify => while (reader.read_line(&mut line).unwrap() != 0) { tabify(&mut line, width) },
        Mode::Untabify => while (reader.read_line(&mut line).unwrap() != 0) { untabify(&mut line, width) }
    };

    return Ok(());
}

fn tabify(line: &str, width: &i32) {

}

fn untabify(line: &str, width: &i32) {
    while (line.
}
