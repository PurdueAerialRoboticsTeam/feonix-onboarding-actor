use tokio::sync::{mpsc, oneshot};



// ##################################################### //
// ################### ACTOR BACKEND ################### //
// ##################################################### //

struct Admin {
	receiver: mpsc::Receiver<AdminMessage>,

    underlings: Vec<String>,
	underling_grades: Vec<f64>,
}

#[derive(Debug)]
enum AdminMessage {
	ProcessStudentDump { students: Vec<String> },
	ProcessGradeDump { grades: Vec<f64> },
    CountNumberFailingStudents { reply_to: oneshot::Sender<usize> },
    GetAllStudentGrades { reply_to: oneshot::Sender<Vec<f64>> },
    GetAllStudentNames { reply_to: oneshot::Sender<Vec<String>> },
}

impl Admin {
	fn new(receiver: mpsc::Receiver<AdminMessage>) -> Self {
        Admin {
            receiver: receiver,
            underlings: Vec::new(),
            underling_grades: Vec::new(),
        }
    }

    async fn handle_message(&mut self, msg: AdminMessage) {
		println!("[Actor] Admin is running handle_message() with new AdminMessage: {:?}", msg);
		match msg {
            AdminMessage::ProcessStudentDump { students } => self.underlings = students,
            AdminMessage::ProcessGradeDump { grades } => self.underling_grades = grades,
            AdminMessage::CountNumberFailingStudents { reply_to } => {
                let count_failed = self.underling_grades
                    .iter()
                    .filter(|grade| **grade < 60.0)
                    .count();
                
                let _ = reply_to.send(count_failed);
            },
            AdminMessage::GetAllStudentNames { reply_to } => {
                let _ = reply_to.send(self.underlings.clone());
            }

            AdminMessage::GetAllStudentGrades { reply_to } => {
                let _ = reply_to.send(self.underling_grades.clone());
            }
        }
    }
}

// ###################################################### //
// ################### ACTOR FRONTEND ################### //
// ###################################################### //


#[derive(Clone, Debug)]
pub struct AdminHandle {
    sender: mpsc::Sender<AdminMessage>,
}

async fn run_admin_actor(mut actor: Admin) {
    println!("[run_admin_actor()]: is blocking until a AdminMessage is received...");
    while let Some(msg) = actor.receiver.recv().await {
        println!("\n[run_admin_actor()]: received a new AdminMessage and calling handle_message()...");
        actor.handle_message(msg).await;
    }
}

impl AdminHandle {
    pub async fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Admin::new(receiver);
        tokio::spawn(run_admin_actor(actor));

        AdminHandle {
			sender: sender
		}
    }

	pub async fn submit_student_names(&self, students: Vec<String>) {
        let msg = AdminMessage::ProcessStudentDump { students };
		let _ = self.sender.send(msg).await;
	}

	pub async fn submit_student_grades(&self, grades: Vec<f64>) {
        let msg = AdminMessage::ProcessGradeDump { grades };
		let _ = self.sender.send(msg).await;
	}

    pub async fn count_number_of_failing_students(&self) -> usize {
        let (tx, rx) = oneshot::channel();
        
        let msg = AdminMessage::CountNumberFailingStudents { reply_to: tx };
        let _ = self.sender.send(msg).await;
        
        rx.await.unwrap_or(0)
    }

    pub async fn get_all_student_names(&self) -> Vec<String> {
        let (tx, rx) = oneshot::channel();

        let msg = AdminMessage::GetAllStudentNames { reply_to: tx };
        let _ = self.sender.send(msg).await;

        rx.await.unwrap_or_default()
    }

    pub async fn get_all_student_grades(&self) -> Vec<f64> {
        let (tx, rx) = oneshot::channel();

        let msg = AdminMessage::GetAllStudentGrades { reply_to: tx };
        let _ = self.sender.send(msg).await;

        rx.await.unwrap_or_default()
    }
}
