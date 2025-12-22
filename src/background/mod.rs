use tokio::time::{interval, Duration};
use crate::database::Pool;
use crate::services::heartbeat::cleanup_inactive_users;
use log::info;

// 后台清理任务
pub async fn start_cleanup_task(pool: Pool, interval_minutes: u64) {
    info!("Starting inactive user cleanup task, running every {} minutes", interval_minutes);
    
    let mut interval = interval(Duration::from_secs(interval_minutes * 60));
    
    loop {
        interval.tick().await;
        
        info!("Running inactive user cleanup task");
        
        match cleanup_inactive_users(&pool, interval_minutes as i64).await {
            Ok(_) => {
                info!("Inactive user cleanup task completed successfully");
            }
            Err(err) => {
                log::error!("Failed to run inactive user cleanup task: {}", err);
            }
        }
    }
}
