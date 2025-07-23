use crate::notify;
use cron::Schedule;
use chrono::{Timelike, Utc};
use std::str::FromStr;
use tokio::time::{sleep, sleep_until, Duration, Instant};

pub async fn startScheduler() {
    //Define Schedules
    let spiro_am = Schedule::from_str("0 9 * * * *").unwrap();
    let spiro_pm = Schedule::from_str("0 21 * * * *").unwrap();
    let estrogen = Schedule::from_str("0 15 * * 1 *").unwrap();
    
    loop {
        let now = Utc::now();
        let next_am = spiro_am.upcoming(Utc).next().unwrap();
        let next_pm = spiro_pm.upcoming(Utc).next().unwrap();
        let next_estrogen = estrogen.upcoming(Utc).next().unwrap();
        
        let next_event = *[next_am, next_pm, next_estrogen].iter().min().unwrap();
        
        let delay = (next_event - now).to_std().unwrap_or(Duration::from_secs(60));
        sleep_until(Instant::now() + delay).await;
        
        let hour = next_event.hour();
        let title;
        let body;
        
        if hour == 9 {
            title = "Morning Spiro";
            body = "Take morning dose of Spironolactone"
        } else if hour == 21 {
            title = "Evening Spiro";
            body = "Take Evening dose of Spironolactone"
        } else {
            title = "Injection Needed";
            body = "Weekly Injection Dose is due today";
        }
        notify::send(title, body);
    }
}