use std::env;
use std::process::exit;
use std::path::{Path, PathBuf};
use is_executable::is_executable;
use clap::{arg, command,Arg, ArgMatches};


fn parse_args() -> ArgMatches{
    let matches = command!().about("rwhich returns the pathnames of the executable")
        .arg(
            Arg::new("bin")
                .required(true)
                .help("The name of binary")
        )
        .arg(
            arg!(
                -a --all "Show all occurence of binary"
            )
            .help("Print all matching pathnames of binary")
        )
        .get_matches();

    matches
}

fn main(){
    const ENV_VAR: &str = "PATH";
    const PATH_SEP: &str = if cfg!(windows) {";"} else {":"};

    let args = parse_args();
    let bin_name = args.get_one::<String>("bin").unwrap();
    let find_all = args.get_one::<bool>("all").unwrap();
    

    let res = env::var(ENV_VAR);

    let value = match  res{
        Ok(res) => res,
        Err(_) => {
            eprintln!("Environment variable {} not found", ENV_VAR);
            exit(1);
        }, 
    };

    let splited = value.split(PATH_SEP)
            .enumerate()
            .filter(|&(_, f)| f != "")
            .map(|(_, f)| f);

    let mut _path: PathBuf;

    for path in splited{

            _path = Path::new(path).join(bin_name);

            if _path.exists() && is_executable(_path.as_path()){
                println!("{}", _path.as_path().display());
                if !find_all{
                    break;
                }
            }
    }
}
