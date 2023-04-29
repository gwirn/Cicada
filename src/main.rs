#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use chrono::Datelike;
use chrono::{Local, NaiveDate};
use regex::Regex;
use std::fs;
use std::io;
use std::io::prelude::*;
mod argparse;
mod date_utils;
mod encrypt;
mod view_dates;
mod view_month;
use crate::argparse::argparse;
use crate::date_utils::{
    append_file, argsort, check_dates, offset_and_time, read_file, remove_entry, saved_data_header,
    time_date_lef, SavedDate,
};
use crate::view_dates::{get_next_n, grep_by_date, grep_by_description, last_added};
use crate::view_month::{appointment_check, month_len, month_view};

const DATE_FILE_PATH: &str = "./src/dates/date.file";
const PASSWORD: &str = "mysecretpassword";
const SALT_LOC: &str = "./src/dates/.salt";

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
