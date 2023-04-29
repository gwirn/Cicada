use crate::encrypt::{gen_key_pwd, get_pwd_file, read_bin, write_bin};
use crate::{PWD_LOC, SALT_LOC};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use notify_rust::Notification;
use orion::aead;
use std::fmt;
use std::path::Path;

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
                .icon("Calendar")
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
    let mut dec_data = if !Path::new(&filepath)
        .try_exists()
        .expect("Couldn't check date.file for existance")
        || !Path::new(&SALT_LOC)
            .try_exists()
            .expect("Couldn't check .salt for existance")
    {
        let dec_d: Vec<u8> = Vec::new();
        dec_d
    } else {
        let (_, key) = gen_key_pwd(
            &get_pwd_file(&PWD_LOC),
            orion::kdf::Salt::from_slice(read_bin(&SALT_LOC).as_ref())
                .expect("Couldn't retrieve salt from file"),
        );
        let dec_d = aead::open(&key, &read_bin(&filepath)).expect("Couldn't decipher the file");
        dec_d
    };
    // time stamp of creation == id
    let entry_ts: i64 = Local::now().timestamp() - 1681429910;
    let entry_id = entry_ts.to_string();
    let file_line = format!("{},{}-X-", &entry_id, line);

    for i in file_line.as_bytes() {
        dec_data.push(*i);
    }
    let (salt, key) = gen_key_pwd(&get_pwd_file(&PWD_LOC), orion::kdf::Salt::default());
    write_bin(&salt.as_ref().to_vec(), &SALT_LOC);
    let cipher_text = aead::seal(&key, &dec_data).expect("Couldn't encrypt the data");
    write_bin(&cipher_text, &filepath);
}

pub fn read_file(filepath: &str) -> Vec<SavedDate> {
    // read date.file content into vector of type SavedDate
    let mut date_vec: Vec<SavedDate> = Vec::new();
    if std::path::Path::new(&filepath)
        .try_exists()
        .expect("Couldn't check if date.file exists")
        && Path::new(&SALT_LOC)
            .try_exists()
            .expect("Couldn't check salt file for existance")
    {
        let (_, key) = gen_key_pwd(
            &get_pwd_file(&PWD_LOC),
            orion::kdf::Salt::from_slice(read_bin(&SALT_LOC).as_ref())
                .expect("Couldn't retrieve salt from file"),
        );
        let dec_data = aead::open(&key, &read_bin(&filepath)).expect("Couldn't decipher the file");
        for line in std::str::from_utf8(&dec_data)
            .expect("Failed to convert decoded file to str")
            .split("-X-")
            .collect::<Vec<_>>()
        {
            if &line.len() > &0 {
                let words_split: Vec<&str> = line.split(",").collect();
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
        }
    }
    date_vec
}

pub fn remove_entry(rm_id: &str, file_path: &str) {
    // remove entry by id
    let file_content = read_file(file_path);
    let mut removed = Vec::new();
    for i in file_content.iter() {
        if i.id.to_string() != rm_id {
            let acc_line = format!(
                "{},{},{},{},{}-X-",
                i.id, i.due, i.length, i.alert_time_h, i.description
            );
            for k in acc_line.as_bytes() {
                removed.push(k.to_owned());
            }
        }
    }
    let (salt, key) = gen_key_pwd(&get_pwd_file(&PWD_LOC), orion::kdf::Salt::default());
    write_bin(&salt.as_ref().to_vec(), &SALT_LOC);
    let cipher_text = aead::seal(&key, &removed).expect("Couldn't encrypt the data");
    write_bin(&cipher_text, &file_path);
}
