use chrono::Datelike;
use chrono::Local;
use std::fs;
use std::io;
mod argparse;
mod date_utils;
mod encrypt;
mod view_dates;
mod view_month;
use crate::argparse::argparse;
use crate::date_utils::read_file;

const DATE_FILE_PATH: &str = "./src/dates/date.file";
const SALT_LOC: &str = "./src/dates/.salt";
const PWD_LOC: &str = "./src/dates/.pwd";

fn main() -> io::Result<()> {
    let now = Local::now().date_naive();
    let cur_day: u32 = now.day();
    let cur_month: u32 = now.month();
    let cur_year: i32 = now.year();

    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&DATE_FILE_PATH)
    {
        Ok(_) => println!("Created date.file at '{}'", &DATE_FILE_PATH),
        Err(_) => {}
    }
    let file_content = read_file(&DATE_FILE_PATH);
    argparse(file_content, cur_day, cur_month, cur_year);
    Ok(())
}
