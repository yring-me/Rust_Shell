pub mod _ls_{
    use std::ffi::CStr;
    use libc;
    use std::fs;
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    use std::path::Path;
    use std::time::UNIX_EPOCH;
    use chrono::{NaiveDateTime};
    use crate::config::SyscallHandler;
    use crate::config::_config_::Command;
    use crate::color;
    pub struct Ls<'ls>{
        pub(crate) is_l:bool,
        pub(crate) is_a:bool,
        pub(crate) file_names:Vec<&'ls str>
    }

    impl SyscallHandler for Ls<'static> {
        fn handler(cmd: &Command) -> Result<&str, String> {
            let mut ls = Ls{is_a:false,is_l:false,file_names:vec!["."]};
            let mut flag:bool = false;
            for arg in &cmd.args{
                if arg.starts_with("-"){
                    match arg.as_str() {
                        "-l" => { ls.is_l = true;continue}
                        "-a" => { ls.is_a = true;continue}
                        "-al" => {ls.is_a = true;ls.is_l = true;continue}
                        "-la" => {ls.is_l = true;ls.is_a = true;continue}
                        _ => {println!("{}Warning invalid arguments {}{}",color!("yellow"),&arg,color!("reset"));continue;}
                    }
                }
                if flag == false{
                    ls.file_names.clear();
                    flag = true;
                }
                ls.file_names.push(&arg);
            }

            for file_name in &ls.file_names{
                if ls.file_names.len() > 1 {println!("{}:",file_name);}

                if ls.is_l == true{
                    match ls_l(file_name,ls.is_a) {
                        Err(e) => {println!("{}",e);println!()}
                        Ok(_) => {println!();continue; }
                    }
                    continue;
                }

                else {
                    match ls_default(file_name,ls.is_a) {
                        Err(e) => {println!("{}",e);println!()}
                        Ok(_) => {println!();continue; }
                    }
                    continue;
                }

            }
            return Ok("")

        }
    }

    fn ls_default(file_name:&str,is_a:bool) -> Result<&str,String>{
        match fs::read_dir(file_name) {
            Err(e) => { println!("{}", e);return Err("Failed to ls ".to_string() + file_name); },
            Ok(paths) => for path in paths {
                let name = path.unwrap().path().to_str().unwrap().to_string();
                match fs::metadata(Path::new(&name)){
                    Err(_) => {continue}

                    Ok(data) => {
                        let mut name = name.replacen(file_name,"",1);
                        if name.starts_with("/"){name = name.replace("/","");}
                        // escape hidden file
                        if is_a == false && name.starts_with("."){continue}
                        // dir
                        if data.is_dir(){print!("{}{}{}\t",color!{"blue"},name,color!("reset"));}
                        //file
                        if data.is_file(){print!("{}\t",name);}
                        //sym
                        if data.is_symlink(){print!("{}{}{}\t",color!{"magenta"},name,color!("reset"));}
                    },
                };
            }
        }
        // println!();
        Ok("")
    }


    fn ls_l(file_name:&str,is_a:bool) -> Result<&str,String>{
        let mat = ["---", "--x", "-w-", "-wx", "r--", "r-x", "rw-", "rwx"];

        match fs::read_dir(file_name) {
            Err(e) => { println!("{}", e);return Err("Failed to ls ".to_string() + file_name); },
            Ok(paths) => for path in paths {
                let name = path.unwrap().path().to_str().unwrap().to_string();

                match fs::metadata(Path::new(&name)){
                    Err(_) => {continue}

                    Ok(data) => {
                        // original name will starts_with the input,therefor use replace to cut it
                        // specially . ./
                        let mut name = name.replacen(file_name,"",1);
                        if name.starts_with("/"){name = name.replace("/","");}

                        if is_a == false && name.starts_with(".") {continue}

                        // dir or normal file
                        if data.is_dir(){print!("d");}
                        else { print!("-"); }

                        unsafe {
                            print!("d{}{}{}  {}\t{}\t{}\t{}\t{:?}\t",
                                     mat[data.permissions().mode() as usize & 0x3],
                                     mat[(data.permissions().mode() as usize >> 3) & 0x3],
                                     mat[(data.permissions().mode() as usize >> 6) & 0x3],
                                     data.nlink(),
                                     CStr::from_ptr((*libc::getpwuid(data.uid())).pw_name).to_str().unwrap(),
                                     CStr::from_ptr((*libc::getgrgid(data.gid())).gr_name).to_str().unwrap(),
                                     data.len(),
                                     NaiveDateTime::from_timestamp_opt(data.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64, 0).unwrap());
                        }
                        // dir
                        if data.is_dir(){
                                println!("{}{}{}\t", color! {"blue"}, name, color!("reset"));
                            }

                        //file
                        if data.is_file(){
                                println!("{}\t", name);
                        }
                        //sym
                        if data.is_symlink(){
                                println!("{}{}{}\t", color! {"magenta"}, name, color!("reset"));
                        }
                    },
                };
            }
        }
        // println!();
        Ok("")
    }

}