pub struct Identity {
    pub profile: String,
}

pub struct Project {
    pub name: String,
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
            }),
            project: Some(Project {
                name: "demo".to_owned(),
            }),
        }
    }
}
