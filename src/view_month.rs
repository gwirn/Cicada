use crate::date_utils::SavedDate;
use chrono::NaiveDate;

/// Get number of days of the month
///
/// # Arguments
///
/// * `year` - the year of the month of interest
/// * `month` - the month of interest (no leading 0)
pub fn month_len(year: &i32, month: &u32) -> u32 {
    {
        if *month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(*year, *month + 1, 1)
        }
        .expect("Couldn't retrieve month")
        .signed_duration_since(
            NaiveDate::from_ymd_opt(*year, *month, 1).expect("Couldn't retrieve month"),
        )
        .num_days()
        .try_into()
        .expect("Couldn't convert month to u32")
    }
}

/// Get the name of the month by its number
///
/// # Arguments
///
/// * `month` - the month as uint
fn month_name(month: &u32) -> String {
    match month {
        1 => "January".to_string(),
        2 => "February".to_string(),
        3 => "March".to_string(),
        4 => "April".to_string(),
        5 => "May".to_string(),
        6 => "June".to_string(),
        7 => "July".to_string(),
        8 => "August".to_string(),
        9 => "September".to_string(),
        10 => "October".to_string(),
        11 => "November".to_string(),
        12 => "December".to_string(),
        _ => "Invalid month".to_string(),
    }
}

/// View a month in a nice form and indicators for scheduled dates
///
/// # Arguments
///
/// * `year` - the year of the month of interest
/// * `month` - the month of interest
/// * `month_len` - the length of the month
/// * `cur_day` - the current day
/// * `event` - dates where a date is scheduled
/// * `fdm` - number of days between Monday and the first day of the month
pub fn month_view(
    year: &i32,
    month: &u32,
    month_len: &u32,
    cur_day: &u32,
    event: &Vec<u32>,
    fdm: &u32,
) {
    println!("{:>15} {}", month_name(month), &year);
    println!("\x1b[4;1m Mo  Tu  We  Th  Fr  Sa  Su\x1b[0m");
    // get number of empty fields to print when 1st is not a Monday
    let skip: u32 = *fdm - 1;
    let month_len = month_len + 1;

    for i in 1..(month_len + skip) {
        if i > skip {
            let mut add = "";
            if i % 7 == 0 && i > 0 {
                add = "\n";
            }
            let i_skip = &i - skip;
            let mut color_num = "0";
            if i_skip == *cur_day {
                color_num = "32;1";
            }
            let mut event_sign = " ";
            if event.contains(&i_skip) {
                event_sign = "\x1b[91m∆\x1b[0m";
            }

            print!(
                "\x1b[{0}m{1:>3}\x1b[0m{2}{3}",
                color_num, &i_skip, event_sign, add,
            );
        } else {
            print!("    ");
        }
    }
    println!();
}

/// Check for a given month in a year what dates are in this month
///
/// # Arguments
///
/// * `data_vect` - vector containing all dates
/// * `month` - the moth of interest (no leading 0)
/// * `year` - the year of the month of interest
pub fn appointment_check(data_vect: &Vec<SavedDate>, month: &u32, year: &i32) -> Vec<u32> {
    let mut appointments: Vec<u32> = Vec::new();
    for i in data_vect {
        let i_split = i.due.split('-').collect::<Vec<&str>>();
        if i_split[1]
            .parse::<u32>()
            .expect("Couldn't convert month from date.file to u32")
            == *month
            && i_split[2]
                .parse::<i32>()
                .expect("Couldn't convert month from date.file to i32")
                == *year
        {
            appointments.push(
                i_split[0]
                    .parse::<u32>()
                    .expect("Couldn't convert day from date.file to u32"),
            )
        }
    }
    appointments
}
