use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

use utime::Utime;

const ERROR: u8 = 0;
const WARN: u8 = 1;
const INFO: u8 = 2;
const DEBUG: u8 = 3;

static LOG_LEVEL: AtomicU8 = AtomicU8::new(DEBUG);

pub struct Ulog {
    level: u8,
    fragments: Vec<String>,
}

pub fn set_log_level(level: u8) {
    if level > DEBUG {
        LOG_LEVEL.store(DEBUG, Relaxed);
    } else {
        LOG_LEVEL.store(level, Relaxed);
    }
}

pub fn get_log_level() -> u8 {
    LOG_LEVEL.load(Relaxed)
}

pub fn error(msg: &str) -> Ulog {
    log(msg, ERROR, None)
}

pub fn warn(msg: &str) -> Ulog {
    log(msg, WARN, None)
}

pub fn info(msg: &str) -> Ulog {
    log(msg, INFO, None)
}

pub fn debug(msg: &str) -> Ulog {
    log(msg, DEBUG, None)
}

fn log(msg: &str, level: u8, fragments: Option<&Vec<String>>) -> Ulog {
    let level_str = match level {
        ERROR => "ERROR",
        WARN => "WARN",
        INFO => "INFO",
        DEBUG => "DEBUG",
        _ => "DEBUG",
    };

    let mut f: Vec<String> = vec![
        format!("time=\"{}\"", Utime::now().as_iso_8601_datetime()),
        format!("level=\"{}\"", level_str),
        format!("message=\"{}\"", msg.replace("\"", "\\\"")),
    ];

    if fragments.is_some() {
        for s in fragments.unwrap() {
            f.push(s.clone());
        }
    }

    Ulog {
        level,
        fragments: f,
    }
}

impl Drop for Ulog {
    fn drop(&mut self) {
        if self.level <= LOG_LEVEL.load(Relaxed) {
            println!("{}", self.fragments.join(" "));
        }
    }
}

impl Ulog {
    pub fn data(mut self, key: &str, value: &str) -> Ulog {
        self.fragments
            .push(format!("{}=\"{}\"", key, value.replace("\"", "\\\"")));
        return self;
    }
}

pub struct Ulogger {
    fragments: Vec<String>,
}

impl Ulogger {
    pub fn new() -> Ulogger {
        Self { fragments: vec![] }
    }

    pub fn add_data(&mut self, key: &str, value: &str) {
        self.fragments
            .push(format!("{}=\"{}\"", key, value.replace("\"", "\\\"")));
    }

    pub fn error(&self, msg: &str) -> Ulog {
        log(msg, ERROR, Some(&self.fragments))
    }

    pub fn warn(&self, msg: &str) -> Ulog {
        log(msg, WARN, Some(&self.fragments))
    }

    pub fn info(&self, msg: &str) -> Ulog {
        log(msg, INFO, Some(&self.fragments))
    }

    pub fn debug(&self, msg: &str) -> Ulog {
        log(msg, DEBUG, Some(&self.fragments))
    }
}
