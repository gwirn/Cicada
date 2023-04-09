use chrono::Datelike;
use chrono::{Local, NaiveDate};
use std::collections::HashMap;
fn main() -> std::io::Result<()> {
    let event_dates: Vec<u8> = vec![8, 12];
    let now = Local::now().date_naive();
    let cur_day = now.day() as u8;
    let cur_month = now.month() as u8;

    let first_day_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .expect("First day of the month couldn't be calculated")
        .weekday()
        .number_from_monday() as u8;

    let month_hash = month_hashmap();
    match month_hash.get(&cur_month) {
        Some(&month) => month_view(&month, &cur_day, &event_dates, &first_day_month),
        _ => println!("Couldn't retrieve current month"),
    }
    Ok(())
}

fn month_hashmap() -> HashMap<u8, u8> {
    // could be better done to test for feb 29
    let mut month_lenght = HashMap::new();
    month_lenght.insert(1u8, 32);
    month_lenght.insert(2u8, 29);
    month_lenght.insert(3u8, 32);
    month_lenght.insert(4u8, 31);
    month_lenght.insert(5u8, 32);
    month_lenght.insert(6u8, 31);
    month_lenght.insert(7u8, 32);
    month_lenght.insert(8u8, 32);
    month_lenght.insert(9u8, 31);
    month_lenght.insert(10u8, 32);
    month_lenght.insert(11u8, 31);
    month_lenght.insert(12u8, 32);
    month_lenght
}

fn month_view(month: &u8, cur_day: &u8, event: &Vec<u8>, fdm: &u8) {
    println!("\x1b[4;1m Mo  Tu  We  Th  Fr  Sa  Su\x1b[0m");
    let skip: u8 = fdm - 2;

    for i in 0..(month + skip) {
        if i > skip {
            let mut add = "";
            if (i + 1) % 7 == 0 && i > 0 {
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
                color_num,
                i - skip,
                event_sign,
                add,
            );
        } else {
            print!("    ")
        }
    }
    println!("");
}
