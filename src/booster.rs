use tokio::sync::{mpsc, oneshot};

use crate::*;

// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Booster {
    admin: Option<AdminHandle>,

    receiver: mpsc::Receiver<BoosterMessage>,
    amount: f64,
}

#[derive(Debug)]
enum BoosterMessage {
    BoostGrades { grades: Vec<f64> },
    SetAdmin { admin_handle: AdminHandle },
}

impl Booster {
    fn new(receiver: mpsc::Receiver<BoosterMessage>) -> Self {
        Booster {
            admin: None,
            receiver,
            amount: 100.0,
        }
    }

    async fn handle_message(&mut self, msg: BoosterMessage) {
        println!(
            "[Actor] Booster is running handle_message() with new BoosterMessage: {:?}",
            msg
        );
        match msg {
            BoosterMessage::BoostGrades { grades } => {
                let mut real_grades = vec![];
                for grade in grades {
                    real_grades.push(grade + self.amount);
                }
                self.admin
                    .clone()
                    .unwrap()
                    .submit_student_grades(real_grades)
                    .await;
            }

            BoosterMessage::SetAdmin { admin_handle } => {
                self.admin = Some(admin_handle);
            }
        };
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //

async fn run_booster_actor(mut actor: Booster) {
    while let Some(msg) = actor.receiver.recv().await {
        println!(
            "\n[run_admin_actor()]: received a new AdminMessage and calling handle_message()..."
        );
        actor.handle_message(msg).await;
    }
}

#[derive(Clone, Debug)]
pub struct BoosterHandle {
    sender: mpsc::Sender<BoosterMessage>,
}

impl BoosterHandle {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Booster::new(receiver);
        tokio::spawn(run_booster_actor(actor));

        BoosterHandle { sender: sender }
    }

    pub async fn set_admin(&self, admin: AdminHandle) {
        self.sender
            .send(BoosterMessage::SetAdmin {
                admin_handle: admin,
            })
            .await
            .unwrap();
    }

    pub async fn submit_student_grades(&self, grades: Vec<f64>) {
        self.sender
            .send(BoosterMessage::BoostGrades { grades })
            .await
            .unwrap();
    }
}
