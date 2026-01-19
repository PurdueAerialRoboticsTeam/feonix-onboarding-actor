use tokio::sync::{mpsc, oneshot};

use crate::*;

// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Booster {
    // TODO
}

#[derive(Debug)]
enum BoosterMessage {
	  // TODO
}

impl Booster {
	fn new(receiver: mpsc::Receiver<BoosterMessage>) -> Self {
        // TODO
        todo!()
    }

    async fn handle_message(&mut self, msg: BoosterMessage) {
        println!("[Actor] Booster is running handle_message() with new BoosterMessage: {:?}", msg);
        match msg {
            // TODO
        };
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //

async fn run_booster_actor(mut actor: Booster) {
    // TODO
    todo!()
}

#[derive(Clone, Debug)]
pub struct BoosterHandle {
    sender: mpsc::Sender<BoosterMessage>,
}

impl BoosterHandle {
    pub async fn new() -> Self {
        // TODO
        todo!()
    }

    // TODO
}
