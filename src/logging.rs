use std::{process, thread};
use std::backtrace::Backtrace;
use std::io::{Read, stdin, stdout, Write};
use std::time::SystemTime;
use chrono::*;
use inline_colorization::*;
use log::trace;

//im no longer colored :(

fn generate_log(message:&str,process:&str,time:NaiveTime,date:NaiveDate,log_type: String){
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    let thread = thread::current();
    let thread_name = match thread.name() {
        None => {"anon"}
        Some(name) => {
            name
        }
    };
    println!("{reset}{color_bright_cyan}[{process}|{thread_name}] {color_blue}[{date}|{time}] {style_bold}{log_type}{reset} {message}{reset}")
}
fn generate_log_with_color(message:&str,process:&str,time:NaiveTime,date:NaiveDate,log_type: String,color:String){
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    let thread = thread::current();
    let thread_name = match thread.name() {
        None => {"anon"}
        Some(name) => {
            name
        }
    };
    println!("{reset}{color_bright_cyan}[{process}|{thread_name}] {color_blue}[{date}|{time}] {style_bold}{log_type}{reset}{color} {message}{reset}")
}

pub fn log_info(message:&str,process:&str){
    let time = Local::now().time();
    let date = Local::now().date_naive();
    generate_log(message,process,time,date,format!("{color_green}INFO"))
}

pub fn log_debug(message:&str,process:&str){
    let time = Local::now().time();
    let date = Local::now().date_naive();
    generate_log(message,process,time,date,format!("{color_magenta}DEBUG"))
}

pub fn log_warn(message:&str,process:&str){
    let time = Local::now().time();
    let date = Local::now().date_naive();
    generate_log(message,process,time,date,format!("{color_yellow}WARN"))
}

pub fn log_error(message:&str,process:&str){
    let time = Local::now().time();
    let date = Local::now().date_naive();
    generate_log(message,process,time,date,format!("{color_red}ERROR"))
}

pub fn log_critical(message:&str,process:&str){
    let time = Local::now().time();
    let date = Local::now().date_naive();
    generate_log_with_color(message,process,time,date,format!("{color_black}{bg_bright_red}CRITICAL"),format!("{style_bold}{color_bright_red}"));

}

pub fn log_trace(message:&str,process:&str){
    let trace= Backtrace::force_capture();
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    let thread = thread::current();
    let time = Local::now().time();
    let date = Local::now().date_naive();
    let thread_name = match thread.name() {
        None => {"anon"}
        Some(name) => {
            name
        }
    };
    println!("{reset}{color_bright_cyan}[{process}-{thread_name}] {color_blue}[{date}|{time}] {style_bold}{color_bright_red}TRACE{reset} {message}{reset}\n{trace}")
}

pub fn log_title(message:&str){
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    println!("{style_bold}{color_blue}----------==={color_bright_magenta}{message}{color_blue}===----------{reset}")
}

fn log_pause(message:&str,process:&str) {
    let time = Local::now().time();
    let date = Local::now().date_naive();
    let mut out = stdout();
    let log = generate_log(message,process,time,date,"PAUSE".as_str());
    out.write(log.as_bytes()).unwrap();
    out.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}