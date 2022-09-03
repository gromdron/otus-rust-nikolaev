use thiserror::Error;

trait Command {
    fn execute(&self) -> Result<(), CommandError>;
}

#[derive(Error, Debug)]
enum CommandError {
    #[error("Error `{0}`")]
    Text(String),
}

struct CommandQueue {
    cmds: Vec<Box<dyn Command>>,
}

impl CommandQueue {
    fn add(&mut self, cmd: Box<dyn Command>) {
        self.cmds.push(cmd);
    }

    fn execute(&self) {
        for cmd in self.cmds.iter() {
            if let Err(x) = cmd.execute() {
                println!("{:?}", x.to_string());
            }
        }
    }
}

struct SimpleCommand;

impl Command for SimpleCommand {
    fn execute(&self) -> Result<(), CommandError> {
        println!("Execute command one");
        Ok(())
    }
}

struct BrokenCommand;

impl Command for BrokenCommand {
    fn execute(&self) -> Result<(), CommandError> {
        Err(CommandError::Text("Success".to_string()))
    }
}

fn main() {
    let mut queue = CommandQueue { cmds: Vec::new() };

    queue.add(Box::new(SimpleCommand));
    queue.add(Box::new(SimpleCommand));
    queue.add(Box::new(BrokenCommand));

    queue.execute();
}
