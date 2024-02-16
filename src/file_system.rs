pub mod _easy_shell_ {
    use std::{env, fs};
    use std::f32::consts::E;
    use std::fs::{create_dir, create_dir_all, File};
    use std::io::stdin;
    use std::path::Path;
    use clearscreen;
    use crate::color;
    use crate::config::_config_::Command;
    use crate::config::SyscallHandler;

    pub struct Pwd {}
    impl SyscallHandler for Pwd{
        fn handler(_cmd:&Command) -> Result<&str,String>{
            let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
            println!("{}",current_dir);
            Ok("")
        }
    }
    
    pub struct Cd{}
    impl SyscallHandler for Cd{
        fn handler(cmd: &Command) -> Result<&str, String> {
            let path = &cmd.args.iter().peekable().peek().map_or("/", |x| *x);
            let curr_dir = Path::new(path);
            let ret = env::set_current_dir(curr_dir);
            match ret {
                Ok(_r) => {Ok("")},
                Err(e) => { println!("{}", e);
                                  Err("Failed to Change dir".to_string()) }
            }
        }
    }

    pub struct Mkdir<'mkdir>{
        pub(crate) is_p:bool,
        pub(crate) file_names:Vec<&'mkdir str>
    }
    impl SyscallHandler for Mkdir<'static>{
        fn handler(cmd: &Command) -> Result<&str, String> {
            let mut mkdir = Mkdir{is_p:false,file_names:vec![]};
            for arg in &cmd.args{
                if arg.starts_with("-"){
                    match arg.as_str() {
                        "-p" => {mkdir.is_p = true;continue}
                        _ => {println!("{}Warning invalid arguments {}{}",color!("yellow"),&arg,color!("reset"));continue;}
                    }
                }
                mkdir.file_names.push(&arg);
            }
            if mkdir.file_names.is_empty() {return  Err("Need a file name".parse().unwrap());}
            for file_name in mkdir.file_names{
                if mkdir.is_p == true{
                    let ret = create_dir_all(Path::new(file_name));
                    match ret {
                        Ok(_)=> {}
                        Err(_) => {println!("Failed to create {}",file_name)}
                    }
                }

                else {
                    let ret = create_dir(Path::new(file_name));
                    match ret {
                        Ok(_)=> {}
                        Err(e) => {
                            // ./123
                            let len = file_name.match_indices("/").collect::<Vec<_>>().len();
                            if len == 1 && file_name.starts_with("./"){
                                println!("{}",e);
                                println!("Failed to create {}",file_name);}

                            // ./123/123
                            else if len > 1 {
                                println!("{}",e);
                                println!("Try to use -p to create a cursive dir");
                            }
                        }
                    }
                }
            }
            Ok("")
        }
    }

    pub struct Rm<'rm>{
        pub(crate) is_p:bool,
        pub(crate) file_names:Vec<&'rm str>,
    }
    impl SyscallHandler for Rm<'static> {
        fn handler(cmd: &Command) -> Result<&str, String> {
            let mut rm = Rm{is_p:false,file_names:vec![]};
            for arg in &cmd.args{
                if arg.starts_with("-"){
                    match arg.as_str() {
                        "-p" => {rm.is_p = true;continue}
                        _ => {println!("{}Warning invalid arguments {}{}",color!("yellow"),&arg,color!("reset"));continue;}
                    }
                }
                rm.file_names.push(&arg);
            }
            if rm.file_names.is_empty() {return  Err("Need a file name".parse().unwrap());}

            for file_name in rm.file_names{
                match fs::metadata(file_name) {
                    Err(e) => {println!("{}",e);}
                    Ok(data) => {
                        if rm.is_p == true {
                            match fs::remove_dir_all(file_name) {
                                Err(e) => {println!("{}",e);continue}
                                Ok(_) => continue
                            }
                        }
                        if data.is_file(){match fs::remove_file(file_name) {
                            Err(e) => {println!("{}",e);continue}
                            Ok(_) => continue
                        }}
                        if data.is_dir(){match fs::remove_dir(file_name) {
                            Err(e) => {println!("{}",e);continue}
                            Ok(_) => continue
                        }}

                    }
                }
            }
            Ok("")
        }
    }

    pub struct Touch<'touch> {
        pub(crate) file_names: Vec<&'touch str>
    }

    impl SyscallHandler for Touch<'static>{
        fn handler(cmd: &Command) -> Result<&str, String> {
            let mut touch = Touch{file_names:vec![]};
            for arg in &cmd.args{
                if arg.starts_with("-"){
                    match arg.as_str() {
                        _ => {println!("{}Warning invalid arguments {}{}",color!("yellow"),&arg,color!("reset"));continue;}
                    }
                }
                touch.file_names.push(&arg);
            }
            if touch.file_names.is_empty() {return  Err("Need a file name".parse().unwrap());}

            for file_name in &cmd.args{
                match fs::metadata(Path::new(&file_name)){
                    Ok(_) => {println!("{}Warning! the file {} has existed {}",color!("yellow"),file_name,color!("reset"));
                              println!("Continuing will clear the original file,and create a new one.(y/N)");
                              let mut choice = String::new();
                              stdin().read_line(&mut choice).unwrap();
                              if choice.starts_with("\n") || choice.starts_with("N"){continue}}
                    Err(_) => {}
                }
                return match File::create(file_name){
                    Ok(_) => {Ok("")}
                    Err(e) => {Err(e.to_string())}
                }
            }
            Ok("")
        }
    }
    pub struct Clear {}
    impl SyscallHandler for Clear{
        fn handler(_cmd: &Command) -> Result<&str, String> {
            clearscreen::clear().unwrap();
            Ok("")
        }
    }
}