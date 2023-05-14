use crate::encrypt::{gen_key_pwd, get_pwd_file, read_bin, write_bin};
use crate::{ALERT_LOC, PWD_LOC, SALT_LOC};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use notify_rust::Notification;
use orion::aead;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct SavedDate {
    // the id of the date
    pub id: i64,
    // when the date is happening (date and time)
    pub due: String,
    // how long it takes
    pub length: f32,
    // how much earlier one should be alerted
    pub alert_time_h: f32,
    // description of the date
    pub description: String,
}

impl fmt::Display for SavedDate {
    /// Prints a saved date in a nice form
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|{:>12}|{:>16}|{:>12}|{:>14}|{:>11}|",
            self.id, self.due, self.length, self.alert_time_h, self.description
        )
    }
}

/// Get the offset of the local time to utc time
pub fn offset_and_time() -> (i64, DateTime<Local>) {
    let cur_time = Local::now();
    let local_off_minutes: i64 = (&cur_time.offset().local_minus_utc() / 60) as i64;
    (local_off_minutes, cur_time)
}

/// Convert time string to time passed or until date in minutes
///
/// # Arguments
///
/// * `in_line` - a date as str that can be converted to %d-%m-%Y-%H:%M
pub fn time_date_lef(in_line: &str) -> i64 {
    // the time offset
    let (local_off_minutes, cur_time) = offset_and_time();
    // convert in_line to date
    let time_test = NaiveDateTime::parse_from_str(&in_line, "%d-%m-%Y-%H:%M")
        .expect("Couldn't parse date from date.file");
    let time_fmt: DateTime<Local> = DateTime::<Utc>::from_utc(time_test, Utc).into();
    // calc the time difference
    let time_passed = time_fmt.signed_duration_since(cur_time).num_minutes() - &local_off_minutes;
    time_passed
}

/// Read file content into a vector
/// # Argumnets
///
/// * `file_path` - path to the file to be read
fn read_plain_file(file_path: &str) -> Vec<String> {
    let mut content = Vec::<String>::new();
    if std::path::Path::new(file_path)
        .try_exists()
        .expect("Couldn't check file for existance")
    {
        let file = File::open(file_path).expect("Couldn't read file");
        let buffer = std::io::BufReader::new(file).lines();
        for i in buffer {
            let line = i.expect("Couldn't read line");
            content.push(line);
        }
    }
    content
}

/// Check if a date alert has to be shown
///
/// # Arguments
///
/// * `date_vect` - vector containing all dates
pub fn check_dates(data_vect: &Vec<SavedDate>) {
    let mut checked = read_plain_file(ALERT_LOC);
    let mut change_happened = false;
    for i in data_vect {
        // how much time is left until the date and alert if it's <= alert time for this date
        let time_left = time_date_lef(&i.due) as f32;
        // is date already over
        if time_left > 0.0 {
            // is date in alert time range and was the alert not already shown
            if time_left <= (i.alert_time_h * (60 as f32)) && !checked.contains(&i.id.to_string()) {
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
                checked.push(i.id.to_string());
                change_happened = true;
            }
        } else {
            // remove id from already shown alert file if the date is already over
            if let Some(index) = checked.iter().position(|r| r == &i.id.to_string()) {
                checked.remove(index);
            }
        }
    }
    // write a new already alerted dates file if changes happened
    if change_happened {
        let mut file_to_write = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(ALERT_LOC)
            .expect("Couldn't write to file");
        for i in checked.iter() {
            writeln!(&mut file_to_write, "{}", i).expect("Couldn't write line to file");
        }
    }
}

/// Get the indices that would sort a vector
///
/// # Arguments
///
/// * `to_sort` - a vector to be sorted
pub fn argsort<T: Ord>(to_sort: &[T]) -> Vec<usize> {
    let mut inds = (0..to_sort.len()).collect::<Vec<_>>();
    inds.sort_by_key(|&i| &to_sort[i]);
    inds
}

/// Print header when printing dates
pub fn saved_data_header() {
    println!(
        "\x1b[4;1m|{:>12}|{:>16}|{:>12}|{:>14}|{:>11}|\x1b[0m",
        "Id", "Due", "Duration [h]", "AlertBefor [h]", "Description"
    )
}

/// Add a new date to the file
///
/// # Arguments
///
/// * `file_path` - the path to the file where a date should be added
/// * `line` - the date that should be added
pub fn append_file(filepath: &str, line: &str) {
    // if date.file and salt file exits - read them or create them if they don't exist
    let mut dec_data = if !Path::new(&filepath)
        .try_exists()
        .expect("Couldn't check date.file for existence")
        || !Path::new(&SALT_LOC)
            .try_exists()
            .expect("Couldn't check .salt for existence")
    {
        let dec_d: Vec<u8> = Vec::new();
        dec_d
    } else {
        // get salt and key for decryption
        let (_, key) = gen_key_pwd(
            &get_pwd_file(&PWD_LOC),
            orion::kdf::Salt::from_slice(read_bin(&SALT_LOC).as_ref())
                .expect("Couldn't retrieve salt from file"),
        );
        // decrypt file content
        let dec_d = aead::open(&key, &read_bin(&filepath)).expect("Couldn't decipher the file");
        dec_d
    };
    // time stamp of creation == id
    let entry_ts: i64 = Local::now().timestamp() - 1681429910;
    let entry_id = entry_ts.to_string();
    // create line corresponding to date
    let file_line = format!("{},{}-X-", &entry_id, line);
    // append each byte of the new date to the file content
    for i in file_line.as_bytes() {
        dec_data.push(*i);
    }
    // write everything encrypted to the file
    let (salt, key) = gen_key_pwd(&get_pwd_file(&PWD_LOC), orion::kdf::Salt::default());
    write_bin(&salt.as_ref().to_vec(), &SALT_LOC);
    let cipher_text = aead::seal(&key, &dec_data).expect("Couldn't encrypt the data");
    write_bin(&cipher_text, &filepath);
}

/// Read date.file content into vector of type SavedDate
///
/// # Arguments
///
/// * `filepath` - file path to the date.file
pub fn read_file(filepath: &str) -> Vec<SavedDate> {
    let mut date_vec: Vec<SavedDate> = Vec::new();
    // check if file exists and if so parse it's content to a vector <SavedDate> or return an empty
    // one
    if std::path::Path::new(&filepath)
        .try_exists()
        .expect("Couldn't check if date.file exists")
        && Path::new(&SALT_LOC)
            .try_exists()
            .expect("Couldn't check salt file for existence")
    {
        // get salt and key for decryption
        let (_, key) = gen_key_pwd(
            &get_pwd_file(&PWD_LOC),
            orion::kdf::Salt::from_slice(read_bin(&SALT_LOC).as_ref())
                .expect("Couldn't retrieve salt from file"),
        );
        // decrypt file content
        let dec_data = aead::open(&key, &read_bin(&filepath)).expect("Couldn't decipher the file");
        // split file content into separate dates and add them to the date_vec in the right format
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

/// Remove date from file by it's id
///
/// # Arguments
///
/// * `rm_id` - the id of the date to be removed
/// * `file_path` - the path to the date.file
pub fn remove_entry(rm_id: &str, file_path: &str) {
    let file_content = read_file(file_path);
    let mut removed = Vec::new();
    // iterate over the whole file content and only add it to be readded if it's not the date with
    // the id to be removed
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
    // encrypt data and write to file
    let (salt, key) = gen_key_pwd(&get_pwd_file(&PWD_LOC), orion::kdf::Salt::default());
    write_bin(&salt.as_ref().to_vec(), &SALT_LOC);
    let cipher_text = aead::seal(&key, &removed).expect("Couldn't encrypt the data");
    write_bin(&cipher_text, &file_path);
}
