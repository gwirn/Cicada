use chrono::{DateTime, Local, NaiveDateTime, Utc};
use notify_rust::Notification;
use std::fmt;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
pub struct SavedDate {
    pub id: i64,
    pub due: String,
    pub length: f32,
    pub alert_time_h: f32,
    pub description: String,
}

impl fmt::Display for SavedDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|{:>12}|{:>16}|{:>12}|{:>14}|{:>11}|",
            self.id, self.due, self.length, self.alert_time_h, self.description
        )
    }
}

pub fn offset_and_time() -> (i64, DateTime<Local>) {
    // get how many h the local time is offset ti utc
    let cur_time = Local::now();
    let local_off_minutes: i64 = (&cur_time.offset().local_minus_utc() / 60) as i64;
    (local_off_minutes, cur_time)
}

pub fn time_date_lef(in_line: &str) -> i64 {
    // convert time string to time passed or until date in minutes
    let (local_off_minutes, cur_time) = offset_and_time();
    let time_test = NaiveDateTime::parse_from_str(&in_line, "%d-%m-%Y-%H:%M")
        .expect("Couldn't parse date from date.file");
    let time_fmt: DateTime<Local> = DateTime::<Utc>::from_utc(time_test, Utc).into();
    let time_passed = time_fmt.signed_duration_since(cur_time).num_minutes() - &local_off_minutes;
    time_passed
}
pub fn check_dates(data_vect: &Vec<SavedDate>) {
    // check if a date alert has to be shown
    for i in data_vect {
        let time_left = time_date_lef(&i.due) as f32;
        if time_left > 0.0 && time_left <= (i.alert_time_h * (60 as f32)) {
            let msg_string = format!(
                "Appointment at: {}\nDuration: {} [h]\n{}",
                i.due, i.length, i.description
            );
            Notification::new()
                .summary("Calendar")
                .body(&msg_string)
                .timeout(0)
                .show()
                .expect("Couldn't display notification");
        }
    }
}
pub fn argsort<T: Ord>(to_sort: &[T]) -> Vec<usize> {
    // get the indices that would sort a vector
    let mut inds = (0..to_sort.len()).collect::<Vec<_>>();
    inds.sort_by_key(|&i| &to_sort[i]);
    inds
}

pub fn saved_data_header() {
    // print header when printing dates
    println!(
        "\x1b[4;1m|{:>12}|{:>16}|{:>12}|{:>14}|{:>11}|\x1b[0m",
        "Id", "Due", "Duration [h]", "AlertBefor [h]", "Description"
    )
}
pub fn append_file(filepath: &str, line: &str) {
    // time stamp of creation == id
    let entry_ts: i64 = Local::now().timestamp() - 1681429910;
    let entry_id = entry_ts.to_string();
    let mut out = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filepath)
        .expect("Couldn't open file to write to");
    let file_line = format!("{},{}", &entry_id, line);
    writeln!(out, "{}", file_line).expect("Couldn't write to file");
}
pub fn read_file(filepath: &str) -> Vec<SavedDate> {
    // read date.file content into vector of type SavedDate
    let mut date_vec: Vec<SavedDate> = Vec::new();
    for line in
        std::io::BufReader::new(fs::File::open(filepath).expect("Failed to open date.file")).lines()
    {
        let words = line.unwrap();
        let words_split: Vec<&str> = words.split(",").collect();
        date_vec.push(SavedDate {
            id: words_split[0]
                .parse::<i64>()
                .expect("Couldn't convert id to int"),
            due: String::from(words_split[1]),
            length: words_split[2]
                .parse::<f32>()
                .expect("Couldn't convert length to f32"),
            alert_time_h: words_split[3]
                .parse::<f32>()
                .expect("Couldn't convert alert time to f32"),
            description: String::from(words_split[4]),
        });
    }
    date_vec
}
