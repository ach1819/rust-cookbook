use std::{thread, time::{Duration, Instant}};

use chrono::{DateTime, Utc};

fn main() {
    measure_elapsed_time_between();
    perform_checked_date_time_calculations();
}

fn expensive_function() {
    thread::sleep(Duration::from_secs(1))
}

fn measure_elapsed_time_between() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is {:?}", duration);
}

fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(chrono::Duration::days(1))
}

fn perform_checked_date_time_calculations() {
    println!("\nperform_checked_date_time_calculations - starts");
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now.checked_add_signed(chrono::Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(chrono::Duration::weeks(1)))
        .and_then(day_earlier);

    match almost_three_weeks_from_now {
        Some(x) => println!("{}", x),
        None => eprintln!("Almost three weeks from now overflows!")
    }

    match now.checked_add_signed(chrono::Duration::max_value()) {
        Some(x) => println!("{}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
    println!("perform_checked_date_time_calculations - OK");
}
