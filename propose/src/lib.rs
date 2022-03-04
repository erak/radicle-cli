use std::convert::TryInto;
use std::ffi::OsString;

use anyhow::{anyhow, bail};

pub use git2::{Oid, Reference};

use librad::canonical::Cstring;
use librad::identities::payload::{self};
use librad::PeerId;

use rad_common::{git, keys, person, profile, project};
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

    --help    Print help
"#,
};

pub struct Options {}

impl Args for Options {
    fn from_args(args: Vec<OsString>) -> anyhow::Result<(Self, Vec<OsString>)> {
        use lexopt::prelude::*;

        let mut parser = lexopt::Parser::from_args(args);

        if let Some(arg) = parser.next()? {
            match arg {
                Long("help") => {
                    return Err(Error::Help.into());
                }
                _ => return Err(anyhow::anyhow!(arg.unexpected())),
            }
        }

        Ok((Options {}, vec![]))
    }
}

pub fn run(_options: Options) -> anyhow::Result<()> {
    let (urn, _) = project::cwd()
        .map_err(|_| anyhow!("this command must be run in the context of a project"))?;

    let profile = profile::default()?;
    let sock = keys::ssh_auth_sock();
    let (_, storage) = keys::storage(&profile, sock)?;
    let project = project::get(&storage, &urn)?
        .ok_or_else(|| anyhow!("couldn't load project {} from local state", urn))?;

    term::headline("Creating merge proposal for your 🌱 project");

    // let meta: project::Metadata = project.try_into()?;

    let repo = project::repository()?;
    let master = repo
        .resolve_reference_from_short_name(&format!("rad/{}", project.default_branch))?
        .target();
    let master_oid = master
        .map(|h| format!("{:.7}", h.to_string()))
        .unwrap_or_else(String::new);

    let head = repo.head()?.target();
    let head_oid = head
        .map(|h| format!("{:.7}", h.to_string()))
        .unwrap_or_else(String::new);

    term::info!(
        "Comparing {} ({}) <= {} ({})",
        term::format::highlight(project.default_branch),
        term::format::secondary(master_oid),
        term::format::highlight(repo.head()?.shorthand().unwrap_or("HEAD (no branch)")),
        term::format::secondary(head_oid),
    );

    let merge_base = repo.merge_base(master.unwrap_or(Oid::zero()), head.unwrap_or(Oid::zero()));
    term::info!(
        "Found merge base {} ...",
        term::format::secondary(merge_base
            .map(|h| format!("{:.7}", h.to_string()))
            .unwrap_or(String::new()))
    );
    // let mut table = term::Table::default();

    // table.push([
    //     term::format::bold("master"),
    //     term::format::bold("fea"),
    // ]);

    // table.push([
    //     term::format::secondary(head.unwrap_or(Oid::zero())),
    //     term::format::secondary(head.unwrap_or(Oid::zero())),
    // ]);

    // table.render();

    // term::info!("master                                   <= HEAD");
    // term::info!(
    //     "{}    {}",
    //     head.unwrap_or(Oid::zero()),
    //     head.unwrap_or(Oid::zero())
    // );
    // let oid = repo.merge_base()?;

    Ok(())
}
