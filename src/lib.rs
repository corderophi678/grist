// use std::error::Error;
// use std::fs::DirBuilder;
// // use std::fs;

// #[derive(Debug)]
// pub struct Config {
//     pub command: String,
//     pub path: String,
// }

// impl Config {
//     pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
//         args.next();

//         let command = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a query string"),
//         };
//         let path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get a file name"),
//         };

//         Ok(Config { command, path })
//     }
// }

// pub fn run(config: Config) -> Result<(), &'static str> {
//     // let path = &config.path;
//     // let root_path = "";
//     // let git_path = "";
//     match config.command {
//         _ => Err(format!("grist: {} is not a grist command", config.command)),
//     }

//     Ok(())
// }

// fn make_dirs(dirs: Vec<String>) -> Result<(), Box<dyn Error>> {
//     for dir in dirs {
//         DirBuilder::new().recursive(true).create(dir)?;
//     }
//     Ok(())
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
