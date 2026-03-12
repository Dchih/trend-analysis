#[derive(Debug, Clone)]
pub struct CollectTaskMessage {
    pub task_id: u64,
    pub keyword_id: u64,
    pub keyword: String,
    pub platform: &'static str,
    pub trigger: &'static str,
}

pub trait TaskQueue: Send + Sync {
    fn publish(&self, task: CollectTaskMessage) -> Result<(), String>;
}

#[derive(Debug, Clone, Default)]
pub struct NoopQueue;

impl TaskQueue for NoopQueue {
    fn publish(&self, _task: CollectTaskMessage) -> Result<(), String> {
        Ok(())
    }
}
