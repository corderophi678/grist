#[allow(unused)]
use std::env;
use std::fs::DirBuilder;
use std::path::Path;
use std::process;

fn main() {
    // Grab args from env.
    let mut received_args = env::args();
    // Bypass args[0] because we don't need it.
    received_args.next();
    let command = match received_args.next() {
        Some(arg) => arg,
        None => process::exit(1),
    };
    let path = match received_args.next() {
        Some(arg) => arg,
        None => process::exit(1),
    };

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
}
