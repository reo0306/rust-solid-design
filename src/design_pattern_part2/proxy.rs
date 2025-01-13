trait Server {
    fn handle(&self, user_id: String);
}

struct RealServer;

impl RealServer {
    fn new() -> Self { Self }
}

impl Server for RealServer {
    fn handle(&self, user_id: String) {
        println!("{}の処理を実行中・・・", user_id)
    }
}

struct Proxy<S: Server> {
    server: S,
}

impl<S: Server> Proxy<S> {
    fn new(server: S) -> Proxy<S> {
        Proxy { server }
    }
    fn authorize(&self, user_id: &String) {
        let authorized_user_id = ["1", "2", "3"];

        if !authorized_user_id.contains(&user_id.as_str()) {
            panic!("操作が許可されてません");
        }
    }

    fn handle(&self, user_id: String) {
        self.authorize(&user_id);

        println!("処理を開始します");
        self.server.handle(user_id);
        println!("処理を終了しました");
    }
}

pub struct ProxyMain;

impl ProxyMain {
    pub fn index() {
        let server = RealServer::new();
        let proxy = Proxy::new(server);
        proxy.handle("1".to_string());
    }
}