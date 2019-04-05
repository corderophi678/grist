use std::env::current_dir;
use std::fs::DirBuilder;
use std::path::Path;
use std::process;

#[derive(Debug)]
pub struct Config {
    pub command: String,
    pub path: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("No command issued. Try 'init',"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap(),
        };

        Ok(Config { command, path })
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let command = config.command;
    let path = config.path;
    match command.as_ref() {
        "init" => {
            let root_path = Path::new(&path);
            let git_path = root_path.join(".git");

            for dir in ["objects", "refs"].iter() {
                DirBuilder::new()
                    .recursive(true)
                    .create(git_path.join(dir))
                    .unwrap_or_else(|err| {
                        eprintln!("Problem creating directories: {}", err);
                        process::exit(1);
                    });
            }
            println!("Initialized empty grist repository in {:?}", git_path);
        }
        _ => eprintln!("grist: {} is not a grist command", command),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
