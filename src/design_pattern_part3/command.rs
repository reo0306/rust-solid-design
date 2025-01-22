#[derive(Clone)]
struct File {
    name: String,
}

impl File {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn open(&self) {
        println!("{}が開かれました", self.name)
    }

    fn compress(&self) {
        println!("{}が圧縮されました", self.name)
    }

    fn close(&self) {
        println!("{}が閉じられました", self.name)
    }
}

trait Command {
    fn execute(&self);
}

struct OpenCommand {
    file: File,
}

impl OpenCommand {
    fn new(file: File) -> Self {
        Self { file }
    }
}

impl Command for OpenCommand {
    fn execute(&self) {
        self.file.open()
    }
}

struct CompressCommand {
    file: File,
}

impl CompressCommand {
    fn new(file: File) -> Self {
        Self { file }
    }
}

impl Command for CompressCommand {
    fn execute(&self) {
        self.file.compress()
    }
}

struct CloseCommand {
    file: File,
}

impl CloseCommand {
    fn new(file: File) -> Self {
        Self { file }
    }
}

impl Command for CloseCommand {
    fn execute(&self) {
        self.file.close()
    }
}

struct Queue {
    commands: Vec<Box<dyn Command>>,
}

impl Queue {
    fn new() -> Self {
        Self { commands: Vec::new() }
    }

    fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }

    fn execute_command(&self) {
        for command in &self.commands {
            command.execute();
        }
    }
}

pub struct CommandMain;

impl CommandMain {
    pub fn index() {
        let file = File::new("command.py".to_string());
        let mut queue = Queue::new();

        queue.add_command(Box::new(OpenCommand::new(file.clone())));
        queue.add_command(Box::new(CompressCommand::new(file.clone())));
        queue.add_command(Box::new(CloseCommand::new(file.clone())));

        queue.execute_command();
    }
}