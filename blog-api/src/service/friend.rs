use crate::entity::friend::Friend;
use crate::modal::api_result::PageResult;
use crate::modal::request::PageReq;
use crate::registry::AppRegistry;
use crate::repo::friend::FriendRepo;
use std::sync::Arc;

#[derive(Clone)]
pub struct FriendService {
    friend_repo: Arc<FriendRepo>,
}

impl FriendService {
    pub fn new(registry: &AppRegistry) -> Self {
        Self {
            friend_repo: Arc::clone(&registry.friend_repo),
        }
    }

    pub async fn get_friend(&self, page_req: PageReq) -> anyhow::Result<PageResult<Friend>> {
        let result = self.friend_repo.get_friend(page_req).await?;
        Ok(PageResult {
            record_list: result.0,
            count: result.1,
        })
    }
}
