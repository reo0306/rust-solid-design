trait MessageApp {
    fn send(&self);
}

struct Line;

impl Line {
    fn new() -> Self {
        Self
    }
}

impl MessageApp for Line {
    fn send(&self) {
        println!("LINEでメッセージを送信しました")
    }
}

struct Twitter;

impl Twitter {
    fn new() -> Self {
        Self
    }
}

impl MessageApp for Twitter {
    fn send(&self) {
        println!("Twitterでメッセージを送信しました")
    }
}

struct Facebook;

impl Facebook {
    fn new() -> Self {
        Self
    }
}

impl MessageApp for Facebook {
    fn send(&self) {
        println!("Facebookでメッセージを送信しました")
    }
}

trait Os {
    fn send_message(&self);
}

struct Ios<T> {
    app: Option<T>,
}

impl<T: MessageApp> Ios<T> {
    fn new(app: T) -> Self {
        Self { app: Some(app) }
    }
}

impl<T: MessageApp> Os for Ios<T> {
    fn send_message(&self) {
        println!("iOSでメッセージ送信");

        if let Some(app) = &self.app {
            app.send();
        } else {
            panic!("アプリが指定されていません。");
        }
    }
}

struct Android<T> {
    app: Option<T>,
}

impl<T: MessageApp> Android<T> {
    fn new(app: T) -> Self {
        Self { app: Some(app) }
    }
}

impl<T: MessageApp> Os for Android<T> {
    fn send_message(&self) {
        println!("Androidでメッセージ送信");

        if let Some(app) = &self.app {
            app.send();
        } else {
            panic!("アプリが指定されていません。");
        }
    }
}

pub struct BridgeMain;

impl BridgeMain {
    pub fn index() {
        let line = Line::new();
        let twitter = Twitter::new();
        let facebook = Facebook::new();

        let ios = Ios::new(line);
        let android = Android::new(facebook);

        ios.send_message();

        android.send_message();
    }
}