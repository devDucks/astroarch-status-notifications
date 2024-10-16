use notify_rust::Notification;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NotificationMessage {
    summary: String,
    body: String,
}

fn main() {
    match ureq::get("http://astromatto.com:9000/notifications.json").call() {
        Ok(response) => {
            if let Ok(messages) = response.into_json::<Vec<NotificationMessage>>() {
                for message in messages.into_iter() {
                    let _ = Notification::new()
                        .appname("AstroArch news")
                        .summary(message.summary.as_ref())
                        .body(message.body.as_ref())
                        .icon("astroarch")
                        .timeout(0)
                        .show()
                        .unwrap();
                }
            }
        }
        Err(_) => (),
    }
}
