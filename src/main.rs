
mod config;
mod file_system;
mod ls;

use std::io::stdin;
use config::_config_;
use crate::config::SyscallHandler;

fn main(){

    let mut input_command = _config_::Command::new();

    loop {
        _config_::Command::handler(&input_command).expect("Fail to set prompt");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        _config_::Command::set_config(input,&mut input_command);

        if true == _config_::Command::execute(&mut input_command){
            continue
        }

        println!("{}No such a cmd {}{}",color!("red"),&input_command.command,color!("reset"));
        _config_::Command::reset_config(&mut input_command);
    }

}