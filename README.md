# Cicada  🦗
***C***ommand l***I***ne ***CA***len***DA***r

Simple and lightweight command line calendar 

## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)
Change the [directory path](https://github.com/gwirn/Cicada/blob/f799b56a5ea189c84c86e71a3e53ca4b7ab51e0b/src/main.rs#L20) to an absolute path so calender can be run from everywhere.

Run `cargo build --release` to create a binary of the program.

## Options and functionality
The default view of the calendar is just the current month.
The current day is highlighted in bold green.
A red ∆ on the right side of the day indicates an appointment.

```
-h | --help   <None>
     Print help message and exit

   | check   <None>
     Check for upcomming appointments and show notification if some are upcomming

-n | --next   <usize>
     Print next n appointments

-p | --prev   <usize>
     Print previous n appointments

-gda | --grepdate   <String>
     Search for all dates with a specific pattern
     e.g. '17-' for all appointments on 17th

-gde | --grepdes   <String>
     Regex pattern to search in date description

-l | -last_added   <usize>
     Show the n last added appointments

-d | --delete   <String>
     Provide an id of the appointment that should be removed

-a | --add   <String>
     Add new appointment in the form '02-02-2022-02:00,2.0,1.5,description of appointment'

-m | --month   <u32, i32>
     Show calendar of specified month in given year like 02 2022
```

To check your calendar continously you can set a job in [crontab](https://www.man7.org/linux/man-pages/man1/crontab.1.html) that runs `calendar check` every n minutes to get alerts for upcoming events.
