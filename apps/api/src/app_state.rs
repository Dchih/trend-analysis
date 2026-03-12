use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::services::task_queue::{NoopQueue, TaskQueue};

#[derive(Clone)]
pub struct AppState {
    queue: Arc<dyn TaskQueue>,
    next_keyword_id: Arc<AtomicU64>,
    next_task_id: Arc<AtomicU64>,
}

impl AppState {
    pub fn new(queue: Arc<dyn TaskQueue>) -> Self {
        Self {
            queue,
            next_keyword_id: Arc::new(AtomicU64::new(1)),
            next_task_id: Arc::new(AtomicU64::new(1)),
        }
    }

    pub fn queue(&self) -> Arc<dyn TaskQueue> {
        Arc::clone(&self.queue)
    }

    pub fn next_keyword_id(&self) -> u64 {
        self.next_keyword_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn next_task_id(&self) -> u64 {
        self.next_task_id.fetch_add(1, Ordering::SeqCst)
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new(Arc::new(NoopQueue))
    }
}
