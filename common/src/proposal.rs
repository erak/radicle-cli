use std::default::Default;

use chrono::{DateTime, NaiveDateTime, Utc};

use anyhow::anyhow;
use regex::Regex;
pub use serde::{Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

pub use git2::{Note, Repository};
use librad::crypto::BoxedSigner;
pub use librad::git::refs::Refs;
pub use librad::git::storage::Storage;
pub use librad::git::Urn;

use crate::{git, profile};

use rad_terminal::components as term;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposal {
    pub meta: Metadata,
    pub description: String,
}

impl Proposal {
    fn remove_comment(&mut self) {
        self.description = Regex::new("<!--(.*?)-->")
            .unwrap()
            .replace_all(&self.description, "")
            .to_string();
    }
}

impl Default for Proposal {
    fn default() -> Self {
        Proposal {
            meta: Metadata {
                title: "".to_string(),
            },
            description: "".to_string(),
        }
    }
}

impl ToString for Proposal {
    fn to_string(&self) -> String {
        format!(
            r#"
---
title: '{}'
---
{}"#,
            &self.meta.title, &self.description
        )
    }
}

// TODO(erikli): Implement (From<&git2::Note<'_>> also needed?).
// impl TryFrom<&git2::Note<'_>> for Proposal {
//     type Error = anyhow::Error;
//     fn try_from(note: &git2::Note) -> Result<Self, Self::Error> {
//         let message = match note.message() {
//             Some(msg) => msg,
//             None => return Err(anyhow!("Note's message is not UTF-8.")),
//         };

//         let document = YamlFrontMatter::parse::<Metadata>(&message).unwrap();
//         let description = "".to_string();
//         Ok(Proposal {
//             meta: document.metadata,
//             description: description
//         })
//     }
// }

pub fn create(
    storage: &Storage,
    repo: &git2::Repository,
    urn: &Urn,
    title: &str,
    description: &str,
    force: bool,
    verbose: bool,
) -> anyhow::Result<Proposal> {
    let head = repo.head()?;
    let head_ref = head.target();
    let proposal = Proposal {
        meta: Metadata {
            title: title.to_string(),
        },
        description: description.to_string(),
    };

    let spinner = term::spinner(&format!("Creating note in working copy."));
    match repo.note(
        &repo.signature()?,
        &repo.signature()?,
        None,
        head_ref.unwrap(),
        &proposal.to_string(),
        force,
    ) {
        Ok(oid) => {
            if verbose {
                term::info!("Created note {} in working copy.", oid);
            }
            spinner.finish();
        }
        Err(err) => {
            spinner.failed();
            return Err(anyhow!(err));
        }
    }

    // TODO(erikli): Remove after pushing via remote-helper was enabled.
    let spinner = term::spinner(&format!("Creating note in monorepo."));
    let profile = profile::default()?;

    let monorepo = git2::Repository::open_bare(profile.paths().git_dir())?;
    match monorepo.note(
        &monorepo.signature()?,
        &monorepo.signature()?,
        Some(&format!(
            "refs/namespaces/{}/refs/notes/commits",
            urn.encode_id()
        )),
        head_ref.unwrap(),
        &proposal.to_string(),
        force,
    ) {
        Ok(oid) => {
            if verbose {
                term::info!("Created note {} in monorepo.", oid);
            }
            spinner.finish();
        }
        Err(err) => {
            spinner.failed();
            return Err(anyhow!(err));
        }
    }
    Refs::update(storage, urn)?;

    Ok(proposal)
}

pub fn from_note(note: &git2::Note) -> anyhow::Result<Proposal> {
    let message = match note.message() {
        Some(msg) => msg,
        None => return Err(anyhow!("Note's message is not UTF-8.")),
    };
    let document = YamlFrontMatter::parse::<Metadata>(&message).unwrap();
    Ok(Proposal {
        meta: document.metadata,
        description: document.content,
    })
}

/// Radicle Upstream compatibility:
/// Create and push tag to monorepo.
pub fn create_patch(
    repo: &git2::Repository,
    proposal: &Proposal,
    force: bool,
    verbose: bool,
) -> anyhow::Result<git::Patch> {
    let head = repo.head()?;
    let current_branch = head.shorthand().unwrap_or("HEAD (no branch)");
    let patch_tag_name = format!("radicle-patch/{}", current_branch);
    let spinner = term::spinner(&format!(
        "Creating {} patch.",
        term::format::highlight("Radicle Upstream".to_string())
    ));
    let patch = match git::add_tag(&repo, &proposal.meta.title, &patch_tag_name, force) {
        Ok(patch_tag_name) => patch_tag_name,
        Err(err) => {
            spinner.failed();
            return Err(err);
        }
    };
    if verbose {
        term::info!("Created tag {}", &patch.tag_name);
    }

    let output = match git::push_tag(&patch.tag_name, force) {
        Ok(output) => output,
        Err(err) => {
            spinner.failed();
            return Err(err);
        }
    };
    if verbose {
        term::blob(output);
    }
    spinner.finish();

    Ok(patch)
}

pub fn exists(repo: &git2::Repository) -> anyhow::Result<bool> {
    let head = repo.head()?;
    let head_ref = head.target().unwrap();

    let exists = match repo.find_note(None, head_ref) {
        Ok(_) => true,
        Err(_) => false,
    };

    Ok(exists)
}

pub fn list_outgoing(profile: &profile::Profile, urn: &Urn, verbose: bool) -> anyhow::Result<()> {
    let notes_ref = format!("refs/namespaces/{}/refs/notes/commits", urn.encode_id());
    let prefix = "[↑]".to_string();

    list(&profile, &notes_ref, &prefix, verbose)?;

    Ok(())
}

pub fn list_incoming(profile: &profile::Profile, urn: &Urn, verbose: bool) -> anyhow::Result<()> {
    let notes_ref = format!("refs/namespaces/{}/refs/notes/commits", urn.encode_id());
    let prefix = "[↓]".to_string();

    list(&profile, &notes_ref, &prefix, verbose)?;

    Ok(())
}

fn list(profile: &profile::Profile, notes_ref: &str, prefix: &str, verbose: bool) -> anyhow::Result<()> {
    let storage = git2::Repository::open_bare(profile.paths().git_dir())?;
    // TODO(erikli): Replace above with wrapped
    // let peer = storage.peer_id();
    // let notes = refs::notes(&storage, &urn, Some(*peer))?;
    
    let mut notes = storage.notes(Some(&notes_ref))?;
    while let Some(note) = notes.next() {
        let (_, commit_id) = note?;

        let proposal = match storage.find_note(Some(&notes_ref), commit_id) {
            Ok(note) => from_note(&note).unwrap(),
            Err(_) => Proposal::default(),
        };
        let note = storage.find_note(Some(&notes_ref), commit_id)?;
        let author = note.author();

        let when = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(author.when().seconds(), 0),
            Utc,
        );
        let since = chrono::offset::Utc::now().signed_duration_since(when);
        let date = match since.num_days() {
            0 => match since.num_hours() {
                0 => match since.num_minutes() {
                    0 => format!("{} seconds ago", since.num_seconds()),
                    _ => format!("{} minutes ago", since.num_minutes()),
                },
                _ => format!("{} hours ago", since.num_hours()),
            },
            1 => "yesterday".to_string(),
            2 => "2 days ago".to_string(),
            3 => "3 days ago".to_string(),
            4 => "4 days ago".to_string(),
            _ => format!("on {}", when.format("%B %d")),
        };

        // TODO(erikli): Add indicator for local / synced state.
        term::info!(
            "{} {} {}",
            term::format::secondary(format!("{:.7}", commit_id.to_string())),
            term::format::bold(prefix),
            term::format::bold(proposal.meta.title),
        );
        term::info!(
            "└── {}",
            term::format::italic(format!("opened {} by {}", date, author.name().unwrap()))
        );

        if verbose {
            term::markdown(&proposal.description);
            term::blank();
        }
    }

    Ok(())
}
