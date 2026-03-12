use std::sync::Arc;

use crate::{
    repositories::keywords::{InMemoryKeywordRepository, KeywordRepository},
    services::task_queue::{NoopQueue, TaskQueue},
};

#[derive(Clone)]
pub struct AppState {
    repository: Arc<dyn KeywordRepository>,
    queue: Arc<dyn TaskQueue>,
}

impl AppState {
    pub fn new(repository: Arc<dyn KeywordRepository>, queue: Arc<dyn TaskQueue>) -> Self {
        Self {
            repository,
            queue,
        }
    }

    pub fn repository(&self) -> Arc<dyn KeywordRepository> {
        Arc::clone(&self.repository)
    }

    pub fn queue(&self) -> Arc<dyn TaskQueue> {
        Arc::clone(&self.queue)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(
            Arc::new(InMemoryKeywordRepository::default()),
            Arc::new(NoopQueue),
        )
    }
}
