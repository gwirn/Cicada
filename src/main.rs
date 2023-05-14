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

// path where the file storing all dates is located
const DATE_FILE_PATH: &str = "./src/dates/date.file";
// path where the salt file for key generation should be stored
const SALT_LOC: &str = "./src/dates/.salt";
// path where the password should be stored
const PWD_LOC: &str = "./src/dates/.pwd";
// path where already alerted appointment ids are stored
const ALERT_LOC: &str = "./src/dates/.alerted";

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
