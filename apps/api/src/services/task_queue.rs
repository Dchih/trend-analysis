use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CollectTaskMessage {
    pub task_id: u64,
    pub keyword_id: u64,
    pub keyword: String,
    pub platform: String,
    pub trigger: String,
    pub time_range: String,
}

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn publish(&self, task: CollectTaskMessage) -> Result<(), String>;
}

#[derive(Debug, Clone, Default)]
pub struct NoopQueue;

#[async_trait]
impl TaskQueue for NoopQueue {
    async fn publish(&self, _task: CollectTaskMessage) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct RedisTaskQueue {
    client: redis::Client,
    stream_key: String,
}

impl RedisTaskQueue {
    pub fn new(client: redis::Client, stream_key: String) -> Self {
        Self { client, stream_key }
    }
}

#[async_trait]
impl TaskQueue for RedisTaskQueue {
    async fn publish(&self, task: CollectTaskMessage) -> Result<(), String> {
        let payload = serde_json::to_string(&task).map_err(|error| error.to_string())?;
        let mut connection = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|error| error.to_string())?;

        redis::cmd("XADD")
            .arg(&self.stream_key)
            .arg("*")
            .arg("data")
            .arg(payload)
            .query_async::<String>(&mut connection)
            .await
            .map_err(|error| error.to_string())?;

        Ok(())
    }
}
