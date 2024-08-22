use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::{fs, time::UNIX_EPOCH};
// use std::path::Path;

const FILE_PATH: &str = "build_info.h";

fn create_new_version_header_file(file: &str) -> () {
    let time_stamp = std::time::SystemTime::now();
    let time_samp = time_stamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    println!(
        "\nNew file create please check version code \n timestamp:{:?}\n version:1.0.0\n file:{:?}\n",
        time_samp,
        file
    );
    let rst_str = format!(
        "#ifndef BUILD_INFO_H\n\
        #define BUILD_INFO_H\n\
        #define BUILD_TIME_STAMP {}\n\
        #define BUILD_VERSION_MAJOR 1\n\
        #define BUILD_VERSION_MINOR 0\n\
        #define BUILD_VERSION_PATCH 0\n\
        #endif // BUILD_INFO_H\n",
        time_samp
    );
    let mut file = fs::File::create(file).unwrap();
    writeln!(file, "{}", rst_str).unwrap();
    ()
}

fn update_version_header_file(file: &str) -> () {
    let time_stamp = std::time::SystemTime::now();
    let time_stamp = time_stamp
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    let tim_str = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "Head file update\ntime is {:?} \nnew timestamp:{:?} ",
        time_stamp, tim_str,
    );
    let file_path = fs::File::open(file).unwrap();
    let reader = BufReader::new(file_path);
    let mut lines = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if i == 2 {
            let updateline = format!("#define BUILD_TIME_STAMP {}", time_stamp);
            lines.push(updateline);
        } else {
            lines.push(line);
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .unwrap();
    for line in lines.iter() {
        writeln!(file, "{}", line).unwrap();
    }

    ()
}

fn main() {
    if let Ok(metadata) = fs::metadata(FILE_PATH) {
        if metadata.is_file() {
            update_version_header_file(FILE_PATH)
        } else {
            println!("some error ")
        }
    } else {
        create_new_version_header_file(FILE_PATH)
    }
}
