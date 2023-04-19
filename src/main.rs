#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use chrono::Datelike;
use chrono::{Local, NaiveDate};
use regex::Regex;
use std::fs;
use std::io;
use std::io::prelude::*;
mod date_utils;
mod view_dates;
mod view_month;
use crate::date_utils::{
    append_file, argsort, check_dates, offset_and_time, read_file, remove_entry, saved_data_header,
    time_date_lef, SavedDate,
};
use crate::view_dates::{get_next_n, grep_by_date, grep_by_description, last_added};
use crate::view_month::{appointment_check, month_len, month_view};

const DATE_FILE_PATH: &str = "./src/dates/date.file";

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

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            let first_day_month = NaiveDate::from_ymd_opt(cur_year, cur_month as u32, 1)
                .expect("First day of the month couldn't be calculated")
                .weekday()
                .number_from_monday();
            let event_dates: Vec<u32> = appointment_check(&file_content, &cur_month, &cur_year);
            let month_hash = month_len(&cur_year, &cur_month);
            month_view(&month_hash, &cur_day, &event_dates, &first_day_month);
        }
        2 => {
            match &args[1][..] {
                "check" => {
                    check_dates(&file_content);
                    println!("Checked for upcomming dates")
                }
                "-h" | "--help" => {
                    arg_print("-h", "--help", "None", "Print help message and exit");
                    arg_print("  ", "check", "None", "Check for upcomming appointments and show notification if some are upcomming");
                    arg_print("-n", "--next", "usize", "Print next n appointments");
                    arg_print("-p", "--prev", "usize", "Print previous n appointments");
                    arg_print("-gda", "--grepdate", "String", "Search for all dates with a specific pattern\n     e.g. '17-' for all appointments on 17th");
                    arg_print(
                        "-gde",
                        "--grepdes",
                        "String",
                        "Regex pattern to search in date description",
                    );
                    arg_print(
                        "-l",
                        "-last_added",
                        "usize",
                        "Show the n last added appointments",
                    );
                    arg_print(
                        "-d",
                        "--delete",
                        "String",
                        "Provide an id of the appointment that should be removed",
                    );
                    arg_print("-a", "--add", "String", "Add new appointment in the form '02-02-2022-02:00,2.0,1.5,description of appointment'");
                    arg_print(
                        "-m",
                        "--month",
                        "u32, i32",
                        "Show calendar of specified month in given year like 02 2022",
                    )
                }
                _ => eprintln!("Invalid command '{}'", &args[1]),
            }
        }
        3 => {
            let cmd = &args[1];
            let argument = &args[2];
            match &cmd[..] {
                "-n" | "--next" => {
                    get_next_n(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                        "forward",
                    );
                }
                "-p" | "--prev" => {
                    get_next_n(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                        "reverse",
                    );
                }
                "-gda" | "--grepdate" => {
                    grep_by_date(argument, &file_content);
                }
                "-gde" | "--grepdes" => {
                    grep_by_description(argument, &file_content);
                }
                "-l" | "--last_add" => {
                    last_added(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                    );
                }
                "-d" | "--delete" => {
                    remove_entry(argument, &DATE_FILE_PATH);
                }
                "-a" | "--add_date" => {
                    let re = Regex::new(
                        r"[0-9]{2}-[0-9]{2}-[0-9]{4}-[0-9]{2}:[0-9]{2},[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+),[+-]?([0-9]+([.][0-9]*)?|[.][0-9]+),",
                    )
                    .expect("Invalid regex pattern");
                    if re.is_match(argument) {
                        append_file(&DATE_FILE_PATH, argument);
                    } else {
                        eprintln!("Invalid argument '{}' has to have the following pattern '02-02-2022-02:00,2.0,1.5,description of appointment'", argument);
                    }
                }

                _ => eprintln!("Invalid command '{} {}'", cmd, argument),
            }
        }
        4 => {
            let cmd = &args[1];
            match &cmd[..] {
                "-m" | "--month" => {
                    let argument1 = &args[2]
                        .parse::<u32>()
                        .expect("Couldn't convert 1st argument to u32");
                    let argument2 = &args[3]
                        .parse::<i32>()
                        .expect("Couldn't convert 2nd argument to u32");
                    let first_day_month = NaiveDate::from_ymd_opt(*argument2, *argument1, 1)
                        .expect("First day of the month couldn't be calculated")
                        .weekday()
                        .number_from_monday();
                    let event_dates: Vec<u32> =
                        appointment_check(&file_content, &*argument1, &*argument2);
                    let month_hash = month_len(&*argument2, &*argument1);
                    month_view(&month_hash, &0, &event_dates, &first_day_month);
                    println!("Print the calender of the specific month")
                }
                _ => eprintln!("Invalid command '{} {} {}'", cmd, &args[2], &args[3]),
            }
        }
        _ => eprintln!("Invalid command '{} {} {}'", &args[1], &args[2], &args[3]),
    }
    Ok(())
}

fn arg_print(arg_short: &str, arg_long: &str, dtype: &str, msg: &str) {
    println!("{} | {}   <{}>\n     {}\n", arg_short, arg_long, dtype, msg)
}
