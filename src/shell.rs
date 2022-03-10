use std::io::Result;
use std::{
    io::{stdin, stdout, Write},
    process::{Command, ExitStatus},
};

pub enum CLIStatus {
    RUNNING,
    EXIT,
}

pub enum ExecuteStatus {
    SUCCESS,
    FAIL,
}

pub struct Shell {
    pub line: String,
    pub args: Vec<String>,
    pub status: CLIStatus,
}

impl Shell {
    pub fn start() -> Shell {
        Shell {
            line: "".to_string(),
            args: Vec::new(),
            status: CLIStatus::RUNNING,
        }
    }

    pub fn shy_loop(&mut self) {
        let testargs = ["", ""];
        loop {
            print!("> ");
            self.line = self.read_line();
            self.execute_cmd("nvim", &testargs);
            println!("{}", self.line);
        }
    }

    pub fn read_line(&self) -> String {
        let mut line_read = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut line_read)
            .expect("Shyll: Error Parsing Line");
        if let Some('\n') = line_read.chars().next_back() {
            line_read.pop();
        }
        if let Some('\r') = line_read.chars().next_back() {
            line_read.pop();
        }
        line_read
    }

    //    pub fn split_line() -> Vec<String> {
    //
    //    }
    //
    pub fn execute_cmd(&self, exe: &str, args: &[&str]) -> Result<ExitStatus> {
        Command::new(exe).args(args).spawn()?.wait()
    }
}
