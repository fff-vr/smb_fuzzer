use std::process::{Command, Child};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn execute_command(mut child: Child, receiver: mpsc::Receiver<()>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            // Check if a termination signal has been received
            if receiver.try_recv().is_ok() {
                // Termination signal received, kill the child process
                match child.kill() {
                    Ok(_) => println!("Child process killed."),
                    Err(e) => println!("Failed to kill child process: {}", e),
                }
                return;
            }

            // Place for additional processing or process status checks
            // ...

            // Sleep to reduce CPU usage
            thread::sleep(Duration::from_millis(100));
        }
    })
}

fn main() {
    let mut children = Vec::new();
    let mut senders = Vec::new();

    for i in 0..12 {
        let (sender, receiver) = mpsc::channel();

        // Create a unique command for each thread
        let command = format!("your_command_here {}", i);
        let child = Command::new(command)
            .spawn()
            .expect("Failed to execute command");

        let child_thread = execute_command(child, receiver);
        children.push(child_thread);
        senders.push(sender);
    }

    // Main thread operations
    // ...

    // Send a termination signal to each thread's command
    for sender in senders {
        sender.send(()).unwrap();
    }

    // Wait for all child threads to finish
    for child in children {
        child.join().unwrap();
    }
}

