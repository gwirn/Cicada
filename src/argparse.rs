use crate::date_utils::{append_file, check_dates, remove_entry, SavedDate};
use crate::view_dates::{get_next_n, grep_by_date, grep_by_description, last_added};
use crate::view_month::{appointment_check, month_len, month_view};
use crate::DATE_FILE_PATH;
use chrono::Datelike;
use chrono::NaiveDate;
use regex::Regex;
/// print different argument in the same format
///
/// # Arguments
///
/// * `arg_short` - the short form of the argument
/// * `arg_long` - the long form of the argument
/// * `dtype` -  the type of data the argument will be transformed in
/// * `msg` - a help message to understand the usage of the argument
pub fn arg_print(arg_short: &str, arg_long: &str, dtype: &str, msg: &str) {
    println!("{} | {}   <{}>\n     {}\n", arg_short, arg_long, dtype, msg)
}

/// retrieve the supplied arguments and execute the respective commands
///
/// # Arguments
///
/// * `file_content` - the decrypted content of the date.file
/// * `cur_day` - the current day date as u32
/// * `cur_month` - the current month as u32
/// * `cur_year` - the current year as i32
pub fn argparse(file_content: Vec<SavedDate>, cur_day: u32, cur_month: u32, cur_year: i32) {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            let first_day_month = NaiveDate::from_ymd_opt(cur_year, cur_month as u32, 1)
                .expect("First day of the month couldn't be calculated")
                .weekday()
                .number_from_monday();
            let event_dates: Vec<u32> = appointment_check(&file_content, &cur_month, &cur_year);
            let month_hash = month_len(&cur_year, &cur_month);
            month_view(
                &cur_year,
                &cur_month,
                &month_hash,
                &cur_day,
                &event_dates,
                &first_day_month,
            );
        }
        2 => {
            match &args[1][..] {
                // check if an appointment is upcoming
                "check" => {
                    check_dates(&file_content);
                    println!("Checked for upcoming dates")
                }
                "-h" | "--help" => {
                    arg_print("-h", "--help", "None", "Print help message and exit");
                    arg_print("  ", "check", "None", "Check for upcoming appointments and show notification if some are upcoming");
                    arg_print("-n", "--next", "usize", "Print next n appointments");
                    arg_print("-p", "--prev", "usize", "Print previous n appointments");
                    arg_print("-gda", "--grepdate", "String", "Search for all dates with a specific pattern\n     e.g. '4-' for all appointments on 4th");
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
                    arg_print("-a", "--add", "String", "Add new appointment in the form '04-05-2022-02:00,2.0,1.5,description of appointment'\n     This specifies the date-the time, the duration in hours, \n     the number of hours before the event to show an alert and a description\n     The single quotes around the string are needed \n     The date format is %d-%m-%Y-%H:%M");
                    arg_print(
                        "-m",
                        "--month",
                        "u32, i32",
                        "Show calendar of specified month in given year like 5 2022",
                    )
                }
                _ => eprintln!("Invalid command '{}'", &args[1]),
            }
        }
        3 => {
            let cmd = &args[1];
            let argument = &args[2];
            match &cmd[..] {
                // the next n upcoming appointments
                "-n" | "--next" => {
                    get_next_n(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                        "forward",
                    );
                }
                // the previous n appointments
                "-p" | "--prev" => {
                    get_next_n(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                        "reverse",
                    );
                }
                // dates matching time
                "-gda" | "--grepdate" => {
                    grep_by_date(argument, &file_content);
                }
                // dates matching description
                "-gde" | "--grepdes" => {
                    grep_by_description(argument, &file_content);
                }
                // the last n added appointments
                "-l" | "--last_add" => {
                    last_added(
                        argument
                            .parse::<usize>()
                            .expect("Couldn't convert argument to usize"),
                        &file_content,
                    );
                }
                // delete an appointment by id
                "-d" | "--delete" => {
                    remove_entry(argument, &DATE_FILE_PATH);
                }
                // add an appointment
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
            // print month calendar of specific month
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
                    month_view(
                        &argument2,
                        &argument1,
                        &month_hash,
                        &0,
                        &event_dates,
                        &first_day_month,
                    );
                }
                _ => eprintln!("Invalid command '{} {} {}'", cmd, &args[2], &args[3]),
            }
        }
        _ => eprintln!("Invalid command '{} {} {}'", &args[1], &args[2], &args[3]),
    }
}
