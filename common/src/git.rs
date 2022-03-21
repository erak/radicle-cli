//! Git-related functions and types.
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context as _;

use librad::git::local::url::LocalUrl;
use librad::git::types::remote::Remote;

use librad::profile::Profile;
use librad::{crypto::BoxedSigner, PeerId};

pub use git2::Oid;
pub use git2::Repository;
pub use librad::git::local::transport;
pub use librad::git::types::remote::LocalFetchspec;
pub use serde::{Deserialize, Serialize};

use crate::keys;

use rad_terminal::components as term;

pub const CONFIG_COMMIT_GPG_SIGN: &str = "commit.gpgsign";
pub const CONFIG_SIGNING_KEY: &str = "user.signingkey";
pub const CONFIG_GPG_FORMAT: &str = "gpg.format";
pub const CONFIG_GPG_SSH_PROGRAM: &str = "gpg.ssh.program";
pub const CONFIG_GPG_SSH_ALLOWED_SIGNERS: &str = "gpg.ssh.allowedSignersFile";

/// Minimum required git version.
pub const VERSION_REQUIRED: Version = Version {
    major: 2,
    minor: 34,
    patch: 0,
};

/// A parsed git version.
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl std::str::FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rest = input
            .strip_prefix("git version ")
            .ok_or(anyhow!("malformed git version string"))?;
        let rest = rest
            .split(' ')
            .next()
            .ok_or(anyhow!("malformed git version string"))?;
        let rest = rest.trim_end();

        let mut parts = rest.split('.');
        let major = parts
            .next()
            .ok_or(anyhow!("malformed git version string"))?
            .parse()?;
        let minor = parts
            .next()
            .ok_or(anyhow!("malformed git version string"))?
            .parse()?;

        let patch = match parts.next() {
            None => 0,
            Some(patch) => patch.parse()?,
        };

        Ok(Self {
            major,
            minor,
            patch,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Patch {
    pub title: String,
    pub tag_name: String,
}

/// Get the system's git version.
pub fn version() -> Result<Version, anyhow::Error> {
    let output = Command::new("git").arg("version").output()?;

    if output.status.success() {
        let output = String::from_utf8(output.stdout)?;
        let version = output
            .parse()
            .with_context(|| format!("unable to parse git version string {:?}", output))?;

        return Ok(version);
    }
    Err(anyhow!("failed to run `git version`"))
}

/// Get the git repository in the current directory.
pub fn repository() -> Result<Repository, anyhow::Error> {
    match Repository::open(".") {
        Ok(repo) => Ok(repo),
        Err(err) => Err(err).context("the current working directory is not a git repository"),
    }
}

/// Execute a git command by spawning a child process.
pub fn git<S: AsRef<std::ffi::OsStr>>(
    repo: &std::path::Path,
    args: impl IntoIterator<Item = S>,
) -> Result<String, anyhow::Error> {
    let output = Command::new("git").current_dir(repo).args(args).output()?;

    if output.status.success() {
        let out = if output.stdout.is_empty() {
            &output.stderr
        } else {
            &output.stdout
        };
        return Ok(String::from_utf8_lossy(out).into());
    }

    Err(anyhow::Error::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        String::from_utf8_lossy(&output.stderr),
    )))
}

/// Configure SSH signing in the given git repo, for the given peer.
pub fn configure_signing(repo: &Path, peer_id: &PeerId) -> Result<(), anyhow::Error> {
    let key = keys::to_ssh_key(peer_id)?;

    git(repo, ["config", "--local", CONFIG_SIGNING_KEY, &key])?;
    git(repo, ["config", "--local", CONFIG_GPG_FORMAT, "ssh"])?;
    git(repo, ["config", "--local", CONFIG_COMMIT_GPG_SIGN, "true"])?;
    git(
        repo,
        ["config", "--local", CONFIG_GPG_SSH_PROGRAM, "ssh-keygen"],
    )?;
    git(
        repo,
        [
            "config",
            "--local",
            CONFIG_GPG_SSH_ALLOWED_SIGNERS,
            ".gitsigners",
        ],
    )?;

    Ok(())
}

/// Write a `.gitsigners` file in the given repository.
/// Fails if the file already exists.
pub fn write_gitsigners<'a>(
    repo: &Path,
    signers: impl IntoIterator<Item = &'a PeerId>,
) -> Result<PathBuf, io::Error> {
    let path = Path::new(".gitsigners");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(repo.join(path))?;

    for peer_id in signers.into_iter() {
        write_gitsigner(&mut file, peer_id)?;
    }
    Ok(path.to_path_buf())
}

/// Add signers to the repository's `.gitsigners` file.
pub fn add_gitsigners<'a>(
    path: &Path,
    signers: impl IntoIterator<Item = &'a PeerId>,
) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path.join(".gitsigners"))?;

    for peer_id in signers.into_iter() {
        write_gitsigner(&mut file, peer_id)?;
    }
    Ok(())
}

/// Read a `.gitsigners` file.
pub fn read_gitsigners(path: &Path) -> Result<HashSet<PeerId>, io::Error> {
    use std::io::BufRead;

    let mut peers = HashSet::new();
    let file = File::open(path.join(".gitsigners"))?;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if let Some((peer_id, key)) = line.split_once(' ') {
            let peer = PeerId::from_str(peer_id)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let expected = keys::to_ssh_key(&peer)?;
            if key != expected {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "key does not match peer id",
                ));
            }
            peers.insert(peer);
        }
    }
    Ok(peers)
}

/// Add a path to the repository's git ignore file. Creates the
/// ignore file if it does not exist.
pub fn ignore(repo: &Path, item: &Path) -> Result<(), io::Error> {
    let mut ignore = OpenOptions::new()
        .append(true)
        .create(true)
        .open(repo.join(".gitignore"))?;

    writeln!(ignore, "{}", item.display())
}

/// Check whether SSH or GPG signing is configured in the given repository.
pub fn is_signing_configured(repo: &Path) -> Result<bool, anyhow::Error> {
    Ok(git(repo, ["config", CONFIG_SIGNING_KEY]).is_ok())
}

/// Return the list of radicle remotes for the given repository.
pub fn remotes(repo: &git2::Repository) -> anyhow::Result<Vec<(String, PeerId)>> {
    let mut remotes = Vec::new();

    for name in repo.remotes().iter().flatten().flatten() {
        let remote = repo.find_remote(name)?;
        for refspec in remote.refspecs() {
            if refspec.direction() != git2::Direction::Fetch {
                continue;
            }
            if let Some((peer, _)) = refspec.src().and_then(self::parse_remote) {
                remotes.push((name.to_owned(), peer));
            }
        }
    }

    Ok(remotes)
}

/// Set the upstream for the given remote and branch.
/// Creates the tracking branch if it does not exist.
pub fn set_upstream(repo: &Path, remote: &str, branch: &str) -> anyhow::Result<String> {
    let branch_name = format!("{}/{}", remote, branch);

    git(
        repo,
        [
            "branch",
            &branch_name,
            &format!("{}/heads/{}", remote, branch),
        ],
    )?;

    Ok(branch_name)
}

/// Call `git pull`, optionally with `--force`.
pub fn pull(repo: &Path, force: bool) -> anyhow::Result<String> {
    let mut args = vec!["-c", "color.diff=always", "pull", "-v"];
    if force {
        args.push("--force");
    }
    git(repo, args)
}

/// Fetch remote refs into working copy.
pub fn fetch_remote(
    remote: &mut Remote<LocalUrl>,
    repo: &git2::Repository,
    signer: BoxedSigner,
    profile: &Profile,
) -> anyhow::Result<()> {
    let settings = transport::Settings {
        paths: profile.paths().clone(),
        signer,
    };
    remote
        .fetch(settings, repo, LocalFetchspec::Configured)?
        .for_each(drop);

    Ok(())
}

/// Clone the given repository via `git clone` into a directory.
pub fn clone(repo: &str, destination: &Path) -> Result<String, anyhow::Error> {
    git(
        Path::new("."),
        ["clone", repo, &destination.to_string_lossy()],
    )
}

/// Check that the system's git version is supported. Returns an error otherwise.
pub fn check_version() -> Result<Version, anyhow::Error> {
    let git_version = self::version()?;

    if git_version < VERSION_REQUIRED {
        anyhow::bail!("a minimum git version of {} is required", VERSION_REQUIRED);
    }
    Ok(git_version)
}

/// Parse a remote refspec into a peer id and ref.
pub fn parse_remote(refspec: &str) -> Option<(PeerId, &str)> {
    refspec
        .strip_prefix("refs/remotes/")
        .and_then(|s| s.split_once('/'))
        .and_then(|(peer, r)| PeerId::from_str(peer).ok().map(|p| (p, r)))
}

pub fn add_tag(
    repo: &git2::Repository,
    title: &str,
    patch_tag_name: &str,
    force: bool
) -> anyhow::Result<Patch> {
    let head = repo.head()?;
    let commit = head.peel(git2::ObjectType::Commit).unwrap();
    
    repo.tag(&patch_tag_name, &commit, &repo.signature()?, &title, force)?;

    Ok(Patch {
        title: title.to_string(),
        tag_name: patch_tag_name.to_string()
    })
}

pub fn push_tag(tag_name: &str, force: bool) -> anyhow::Result<String> {
    git(Path::new("."), vec!["push", if force { "--force" } else { "" }, "rad", "tag", &tag_name])
}

pub fn list_commits(
    repo: &git2::Repository,
    left: &git2::Oid,
    right: &git2::Oid,
    show_header: bool)
-> anyhow::Result<()> {
    let mut table = term::Table::default();

    let left_short = format!("{:.7}", left.to_string());
    let right_short = format!("{:.7}", right.to_string());

    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(&format!("{}..{}", left_short, right_short))?;

    if show_header {
        term::blank();
        term::info!(
            "Found {} commits.",
            term::format::highlight(revwalk.count())
        );
        term::blank();
    }
    
    let mut revwalk = repo.revwalk()?;
    revwalk.push_range(&format!("{}..{}", left_short, right_short))?;

    while let Some(rev) = revwalk.next() {
        // term::info!("{}", rev?);
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());
        // term::info!("{}\t{}", commit.id(), String::from_utf8_lossy(message));
        table.push([
            term::format::secondary(format!("{:.7}", commit.id().to_string())),
            term::format::italic(String::from_utf8_lossy(message)),
        ]);
    }
    table.render();

    Ok(())
}

fn write_gitsigner(mut w: impl io::Write, signer: &PeerId) -> io::Result<()> {
    writeln!(w, "{} {}", signer, keys::to_ssh_key(signer)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_version_ord() {
        assert!(
            Version {
                major: 2,
                minor: 34,
                patch: 1
            } > Version {
                major: 2,
                minor: 34,
                patch: 0
            }
        );
        assert!(
            Version {
                major: 2,
                minor: 24,
                patch: 12
            } < Version {
                major: 2,
                minor: 34,
                patch: 0
            }
        );
    }

    #[test]
    fn test_version_from_str() {
        assert_eq!(
            Version::from_str("git version 2.34.1\n").ok(),
            Some(Version {
                major: 2,
                minor: 34,
                patch: 1
            })
        );

        assert_eq!(
            Version::from_str("git version 2.34.1 (macOS)").ok(),
            Some(Version {
                major: 2,
                minor: 34,
                patch: 1
            })
        );

        assert_eq!(
            Version::from_str("git version 2.34").ok(),
            Some(Version {
                major: 2,
                minor: 34,
                patch: 0
            })
        );

        assert!(Version::from_str("2.34").is_err());
    }
}
