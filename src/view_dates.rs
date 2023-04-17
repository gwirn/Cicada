use crate::date_utils::{argsort, saved_data_header, time_date_lef, SavedDate};
use regex::Regex;
pub fn last_added(n: usize, data_vect: &Vec<SavedDate>) {
    // get the last added dates
    let id_dates: Vec<i64> = data_vect.iter().map(|x| x.id).collect::<Vec<i64>>();
    let id_order = argsort(&id_dates);
    saved_data_header();
    let mut counter = 0;
    for i in id_order {
        if &counter < &n {
            println!("{}", &data_vect[i]);
            counter += 1;
        }
    }
}

pub fn get_next_n(n: usize, data_vect: &Vec<SavedDate>, direction: &str) {
    // get the next n dates or the n dates that were already over
    let due_dates: Vec<i64> = data_vect
        .iter()
        .map(|x| time_date_lef(&x.due))
        .collect::<Vec<i64>>();
    let mut date_order = argsort(&due_dates);

    saved_data_header();
    let mut counter = 0;
    match direction {
        "forward" => {
            for i in date_order {
                if &due_dates[i] >= &0 && &counter < &n {
                    println!("{}", &data_vect[i]);
                    counter += 1;
                }
            }
        }
        "reverse" => {
            date_order.reverse();
            for i in date_order {
                if &due_dates[i] <= &0 && &counter < &n {
                    println!("{}", &data_vect[i]);
                    counter += 1;
                }
            }
        }
        _ => {
            eprintln!("Invalid input for get_next_n")
        }
    }
}

pub fn grep_by_date(search_date: &str, data_vect: &Vec<SavedDate>) {
    // search for date with 17- all dates with 17th or -05- all dates in May
    saved_data_header();
    for i in data_vect {
        if i.due.contains(&search_date) {
            println!("{}", i);
        }
    }
}

pub fn grep_by_description(search_pattern: &str, data_vect: &Vec<SavedDate>) {
    // search with regex pattern in description and print if match was found
    let search_pattern = format!(r"{}", search_pattern);
    let re = Regex::new(&search_pattern).expect("Invalid regex pattern");
    saved_data_header();
    for i in data_vect {
        if re.is_match(&i.description) {
            println!("{}", i)
        }
    }
}
