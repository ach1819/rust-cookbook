use std::{
    thread,
    time::{Duration, Instant},
};

use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, ParseError, Timelike, Utc};

fn main() {
    measure_elapsed_time_between();
    perform_checked_date_time_calculations();
    convert_local_time_to_timezone();
    examine_date_time();
    display_formatted_date_time();
    parse_string_into_datetime().expect("Error parsing dates");
}

fn expensive_function() {
    thread::sleep(Duration::from_secs(1))
}

fn measure_elapsed_time_between() {
    println!("\nmeasure_elapsed_time_between - starts");
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is {:?}", duration);
    println!("measure_elapsed_time_between - OK");
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(chrono::Duration::days(1))
}

fn perform_checked_date_time_calculations() {
    println!("\nperform_checked_date_time_calculations - starts");
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(chrono::Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(chrono::Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(chrono::Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
    println!("perform_checked_date_time_calculations - OK");
}

fn convert_local_time_to_timezone() {
    println!("\nconvert_local_time_to_timezone - starts");
    let local_time = Local::now();
    let utc_time = chrono::Utc::now();
    let china_timezone = FixedOffset::east_opt(8 * 3600);
    let rio_timezone = FixedOffset::west_opt(2 * 3600);
    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone.unwrap())
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone.unwrap())
    );
    println!("convert_local_time_to_timezone - OK");
}

fn examine_date_time() {
    println!("\nexamine_date_time - starts");
    let now = chrono::Utc::now();

    let (is_pm, hour) = now.hour12();

    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );

    println!(
        "And there have been {} seconds since midnight",
        now.num_seconds_from_midnight()
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the Common Era began {} days ago",
        now.num_days_from_ce()
    );
    println!("examine_date_time - OK");
}

fn display_formatted_date_time() {
    println!("\ndisplay_formatted_date_time - starts");

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );

    println!("display_formatted_date_time - OK");
}

fn parse_string_into_datetime() -> Result<(), ParseError> {
    println!("\nparse_string_into_datetime - starts");

    let rfc2822 = chrono::DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339 = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom = DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom);



    let time_only = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let no_timezone = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", no_timezone);

    println!("parse_string_into_datetime - OK");
    Ok(())
}
