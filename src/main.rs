extern crate regex;
extern crate tempfile;

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut tab_width = 4i32;
    let mut mode = Mode::Tabify;
    let mut files = Vec::<String>::new();

    for arg in args.into_iter().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => help(),
            "-V" | "--version" => version(),
            "-t" | "--tabify" => { mode = Mode::Tabify },
            "-u" | "--untabify" => { mode = Mode::Untabify },
            "-w" | "--width" => { tab_width = arg.parse().unwrap_or_else(|_| usage_error("Invalid space count provided!")) },
            _ => { files.push(arg) },
        }
    }

    if files.is_empty() {
        //stdin to stdout mode
        let reader = BufReader::new(std::io::stdin());
        let writer = BufWriter::new(std::io::stdout());

        if let Err(e) = process(reader, writer, &mode, &tab_width) {
            eprintln!("{}", e);
        }
    }
    else {
        for f in &files {
            let path = std::path::Path::new(f);
            if let Err(e) = process_path(path, &mode, &tab_width) {
                eprintln!("{}: {}", path.display(), e);
            }
        }
    }
}

enum Mode {
    Tabify,
    Untabify
}

fn help() {
    println!("tabify by NeoSmart Technologies - https://neosmart.net/");
    println!("");
    println!("USAGE: tabify [OPTIONS] [file1 [file2 ..]]");
    println!("\t -t --tabify        Convert spaces to tabs");
    println!("\t -u --untabify      Convert tabs to spaces");
    println!("\t -w --width WIDTH   Set the tab width in spaces (default: 4)");
    println!("\t -h --help          Print this help message and exit");
    println!("\t -V --version       Display version information");

    std::process::exit(0);
}

fn version() {
    println!("tabify {} by NeoSmart Technologies - https://neosmart.net/",
             env!("CARGO_PKG_VERSION"));
    println!("Report issues at https://github.com/neosmart/tabify");

    std::process::exit(0);
}

fn usage_error(msg: &str) -> ! {
    println!("Error: {}", msg);
    println!("See tabify --help for usage info");

    std::process::exit(-1);
}

fn process_path(path: &Path, mode: &Mode, width: &i32) -> Result<(), String> {
    use tempfile::tempfile;

    if !path.exists() {
        return Err("file not found!".into());
    }
    if !path.is_file() {
        return Err("path does not refer to a file!".into());
    }

    let file = File::open(path).map_err(|e| format!("{}", e))?;
    let reader = BufReader::new(file);
    let temp = tempfile().map_err(|e| format!("{}", e))?;
    let writer = BufWriter::new(temp);

    return process(reader, writer, mode, width);
}

fn process<R,W>(reader: BufReader<R>, mut writer: BufWriter<W>, mode: &Mode, width: &i32) -> Result<(), String> where R: Read, W: Write {
    let transform = match *mode {
        Mode::Tabify => tabify,
        Mode::Untabify => untabify
    };

    for line in reader.lines() {
        let line = line.map_err(|e| format!("{}", e))?;
        let new_line = transform(&line, *width);
        writer.write(new_line.as_bytes()).map_err(|e| format!("{}", e))?;
        writer.write(&[0xAu8]).map_err(|e| format!("{}", e))?;
    }

    return Ok(());
}

enum ParseState {
    Leader,
    Remainder
}

fn tabify(line: &str, width: i32) -> String {
    let mut new_line = Vec::<char>::new();
    let mut space_count = 0;
    let mut state = ParseState::Leader;

    for c in line.chars() {
        match state {
            ParseState::Leader =>
                match c {
                    ' ' => {
                        space_count += 1;
                        if space_count == width {
                            space_count = 0;
                            new_line.push('\t');
                        }
                    },
                    _ => {
                        //end of leading spaces
                        state = ParseState::Remainder;
                        for _ in 0..space_count {
                            new_line.push(' ');
                        }
                        new_line.push(c);
                    }
                },
                ParseState::Remainder => {
                    new_line.push(c);
                }
        }
    }

    return new_line.into_iter().collect();
}

#[test]
fn tabify_test() {
    assert_eq!("  2 leading spaces", tabify("  2 leading spaces", 4));
    assert_eq!("\t4 leading spaces", tabify("    4 leading spaces", 4));
    assert_eq!("\t   7 leading spaces", tabify("       7 leading spaces", 4));
    assert_eq!("\tspaces    in middle", tabify("    spaces    in middle", 4));
}

fn untabify(line: &str, width: i32) -> String {
    let mut new_line = Vec::<char>::new();
    let mut state = ParseState::Leader;

    for c in line.chars() {
        match state {
            ParseState::Leader =>
                match c {
                    '\t' => {
                        for _ in 0..width {
                            new_line.push(' ');
                        }
                    },
                    _ => {
                        //end of leading tabs
                        state = ParseState::Remainder;
                        new_line.push(c);
                    }
                },
                ParseState::Remainder => {
                    new_line.push(c);
                }
        }
    }

    return new_line.into_iter().collect();
}

#[test]
fn untabify_test() {
    assert_eq!("    1 leading tab", untabify("\t1 leading tab", 4));
    assert_eq!("        2 leading tabs", untabify("\t\t2 leading tabs", 4));
    assert_eq!("        \ttab spaces tab", untabify("\t    \ttab spaces tab", 4));
}
