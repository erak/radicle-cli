use std::default::Default;

use anyhow::anyhow;
use regex::Regex;
pub use serde::{Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

pub use git2::{Note, Repository};
pub use librad::git::Urn;
pub use librad::git::refs::Refs;
pub use librad::git::storage::Storage;

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
            meta: Metadata{
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

    // TODO(erikli): Activate or remove depending on remote-helper behaviour.
    // let spinner = term::spinner(&format!("Creating note in monorepo."));
    // let profile = profile::default()?;
    // let repo = git2::Repository::open_bare(storage.as_ref().path())?;
    // let monorepo = profile::monorepo(&profile)?;
    // match monorepo.note(
    //     &monorepo.signature()?,
    //     &monorepo.signature()?,
    //     Some(&format!("refs/namespaces/{}/refs/notes/commits", urn.encode_id())),
    //     head_ref.unwrap(),
    //     &proposal.to_string(),
    //     force,
    // ) {
    //     Ok(oid) => {
    //         if verbose {
    //             term::info!("Created note {} in monorepo.", oid);
    //         }
    //         spinner.finish();
    //     }
    //     Err(err) => {
    //         spinner.failed();
    //         return Err(anyhow!(err));
    //     }
    // }
    // Refs::update(storage, urn)?;

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
        description: document.content
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
        Err(_) => false
    };

    Ok(exists)
}


