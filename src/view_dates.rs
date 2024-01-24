use crate::date_utils::{argsort, saved_data_header, time_date_lef, SavedDate};
use regex::Regex;
/// Get the last n added dates
///
/// # Arguments
///
/// * `n` - how many of the last added dates should be shown
/// * `date_vect` - all dates
pub fn last_added(n: usize, data_vect: &Vec<SavedDate>) {
    // get the last added dates
    let id_dates: Vec<i64> = data_vect.iter().map(|x| x.id).collect::<Vec<i64>>();
    let mut id_order = argsort(&id_dates);
    id_order.reverse();
    saved_data_header();
    let mut counter = 0;
    for i in id_order {
        if &counter < &n {
            println!("{}", &data_vect[i]);
            counter += 1;
        }
    }
}

/// Get the next n dates or the n dates that were already over
///
/// # Arguments
///
/// * `n` - how many dates should be shown
/// * `date_vect` - all dates
/// * `direction` - forward for upcoming dates and reverse for passed dates
pub fn get_next_n(n: usize, data_vect: &Vec<SavedDate>, direction: &str) {
    // get when date are
    let due_dates: Vec<i64> = data_vect
        .iter()
        .map(|x| time_date_lef(&x.due))
        .collect::<Vec<i64>>();
    // order all the dates chronologically
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

/// Search for dates containing specific time format - e.g. with 17- all dates with 17th or -05- all dates in May
///
/// # Arguments
///
/// * `search_date` - the pattern that should be looked for
/// * `data_vect` - vector containing all dates
pub fn grep_by_date(search_date: &str, data_vect: &Vec<SavedDate>) {
    saved_data_header();
    for i in data_vect {
        if i.due.contains(search_date) {
            println!("{}", i);
        }
    }
}

/// Search with regex pattern in description and print if match was found
///
/// # Arguments
///
/// * `search_pattern` - regex search pattern to be searched in the date description
/// * `data_vect` - vector containing all dates
pub fn grep_by_description(search_pattern: &str, data_vect: &Vec<SavedDate>) {
    let search_pattern = search_pattern.to_string();
    let re = Regex::new(&search_pattern).expect("Invalid regex pattern");
    saved_data_header();
    for i in data_vect {
        if re.is_match(&i.description) {
            println!("{}", i)
        }
    }
}
