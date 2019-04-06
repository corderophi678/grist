use std::env::current_dir;
use std::fs;
use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug)]
pub struct Config {
    command: String,
    path: String,
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

#[derive(Debug)]
pub struct Repo {
    root_path: Option<PathBuf>,
    git_path: Option<PathBuf>,
    db_path: Option<PathBuf>,
    file_list: Option<Vec<PathBuf>>,
}
impl Repo {
    pub fn new(
        root_path: Option<PathBuf>,
        git_path: Option<PathBuf>,
        db_path: Option<PathBuf>,
    ) -> Repo {
        let mut repo = Repo {
            root_path,
            git_path,
            db_path,
            file_list: None,
        };
        repo.read_files();
        repo
    }
    pub fn read_files(&mut self) {
        let local_dirs = fs::read_dir(".").unwrap();

        let mut dirs = vec![];

        for entry in local_dirs {
            let entry = entry.unwrap();
            let path = fs::canonicalize(entry.path()).unwrap();

            if path == fs::canonicalize(PathBuf::from(".git")).unwrap() {
                continue;
            } else {
                dirs.push(path);
            }
        }

        self.file_list = Some(dirs);
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
            println!(
                "Initialized empty grist repository in {}",
                fs::canonicalize(git_path).unwrap().display()
            );
        }
        "commit" => {
            let root_path = current_dir().unwrap();
            let git_path = root_path.join(".git");
            let db_path = git_path.join("objects");
            Repo::new(Some(root_path), Some(git_path), Some(db_path));
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
