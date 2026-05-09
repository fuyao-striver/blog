use sqlx::PgPool;

pub struct UserRepo {
    db: PgPool
}

impl UserRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}