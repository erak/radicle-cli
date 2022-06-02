pub struct Identity {
    pub profile: String,
    pub name: String,
    pub urn: String,
    pub peer_id: String,
    pub key_hash: String,
    pub key_full: String,
}

pub struct Project {
    pub name: String,
    pub urn: String,
    pub issues: (usize, usize),
    pub patches: (usize, usize),
    pub issue_list: Vec<Issue>,
}

#[derive(Clone)]
pub struct Issue {
    pub title: String,
    pub author: String,
}

pub struct Context {
    pub identity: Option<Identity>,
    pub project: Option<Project>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            identity: Some(Identity {
                profile: "68ddefe3-81dc-431d-b75b-456416c63a4f".to_owned(),
                name: "test".to_owned(),
                urn: "rad:git:hnrkqtre3dtutrqkeebcpwwwtxjtonaqp9kuy".to_owned(),
                peer_id: "hynp6z36munjbjua6ek8kyztr43dotfopo9pwjmnqifcrrpueyihqo".to_owned(),
                key_hash: "SHA256:ztsGBnbAyhDaoXGAJhw8+EG255MJSNyiWlLCiBfo46o".to_owned(),
                key_full: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIJvr58uYkhTPHkKOoF4k1kcIlg2H20SsTqlYQjZoBXjo".to_owned(),
            }),
            project: Some(Project {
                name: "demo".to_owned(),
                urn: "rad:git:hnrkcnewg4ekq1d18s1qzit4tqshkhqnqnefy".to_owned(),
                issues: (16, 87),
                patches: (5, 67),
                issue_list: vec![
                    Issue {
                        title: "Issue #1".to_owned(),
                        author: "sebastinez".to_owned(),
                    },
                    Issue {
                        title: "Issue #2".to_owned(),
                        author: "cloudhead".to_owned(),
                    },
                    Issue {
                        title: "Issue #3".to_owned(),
                        author: "erikli".to_owned(),
                    },
                ],
            }),
        }
    }
}
