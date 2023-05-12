# Cicada  🦗
***C***ommand l***I***ne ***CA***len***DA***r

Simple command line calendar with encrypted storage file

Shows appointments and can alert when appointments are due.
```
            May 2023
 Mo  Tu  We  Th  Fr  Sa  Su
  1   2   3   4∆  5   6   7 
  8   9  10  11  12  13  14 
 15  16  17  18  19  20  21 
 22  23  24  25  26  27  28 
 29  30  31 
```
## Installation
[Install rust and cargo](https://www.rust-lang.org/tools/install)

Change the directory path to an absolute path so calender can be run from everywhere for these 3 files:
* The date storage file
https://github.com/gwirn/Cicada/blob/f1811ff85513f3fa2fdfc6f9d35b60199185e90f/src/main.rs#L13
* The file containing the password used for de-/encrypting the date storage file
https://github.com/gwirn/Cicada/blob/f1811ff85513f3fa2fdfc6f9d35b60199185e90f/src/main.rs#L15
* The file containing the salt for the password
https://github.com/gwirn/Cicada/blob/f1811ff85513f3fa2fdfc6f9d35b60199185e90f/src/main.rs#L14

Run `cargo build --release` to create a binary of the program.

If you want a faster version go back to this ab2af0cb14e9f06685caa38b1500de6ce6059b2f commit. The file ecryption makes it slower.

## Options and functionality
The default mode of the calendar is just the current month.
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
