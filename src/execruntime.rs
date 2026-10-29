use std::{process::Command, thread};

pub struct Runners {
    threads: Vec<thread::JoinHandle<()>>,
}

impl Runners {
    pub fn new() -> Self {
        Self {
            threads: Vec::new(),
        }
    }

    fn work_thread(path: String) {
        let output = Command::new(&path).output();
        if output.is_err() {
            println!("Failed to execute {}", path);
        }
    }

    pub fn spawn_bin(&mut self, path: &str) {
        let path_shared = path.to_string();
        let thrd = thread::spawn(move || Self::work_thread(path_shared));
        self.threads.push(thrd);
    }
}
