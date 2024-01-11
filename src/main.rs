use std::env;
use std::process::exit;
use std::path::{Path, PathBuf};
use is_executable::is_executable;

fn main(){
    let env_var = "PATH";
    // {{{ Collect cli arguments
    let tmp = env::args().skip(1);

    if tmp.len() < 1{
        eprintln!("Usage: rwhich <bin>");
        exit(1);
    }

    let args: Vec<String> = tmp.collect();
    // }}}

    let res = env::var(env_var);

    let value = match  res{
        Ok(res) => res,
        Err(_) => {
            eprintln!("Environment variable {} not found", env_var);
            exit(1);
        }, 
    };

    let splited = value.split(";")
            .enumerate()
            .filter(|&(_, f)| f != "")
            .map(|(_, f)| f);

    let mut _path: PathBuf;
    println!("{}", is_executable(Path::new("D:\\Neovim\\bin\\rustc.exe")));
    for path in splited{

            _path = Path::new(path).join(&args[0]);

            if _path.exists() && is_executable(_path.as_path()){
                println!("{}", _path.as_path().display())
            }
    }
}
