use clokwerk::{Scheduler, TimeUnits};
use notifica::notify;
use std::error::Error;

fn main() {
    Scheduler::new()
        .every(10.seconds())
        .run(|| notify("Notification", "Check your posture!"));
}
