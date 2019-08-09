use git2::{Commit, Repository, Sort, Tag};
use jilu::{Changelog, Config};
use std::{error::Error, process::exit};

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(err) => {
            eprintln!("Git error: {}", err);
            exit(1);
        }
    };

    let (commits, tags) = match repo_details(&repo) {
        Ok((commits, tags)) => (commits, tags),
        Err(err) => {
            eprintln!("Git error: {}", err);
            exit(1);
        }
    };

    let config = Config::default();
    let mut md = String::new();

    match Changelog::new(config, commits, tags) {
        Ok(log) => match log.format(&mut md) {
            Ok(()) => print!("{}", md),
            Err(err) => eprintln!("application error: {}", err),
        },
        Err(err) => eprintln!("application error: {}", err),
    };
}

fn repo_details<'a>(
    repo: &'a Repository,
) -> Result<(Vec<Commit<'a>>, Vec<Tag<'a>>), Box<dyn Error>> {
    let mut walk = repo.revwalk()?;
    walk.push_head()?;
    walk.simplify_first_parent();
    walk.set_sorting(Sort::REVERSE | Sort::TOPOLOGICAL);

    let commits = walk
        .filter_map(|oid| match oid {
            Ok(oid) => repo.find_commit(oid).ok(),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    let tags = repo
        .tag_names(None)?
        .into_iter()
        .filter_map(|option| {
            option.and_then(|name| {
                repo.revparse_single(name)
                    .ok()
                    .and_then(|o| o.into_tag().ok())
            })
        })
        .collect::<Vec<_>>();

    Ok((commits, tags))
}
