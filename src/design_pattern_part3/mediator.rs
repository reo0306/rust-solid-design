use std::cell::RefCell;
use std::rc::Rc;

trait Mediator {
    fn register_user(&mut self, user: Rc<dyn User>);
    fn send_message(&self, msg: &str, send_user: Rc<dyn User>);
}

struct ChatRoom {
    members: Vec<Rc<dyn User>>,
}

impl ChatRoom {
    fn new() -> Self {
        Self {
            members: Vec::new(),
        }
    }
}

impl Mediator for ChatRoom {
    fn register_user(&mut self, user: Rc<dyn User>) {
        self.members.push(user);
    }

    fn send_message(&self, msg: &str, send_user: Rc<dyn User>) {
        for member in &self.members {
            if !Rc::ptr_eq(member, &send_user) {
                member.receive(msg);
            }
        }
    }
}

trait User {
    fn send(&self, msg: &str);
    fn receive(&self, msg: &str);
}

#[derive(Clone)]
struct ChatUser {
    mediator: Rc<RefCell<dyn Mediator>>,
    name: String,
}

impl ChatUser {
    fn new(mediator: Rc<RefCell<dyn Mediator>>, name: String) -> Self {
        Self { mediator, name }
    }
}

impl User for ChatUser {
    fn send(&self, msg: &str) {
        println!("{} -> メッセージ送信", self.name);
        self.mediator.borrow().send_message(&format!("{}, from {}", msg, self.name), Rc::new(self.clone()));
    }

    fn receive(&self, msg: &str) {
        println!("{} -> メッセージ受信: {}", self.name, msg);
    }
}

pub struct MediatorMain;

impl MediatorMain {
    pub fn index() {
        let chat_room = Rc::new(RefCell::new(ChatRoom::new()));

        let yamada = Rc::new(ChatUser::new(chat_room.clone(), "yamada".to_string()));
        let tanaka = Rc::new(ChatUser::new(chat_room.clone(), "tanaka".to_string()));
        let suzuki = Rc::new(ChatUser::new(chat_room.clone(), "suzuki".to_string()));

        chat_room.borrow_mut().register_user(yamada.clone() as Rc<dyn User>);
        chat_room.borrow_mut().register_user(suzuki.clone() as Rc<dyn User>);
        chat_room.borrow_mut().register_user(tanaka.clone() as Rc<dyn User>);

        yamada.send("こんにちは");
        tanaka.send("こんばんわ");
    }
}