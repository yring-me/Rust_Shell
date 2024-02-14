
mod config;
mod easy_shell;

use std::io::stdin;
use config::_config_;




fn main(){

    let mut command_cfg = _config_::Command::new();
    // let mut input = String::new();
    println!("{}",shell_id!("cd"));
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        _config_::Command::set_config(input,&mut command_cfg);

        _config_::Command::reset_config(&mut command_cfg);
    }

}