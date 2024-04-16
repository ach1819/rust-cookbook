use std::{error::Error, io::{self, Write}};
use chrono::Local;
use env_logger::{Builder, Target};
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use log4rs::{
    append::file::FileAppender,
    config::{runtime::{ConfigError, ConfigErrors}, Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
    
};
use syslog::Facility;

fn main() {
    // should run alone
    // log_message_with_custom_logger().expect("Error with custom loggin");
    // return;
    //

    // should run alone too
    //log_to_unix_syslog().expect("Error in log to the unix syslog");
    //return;

    // should run alone
    //enable_log_levels_by_module();
    //return;

    // should run alone
    //custom_env_variable_setup_logging();
    //return;

    // should run alone
    //include_timestamp_in_log_message();
    //return;

    // should run alone
    //log_message_to_custom_location();
    //return;

    let use_stdout = std::env::var("USE_STDOUT")
        .map(|v| v == "true")
        .unwrap_or(false);

    if use_stdout {
        Builder::new().target(Target::Stdout).init();
    } else {
        env_logger::init();
    }
    log_debug_message();
    log_error_message();
    log_stdout_instead_stderr();
}

fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn execute_query_err(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn log_debug_message() {
    execute_query("DROP TABLE students");
}

fn log_error_message() {
    let response = execute_query_err("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}

fn log_stdout_instead_stderr() {
    log::error!("This error has been printed to Stdout");
}
static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn log_message_with_custom_logger() -> Result<(), SetLoggerError> {
    log::set_logger(&CONSOLE_LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("hello log");
    log::warn!("warning");
    log::error!("oops");
    Ok(())
}

fn log_to_unix_syslog() -> Result<(), syslog::Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;

    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

mod foo {
    mod bar {
        pub fn run() {
            log::warn!("[bar] warn");
            log::info!("[bar] info");
            log::debug!("[bar] debug");
        }
    }

    pub fn run() {
        log::warn!("[foo] warn");
        log::info!("[foo] info");
        log::debug!("[foo] debug");
        bar::run();
    }
}
fn enable_log_levels_by_module() {
    env_logger::init();
    log::warn!("[root] warn");
    log::info!("[root] info");
    log::debug!("[root] debug");
    foo::run();
}

fn custom_env_variable_setup_logging() {
    Builder::new()
        .parse_env(&std::env::var("MY_APP_LOG").unwrap_or_default())
        .init();

    log::info!("informational message");
    log::warn!("warning message");
    log::error!("this is an error {}", "message");
}

fn include_timestamp_in_log_message() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::warn!("warn");
    log::info!("info");
    log::debug!("debug");
}

fn log_message_to_custom_location() -> Result<(), io::Error>{
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("Hello, world!");

    Ok(())
}
