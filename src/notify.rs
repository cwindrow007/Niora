use notify_rust::Notification;

pub fn send(title: &str, body: &str) {
    Notification::new()
        .summary(title)
        .body(body)
        .show()
        .unwrap();
}