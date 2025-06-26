use notify_rust::Notification;

mod notify;
mod schedule;

#[tokio::main]
async fn main() {
    
    Notification::new()
        .summary("Spiro")
        .body("Take morning dose of Spironolactone")
        .show()
        .unwrap();
    schedule::startScheduler().await;
}