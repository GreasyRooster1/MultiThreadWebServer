use tokio::process::Command;
use crate::logging::log_warn;

pub fn execute_command_string(cmd:String,args: Vec<&str>){
    execute_command(cmd.as_str(),args);
}
pub fn execute_command(cmd:&str,args: Vec<&str>){
    match Command::new(cmd)
        .args(args.clone())
        .spawn() {
        Ok(_) =>{},
        Err(res) => {
            log_warn(format!("could not execute command {cmd}{}, failed with: {res}",args.join(" ")).as_str(),"execute_command");
        }
    };
}