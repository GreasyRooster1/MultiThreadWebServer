use std::time::SystemTime;
use chrono::*;
use inline_colorization::*;

//im no longer colored :(

fn generate_log(message:&str,process:&str,time:NaiveTime,date:NaiveDate,log_type: String){
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    println!("{reset}{color_bright_cyan}[{process}] {color_blue}[{date}|{time}] {style_bold}{log_type}{reset} {message}{reset}")
}
fn generate_log_with_color(message:&str,process:&str,time:NaiveTime,date:NaiveDate,log_type: String,color:String){
    let reset = format!("{style_reset}{color_white}{bg_reset}");
    println!("{reset}{color_bright_cyan}[{process}] {color_blue}[{date}|{time}] {style_bold}{log_type}{reset}{color} {message}{reset}")
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
    generate_log_with_color(message,process,time,date,format!("{color_black}{bg_bright_red}CRITICAL"),format!("{color_bright_red}"))
}