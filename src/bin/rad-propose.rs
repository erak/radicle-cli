use rad_propose::{run, HELP};
use rad_terminal::args;

fn main() {
    args::run_command::<rad_sync::Options, _>(HELP, "Proposal", run);
}