//! `onboard-cmd` subcommand

use crate::{entrypoint, node::client, node::node::Node, prelude::app_config};
use abscissa_core::{Command, Options, Runnable};

/// `onboard-cmd` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct OnboardCmd {
    #[options(help = "advance to all next states without supervision")]
    autopilot: bool,

    #[options(help = "advance to the next state")]
    next: bool,

    #[options(help = "attempt to trigger actions at every state")]
    trigger_actions: bool,
}

impl Runnable for OnboardCmd {
    /// Start the application.
    fn run(&self) {
        let args = entrypoint::get_args();
        let mut cfg = app_config().clone();
        let client = client::pick_client(args.swarm_path, &mut cfg).unwrap().0;
        if !self.trigger_actions {
            println!("You can pass --trigger-actions or -t to attempt the next transition\n")
        }
        // let mut host = transitions::HostState::init(client, cfg);
        let mut node = Node::new(client, cfg);
        let state = node.node_maybe_advance(self.trigger_actions);

        println!(
            "\nNode state at exit: {:?}\nMiner state: {:?}",
            &state.vitals.host_state.node_state,
            &state.vitals.host_state.miner_state,
            // host.get_next_action()
        );
    }
}

// pub fn advance(client: DiemClient, trigger_actions: bool, cfg) {

// }
