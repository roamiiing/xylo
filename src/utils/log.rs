use chrono::Utc;
use colored::{ColoredString, Colorize};

fn base_log(label: ColoredString, msg: String) {
    let time = Utc::now();
    let formatted_time = format!("{}", time.format("%H:%M:%S")).dimmed();

    println!("{formatted_time} [{label}] {msg}");
}

pub fn info<L: Into<String>, M: Into<String>>(label: L, msg: M) {
    let colored_label = format!("{}", label.into()).bold().bright_cyan();
    base_log(colored_label, msg.into());
}

pub fn success<L: Into<String>, M: Into<String>>(label: L, msg: M) {
    let colored_label = format!("{}", label.into()).bold().bright_green();
    base_log(colored_label, msg.into());
}

pub fn error<L: Into<String>, M: Into<String>>(label: L, msg: M) {
    let colored_label = format!("{}", label.into()).bold().bright_red();
    base_log(colored_label, msg.into());
}
