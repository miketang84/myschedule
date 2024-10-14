// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::Scheduler;
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new scheduler
    // let mut scheduler = Scheduler::new();
    // or a scheduler with a given timezone
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    // Add some tasks to it
    // scheduler.every(Days(2)).run(|| println!("Periodic task"));
    scheduler.every(Minutes(20)).run(|| {
        println!("Periodic task");
        let output = Command::new("./target/debug/rustcc_scraper")
            .output()
            .expect("Failed to execute command");

        println!("Status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    });

    // Manually run the scheduler in an event loop
    for _ in 1.. {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
    // Or run it in a background thread
    // let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    // The scheduler stops when `thread_handle` is dropped, or `stop` is called
    // thread_handle.stop();
}
