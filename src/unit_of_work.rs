use crate::repositories::users::UserRepository;
use crate::repositories::sessions::SessionRepository;
use crate::repositories::orders::OrderRepository;
use crate::database::db_connect::connect_to_db;

pub struct UnitOfWork {
    pub user_repository: UserRepository,
    pub session_repository: SessionRepository,
    pub orders_repository: OrderRepository,
}


impl UnitOfWork {
    pub async fn init() -> anyhow::Result<Self> {
    let user_repository = UserRepository {
        db: connect_to_db().await.unwrap(),
    };
    let session_repository = SessionRepository {
        db: connect_to_db().await.unwrap(),
    };

    let orders_repository = OrderRepository::new(connect_to_db().await.unwrap());
    anyhow::Ok(Self {
        user_repository,
        session_repository,
        orders_repository,
    })
}

}

