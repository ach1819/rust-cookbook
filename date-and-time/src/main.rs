use std::{
    thread,
    time::{Duration, Instant},
};

use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};

fn main() {
    measure_elapsed_time_between();
    perform_checked_date_time_calculations();
    convert_local_time_to_timezone();
    examine_date_time();
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
        if is_pm { "PM"} else { "AM" }
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
