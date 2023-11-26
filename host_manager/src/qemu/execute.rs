use std::process::{Child, Command};
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn wait_linux_vm(mut child: Child, receiver: mpsc::Receiver<()>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            // Check if a termination signal has been received
            if receiver.try_recv().is_ok() {
                // Termination signal received, kill the child process
                println!("recv signal from master");
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

pub fn execute_linux_vm() -> (Vec<Sender<()>>, Vec<JoinHandle<()>>) {
    let mut children = Vec::new();
    let mut senders = Vec::new();
    for i in 0..1 {
        let (sender, receiver) = mpsc::channel();

        // Create a unique command for each thread
        let command = format!("/usr/bin/qemu-system-x86_64 -m 16G -smp 12,sockets=12,cores=1 -kernel /home/jjy/target/linux//arch/x86/boot/bzImage -append \"console=ttyS0 root=/dev/sda earlyprintk=serial net.ifnames=0\" -hda /home/jjy/tools/smb_fuzzer/vm/bullseye.img -net user,host=10.0.2.10,hostfwd=tcp:0.0.0.0:10021-:22,hostfwd=tcp:0.0.0.0:10022-:8080,hostfwd=tcp:0.0.0.0:10023-:8081 -net nic,model=e1000 -enable-kvm -nographic -serial file://home/jjy/tools/smb_fuzzer/workdir/test{}.txt",i);
        let child = Command::new("/bin/bash")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Failed to execute command");
        let child_thread = wait_linux_vm(child, receiver);
        children.push(child_thread);
        senders.push(sender);
    }
    (senders, children)
}
