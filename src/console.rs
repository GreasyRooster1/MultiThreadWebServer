use tokio::*;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use crate::logging::{log_critical, log_debug, log_error, log_info, log_warn};
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
        _ => {
            log_info("that doesnt appear to be a command!","console")
        }
    }
}