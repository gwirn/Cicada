use chrono::Datelike;
use chrono::{Local, NaiveDate};
use std::collections::HashMap;
fn main() {
    let long_month: Vec<u8> = (1..32).collect();
    let short_month: Vec<u8> = (1..31).collect();
    let feb_month: Vec<u8> = (1..30).collect();

    let event_dates: Vec<u8> = vec![8, 12];
    let now = Local::now().date_naive();
    let cur_day = now.day() as u8;
    let cur_month = now.month() as u8;

    let first_day_month = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
        .expect("First day of the month couldn't be calculated")
        .weekday()
        .number_from_monday() as u8;

    let month_hash = month_hashmap(&feb_month, &short_month, &long_month);
    match month_hash.get(&cur_month) {
        Some(&month) => month_view(&month, &cur_day, &event_dates, &first_day_month),
        _ => println!("Couldn't retrieve current month"),
    }
}

fn month_hashmap<'a>(
    febm: &'a Vec<u8>,
    shortm: &'a Vec<u8>,
    longm: &'a Vec<u8>,
) -> HashMap<u8, &'a Vec<u8>> {
    // could be better done to test for feb 29
    let mut month_lenght = HashMap::new();
    month_lenght.insert(1u8, longm);
    month_lenght.insert(2u8, febm);
    month_lenght.insert(3u8, longm);
    month_lenght.insert(4u8, shortm);
    month_lenght.insert(5u8, longm);
    month_lenght.insert(6u8, shortm);
    month_lenght.insert(7u8, longm);
    month_lenght.insert(8u8, longm);
    month_lenght.insert(9u8, shortm);
    month_lenght.insert(10u8, longm);
    month_lenght.insert(11u8, shortm);
    month_lenght.insert(12u8, longm);
    month_lenght
}

fn month_view(month: &Vec<u8>, cur_day: &u8, event: &Vec<u8>, fdm: &u8) {
    println!("\x1b[4;1m Mo  Tu  We  Th  Fr  Sa  Su\x1b[0m");

    let mut combined_month: Vec<u8> = vec![0; (fdm - 1).into()];
    combined_month.extend(month);

    for (ci, i) in combined_month.iter().enumerate() {
        let mut add = "";
        if (ci + 1) % &7 == 0 && i > &0 {
            add = "\n";
        }
        let mut color_num = "0";
        if i == cur_day {
            color_num = "32;1";
        }
        let mut event_sign = " ";
        if event.contains(i) {
            event_sign = "\x1b[91m∆\x1b[0m";
        }

        let mut date_to_print: String = " ".to_string();
        if i != &0 {
            date_to_print = i.to_string();
        }
        print!(
            "\x1b[{0}m{1:>3}\x1b[0m{2}{3}",
            color_num, date_to_print, event_sign, add,
        );
    }
    println!("");
}
