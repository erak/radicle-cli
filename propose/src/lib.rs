use std::ffi::OsString;

use anyhow::anyhow;

pub use git2::{Oid, Reference};

use librad::git::Storage;
use librad::git::Urn;

use rad_common::{git, keys, profile, project, proposal};
use rad_terminal::args::{Args, Error, Help};
use rad_terminal::components as term;

pub const HELP: Help = Help {
    name: "propose",
    description: env!("CARGO_PKG_DESCRIPTION"),
    version: env!("CARGO_PKG_VERSION"),
    usage: r#"
Usage

    rad propose [<option>...]

Options

    --create  Create a new merge proposal (default: false)
    --help    Print help
"#,
};

#[derive(Default, Debug)]
pub struct Options {
    pub create: bool,
    pub verbose: bool,
}

impl Args for Options {
    fn from_args(args: Vec<OsString>) -> anyhow::Result<(Self, Vec<OsString>)> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_args(args);
        let mut create = false;
        let mut verbose = false;

        if let Some(arg) = parser.next()? {
            match arg {
                Long("create") | Short('c') => {
                    create = true;
                }
                Long("verbose") | Short('v') => {
                    verbose = true;
                }
                Long("help") => {
                    return Err(Error::Help.into());
                }
                _ => return Err(anyhow::anyhow!(arg.unexpected())),
            }
        }

        Ok((Options { create, verbose }, vec![]))
    }
}

pub fn run(options: Options) -> anyhow::Result<()> {
    let (urn, repo) = project::cwd()
        .map_err(|_| anyhow!("this command must be run in the context of a project"))?;

    let profile = profile::default()?;
    let sock = keys::ssh_auth_sock();
    let (_, storage) = keys::storage(&profile, sock)?;
    let project = project::get(&storage, &urn)?
        .ok_or_else(|| anyhow!("couldn't load project {} from local state", urn))?;

    if options.create {
        create(project, &repo, storage, urn, options.verbose)?;
    } else {
        list(project)?;
    }

    Ok(())
}

fn list(project: project::Metadata) -> anyhow::Result<()> {
    term::headline(&format!(
        "🌱 Listing merge proposals for {}.",
        term::format::highlight(project.name)
    ));

    Ok(())
}

fn create(
    project: project::Metadata,
    repo: &git2::Repository,
    storage: Storage,
    urn: Urn,
    verbose: bool,
) -> anyhow::Result<()> {
    let head = repo.head()?;
    let current_branch = head.shorthand().unwrap_or("HEAD (no branch)");

    term::headline(&format!(
        "🌱 Creating merge proposal for {}.",
        term::format::highlight(project.name)
    ));

    let master = repo
        .resolve_reference_from_short_name(&format!("rad/{}", project.default_branch))?
        .target();
    let master_oid = master
        .map(|h| format!("{:.7}", h.to_string()))
        .unwrap_or_else(String::new);

    let head_ref = head.target();
    let head_oid = head_ref
        .map(|h| format!("{:.7}", h.to_string()))
        .unwrap_or_else(String::new);

    term::info!(
        "Proposing {} ({}) <= {} ({}).",
        term::format::highlight(project.default_branch.clone()),
        term::format::secondary(&master_oid),
        term::format::highlight(&current_branch),
        term::format::secondary(&head_oid),
    );

    let (ahead, behind) = repo.graph_ahead_behind(
        head_ref.unwrap_or(Oid::zero()),
        master.unwrap_or(Oid::zero()),
    )?;
    term::info!(
        "This branch is {} commit(s) ahead, {} commit(s) behind {}.",
        term::format::highlight(ahead),
        term::format::highlight(behind),
        term::format::highlight(project.default_branch)
    );

    let merge_base_ref = repo.merge_base(
        master.unwrap_or(Oid::zero()),
        head_ref.unwrap_or(Oid::zero()),
    );

    git::list_commits(&repo, &merge_base_ref.unwrap(), &head_ref.unwrap(), true)?;
    term::blank();

    // TODO(erikli): Replace with repo.diff()
    let workdir = repo
        .workdir()
        .ok_or_else(|| anyhow!("Could not get workdir current repository."))?;
    if term::confirm("View changes?") {
        let diff = git::git(workdir, ["diff", &master_oid, &head_oid])?;
        term::Editor::new().edit(&diff)?;
    }

    let update = proposal::exists(&repo)?;
    if update {
        if !term::confirm("Proposal already exists. Do you want to update?") {
            return Err(anyhow!("Canceled."));
        }
    } else {
        if !term::confirm_with_default("Create proposal using commit(s) above?", true) {
            return Err(anyhow!("Canceled."));
        }
    }
    term::blank();

    let proposal = match proposal::exists(&repo)? {
        true => match repo.find_note(None, head_ref.unwrap()) {
            Ok(note) => proposal::from_note(&note).unwrap(),
            Err(_) => proposal::Proposal::default(),
        },
        false => proposal::Proposal::default(),
    };

    let title: String = term::text_input("Title", Some(proposal.meta.title))?;
    let description = match term::Editor::new().edit(&proposal.description).unwrap() {
        Some(rv) => rv,
        None => String::new(),
    };
    term::success!(
        "{} {}",
        term::format::tertiary_bold("Description".to_string()),
        term::format::tertiary("·".to_string()),
    );
    term::markdown(&description);
    term::blank();

    if term::confirm_with_default("Submit using title and description?", true) {
        term::blank();

        // Create proposal and Radicle Upstream-compatible patch
        let proposal =
            proposal::create(&storage, &repo, &urn, &title, &description, update, verbose)?;
        let _patch = proposal::create_patch(&repo, &proposal, update, verbose)?;

        if term::confirm_with_default("Sync to seed?", true) {
            let sync_options = rad_sync::Options {
                origin: None,
                seed: None,
                refs: rad_sync::Refs::DefaultBranch,
                verbose: true,
                fetch: false,
                identity: false,
                push_self: false,
            };
            rad_sync::run(sync_options)?;
        }
    } else {
        return Err(anyhow!("Canceled."));
    }

    Ok(())
}
