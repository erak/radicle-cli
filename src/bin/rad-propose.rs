use rad_propose::{run, HELP};
use rad_terminal::args;

fn main() {
    args::run_command::<rad_propose::Options, _>(HELP, "Proposal", run);
}