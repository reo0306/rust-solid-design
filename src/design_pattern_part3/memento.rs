use std::fmt::Debug;
use chrono::{Utc, DateTime};

trait Memento: Debug {
    fn get_memo(&self) -> &str;
}

#[derive(Debug)]
struct ConcreteMemento {
    memo: String,
    date: DateTime<Utc>,
}

impl ConcreteMemento {
    fn new(memo: String) -> Self {
        Self {
            memo,
            date: Utc::now(),
        }
    }
}

impl Memento for ConcreteMemento {
    fn get_memo(&self) -> &str {
        &self.memo
    }
}

#[derive(Debug, Clone)]
struct Notepad {
    memo: String,
}

impl Notepad {
    fn new(memo: String) -> Self {
        Self { memo }
    }

    fn get_memo(&self) -> &str {
        &self.memo
    }

    fn add_memo(&mut self, memo: String) {
        self.memo = memo;
    }

    fn save(&self) -> impl Memento {
        println!("メモを保存しました");
        ConcreteMemento::new(self.get_memo().to_string())
    }

    fn restore(&mut self, memento: Box<dyn Memento>) {
        self.add_memo(memento.get_memo().to_string());
    }
}

#[derive(Debug)]
struct Caretaker {
    notepad: Notepad,
    mementos: Vec<Box<dyn Memento>>,
}

impl Caretaker {
    fn new(notepad: Notepad) -> Self {
        Self {
            notepad,
            mementos: Vec::new(),
        }
    }

    fn backup(&mut self) {
        self.mementos.push(Box::new(self.notepad.save()));
    }

    fn undo(&mut self) {
        if self.mementos.len() == 0 {
            println!("スナップショットが存在しません");
            return;
        }

        let memento = self.mementos.pop();
        if let Some(me) = memento {
            self.notepad.restore(me);
        }
        //self.notepad.restore(memento);
    }

    fn show_history(&self) {
        for memento in &self.mementos {
            println!("{:?}", memento);
        }
    }
}

pub struct MementoMain;

impl MementoMain{
    pub fn index() {
        let mut notepad = Notepad::new("first memo".to_string());
        let mut caretaker = Caretaker::new(notepad.clone());
        caretaker.backup();

        notepad.add_memo("second memo".to_string());
        caretaker.backup();

        notepad.add_memo("third memo".to_string());
        caretaker.backup();
        println!("{:?}", notepad.get_memo());
        caretaker.show_history();

        caretaker.undo();
        println!("{:?}", notepad.get_memo());
        caretaker.undo();
        println!("{:?}", notepad.get_memo());
        caretaker.undo();
        println!("{:?}", notepad.get_memo());
        caretaker.undo();
        println!("{:?}", notepad.get_memo());
        caretaker.show_history();
    }
}