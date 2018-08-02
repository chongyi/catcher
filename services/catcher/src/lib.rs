extern crate reqwest;

pub enum Task {
    SingleTarget(TaskTarget),
    MultiTarget(Vec<TaskTarget>),
}

pub struct TaskTarget {
    url: String
}

impl Task {
    pub fn execute(&self) {
        match self {
            Task::SingleTarget(target) => {
                let text = reqwest::get(&target.url).unwrap().text().unwrap();
                println!("body: {}", text);

            },
            _ => ()
        }
    }
}