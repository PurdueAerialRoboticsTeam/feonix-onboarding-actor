use tokio::sync::{mpsc, oneshot};

use crate::*;

// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Brightspace {
    receiver: mpsc::Receiver<BrightspaceMessage>,

    underlings: Vec<String>,
    underling_grades: Vec<f64>,
    admin: Option<AdminHandle>,
    booster: Option<BoosterHandle>,
}

#[derive(Debug)]
enum BrightspaceMessage {
    ProcessStudentDump { students: Vec<String> },
    ProcessGradeDump { grades: Vec<f64> },
    AppendStudentCareerID,
    SetAdmin { admin_handle: AdminHandle },
    SetBooster { booster_handle: BoosterHandle },
    SendAllToAdmin { reply_to: oneshot::Sender<()> },
}

impl Brightspace {
    fn new(receiver: mpsc::Receiver<BrightspaceMessage>) -> Self {
        Brightspace {
            receiver: receiver,
            underlings: Vec::new(),
            underling_grades: Vec::new(),
            admin: None,
            booster: None,
        }
    }

    async fn handle_message(&mut self, msg: BrightspaceMessage) {
        println!(
            "[Actor] Brightspace is running handle_message() with new BrightspaceMessage: {:?}",
            msg
        );
        match msg {
            BrightspaceMessage::ProcessStudentDump { students } => {
                println!("[ACTOR] Brightspace is processing students.");
                self.underlings = students
            }
            BrightspaceMessage::ProcessGradeDump { grades } => {
                println!("[ACTOR] Brightspace is processing grades.");
                self.underling_grades = grades
            }
            BrightspaceMessage::AppendStudentCareerID => {
                self.underlings.iter_mut().for_each(|name| {
                    let career_id = name.as_str().split_once(' ').map(|(first, last)| {
                        let first_initial = first.get(..1).unwrap().to_ascii_lowercase();
                        let last_name = last.to_ascii_lowercase();
                        format!(" ({}{})", first_initial, last_name)
                    });

                    name.push_str(&career_id.unwrap());
                });
            }

            BrightspaceMessage::SetAdmin { admin_handle } => {
                println!("[ACTOR] Brightspace initialized Admin field with AdminHandle.");
                self.admin = Some(admin_handle)
            }
            BrightspaceMessage::SetBooster { booster_handle } => {
                println!("[ACTOR] Brightspace initialized Admin field with AdminHandle.");
                self.booster = Some(booster_handle)
            }
            BrightspaceMessage::SendAllToAdmin { reply_to } => {
                if let Some(ad) = &self.admin {
                    println!("[ACTOR]: Brightspace submitting all students and grades to Admin");

                    ad.submit_student_names(self.underlings.clone()).await;
                    self.booster
                        .clone()
                        .unwrap()
                        .submit_student_grades(self.underling_grades.clone())
                        .await;
                } else {
                    println!(
                        "[ACTOR]: Brightspace does not have Admin initialized so nothing happened"
                    );
                }

                let _ = reply_to.send(());
            }
        }
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //

#[derive(Clone, Debug)]
pub struct BrightspaceHandle {
    sender: mpsc::Sender<BrightspaceMessage>,
}

async fn run_brightspace_actor(mut actor: Brightspace) {
    println!("[run_brightspace_actor()]: is blocking until a BrightspaceMessage is received...");
    while let Some(msg) = actor.receiver.recv().await {
        println!(
            "\n[run_brightspace_actor()]: received a new BrightspaceMessage and calling handle_message()..."
        );
        actor.handle_message(msg).await;
    }
}

impl BrightspaceHandle {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Brightspace::new(receiver);
        tokio::spawn(run_brightspace_actor(actor));

        BrightspaceHandle { sender: sender }
    }

    pub async fn enter_students_into_brightspace(&self, students: Vec<String>) {
        let msg = BrightspaceMessage::ProcessStudentDump { students };
        let _ = self.sender.send(msg).await;
    }

    pub async fn enter_student_grades_into_brightspace(&self, grades: Vec<f64>) {
        let msg = BrightspaceMessage::ProcessGradeDump { grades };
        let _ = self.sender.send(msg).await;
    }

    pub async fn generate_and_append_student_career_id(&self) {
        let msg = BrightspaceMessage::AppendStudentCareerID;
        let _ = self.sender.send(msg).await;
    }

    pub async fn set_admin(&self, admin_handle: AdminHandle) {
        let msg = BrightspaceMessage::SetAdmin { admin_handle };
        let _ = self.sender.send(msg).await;
    }

    pub async fn set_booster(&self, booster_handle: BoosterHandle) {
        let msg = BrightspaceMessage::SetBooster { booster_handle };
        let _ = self.sender.send(msg).await;
    }

    pub async fn report_all_students_and_grades_to_admin(&self) {
        let (tx, rx) = oneshot::channel();

        let msg = BrightspaceMessage::SendAllToAdmin { reply_to: tx };
        let _ = self.sender.send(msg).await;

        let _ = rx.await;
    }
}
