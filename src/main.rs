use jilu::{git, Changelog, Config, Error};

fn main() {
    match run() {
        Ok(log) => print!("{}", log),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<String, Error> {
    let repo = git2::Repository::open(".")?;
    let config = Config::default();
    let mut out = String::new();

    Changelog::new(config, git::commits(&repo)?, git::tags(&repo)?)?.format(&mut out)?;

    Ok(out)
}
