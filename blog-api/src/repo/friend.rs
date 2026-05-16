use crate::entity::friend::Friend;
use crate::modal::request::PageReq;
use crate::modal::request::friend_req::NewFriend;
use chrono::Utc;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub struct FriendRepo {
    db: PgPool,
}

impl FriendRepo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_friend(&self, page_req: PageReq) -> anyhow::Result<(Vec<Friend>, i64)> {
        let offset = (page_req.current - 1) * page_req.size;

        let count_sql = r#"SELECT COUNT(*) FROM "t_friend""#;
        let base_sql = r#"SELECT * FROM "t_friend""#;

        let mut count_builder = QueryBuilder::<Postgres>::new(count_sql);
        let mut data_builder = QueryBuilder::<Postgres>::new(base_sql);

        if let Some(keyword) = &page_req.keyword {
            if !keyword.is_empty() {
                let pattern = format!("%{}%", keyword);
                count_builder
                    .push(" WHERE \"name\" LIKE ")
                    .push_bind(pattern.clone());
                data_builder
                    .push(" WHERE \"name\" LIKE ")
                    .push_bind(pattern);
            }
        }

        // 直接查询，不需要判断
        let total = count_builder
            .build_query_scalar()
            .fetch_one(&self.db)
            .await?;

        data_builder
            .push(r#" ORDER BY "create_time" DESC"#)
            .push(" LIMIT ")
            .push_bind(page_req.size)
            .push(" OFFSET ")
            .push_bind(offset);

        let friends = data_builder
            .build_query_as::<Friend>()
            .fetch_all(&self.db)
            .await?;
        Ok((friends, total))
    }

    /// 新增友链
    pub async fn add_friend(pool: &PgPool, friend: NewFriend) -> anyhow::Result<bool> {
        let now = Utc::now().naive_utc();

        let friend = sqlx::query(
            r#"
            INSERT INTO "t_friend" ("name", "color", "avatar", "url", "introduction", "create_time")
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(friend.name)
        .bind(friend.color)
        .bind(friend.avatar)
        .bind(friend.url)
        .bind(friend.introduction)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(friend.rows_affected() > 0)
    }
}
