use std::process::Command;
use std::{env, thread};
use std::fmt::format;
use tokio::*;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use crate::logging::{log_critical, log_debug, log_error, log_info, log_warn};
use crate::main;
use crate::threadlib::*;

struct InteractiveStdin {
    chan: mpsc::Receiver<std::io::Result<String>>,
}

impl InteractiveStdin {
    fn new() -> Self {
        let (send, recv) = mpsc::channel(16);
        std::thread::spawn(move || {
            for line in std::io::stdin().lines() {
                if send.blocking_send(line).is_err() {
                    return;
                }
            }
        });
        InteractiveStdin {
            chan: recv
        }
    }

    /// Get the next line from stdin.
    ///
    /// Returns `Ok(None)` if stdin has been closed.
    ///
    /// This method is cancel safe.
    async fn next_line(&mut self) -> std::io::Result<Option<String>> {
        self.chan.recv().await.transpose()
    }
}

pub(crate) fn start_async_input()->JoinHandle<()>{
    spawn(async_input())
}

async fn async_input(){
    let mut stdin = InteractiveStdin::new();
    loop {
        select! {
            _ = signal::ctrl_c() => {
                log_info("received ctrl+c, killing process","console");
                std::process::exit(0);
                break;
            }
            res = stdin.next_line() => {
                match res{
                    Ok(None) => {
                        log_warn("no command was inputted","console");
                    }
                    Ok(Some(..))=>{
                        handel_console_input(res.unwrap().unwrap());
                    }
                    Err(_) => {
                        log_warn("error occurred on stdin.next_line()","console");
                        break;
                    }
                }
            }
        }
    }
}

fn handel_console_input(message: String){
    match message.as_str() {
        "forcequit"=>{
            log_info("killing main process...","console");
            std::process::exit(0);
        }
        "logtest"=>{
            log_info("testing the logs, following logs should be ignored","console");
            log_debug("debugging...","console");
            log_info("wow this is a log!","console");
            log_warn("im warning you...","console");
            log_error("uh oh!","console");
            log_critical("some thing really bad happened","console");
            log_info("log test has concluded!","console");
        }
        "restart"=>{
            log_warn("RESTART WILL CAUSE ISSUES, BE CAREFUL","console");
            restart_command();
        }
        "help"=>{
            help_command();
        }
        "shutdown"=>{
            shutdown_command();
        }
        "fetch"=>{
            fetch_command();
        }
        _ => {
            log_info("that doesnt appear to be a command!","console")
        }
    }
}

fn restart_command(){
    match env::current_exe() {
        Ok(exe_path) =>{
            log_info(format!("found self as {}",exe_path.display()).as_str(),"console");
            match Command::new(format!("{}",exe_path.display())).spawn() {
                Ok(_) => {
                    log_info("ran self, shutting down...","console");
                    std::process::exit(0);
                }
                Err(_) => {
                    log_warn("failed to spawn new process","console");
                }
            }

        }
        Err(e) => {
            log_warn("could not restart, cant find self","console");
        }
    };
}
fn help_command(){
    let command_names = vec!["forcequit","logtest","restart","help"];
    log_info(format!("commands: {:#?}",command_names).as_str(),"console");
}
fn shutdown_command(){
    log_info("shutting down over 5 minutes","console");
    let handel = thread::spawn(||{
        thread::sleep(time::Duration::from_secs(5));
        log_info("shutting down...","console");
        std::process::exit(0);
    });
}

fn fetch_command(){
    //git init
    //
    //create

    //git reset --hard FETCH_HEAD
    //git pull MultiThreadWebServer master
}