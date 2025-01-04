use std::sync::Arc;
use anyhow::Result;

trait IUserRepository: Send + Sync + 'static {
    fn create(&self) -> Result<()>;
    fn find_by_id(&self, id: String) -> Result<String>;
}

struct UserRdbRepository;

impl UserRdbRepository {
    fn new() -> Self {
        Self {}
    }
}

impl IUserRepository for UserRdbRepository {
    fn create(&self) -> Result<()> {
        println!("RDBにUserを登録");
        Ok(())
    }

    fn find_by_id(&self, id: String) -> Result<String> {
        println!("ID: {}のユーザーを検索", id);
        Ok(id)
    }
}

trait IUserService {
    fn create(&self) -> Result<()>;
    fn find_by_id(&self, id: String) -> Result<String>;
}

struct UserService {
    repository: Arc<dyn IUserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn IUserRepository>) -> Self {
        Self { repository }
    }
}

impl IUserService for UserService {
    fn create(&self) -> Result<()> {
        self.repository.create()
    }

    fn find_by_id(&self, id: String) -> Result<String> {
        self.repository.find_by_id(id)
    }
}


pub struct UserController {
    user_service: UserService,
}

impl UserController {
    fn new() -> Self {
        let user_service = UserService::new(Arc::new(UserRdbRepository::new()));

        Self {
            user_service,
        }
    }

    fn create(&self) -> Result<()> {
        self.user_service.create()
    }

    fn find_by_id(&self, id: String) -> Result<String> {
        self.user_service.find_by_id(id)
    }
}

pub struct DependMain;

impl DependMain {
    pub fn index() {
        let user_controller = UserController::new();

        let _ = user_controller.create();
        let _ = user_controller.find_by_id("1".to_string());
    }
}