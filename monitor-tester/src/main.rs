use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let pump_bot = monitor_bot::PumpBot::create();
    let pump_bot_arc = Arc::new(pump_bot);
    monitor_bot::PumpBot::run_entry_update(pump_bot_arc).await;
    Ok(())
}
