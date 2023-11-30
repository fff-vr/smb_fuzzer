use std::process::Stdio;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use tokio::process::Child;
use tokio::process::Command;
use crate::config;
pub async fn execute_linux_vm(i: u32) -> Child {
    // Create a unique command for each thread
    let vm_path = format!("{}/bullseye{}.img",config::get_vm_path(),i);
    println!("{}",vm_path);
    let child = Command::new("/usr/bin/qemu-system-x86_64")
        .arg("-m")
        .arg("4G")
        .arg("-smp")
        .arg("2,sockets=2,cores=1")
        .arg("-kernel")
        .arg(config::get_kernel_path())
        .arg("-append")
        .arg("console=ttyS0 root=/dev/sda earlyprintk=serial net.ifnames=0")
        .arg("-drive")
        .arg(format!("file={}",vm_path))
        .arg("-net")
        .arg("user,host=10.0.2.10,hostfwd=tcp:0.0.0.0:10021-:22")
        .arg("-net")
        .arg("nic,model=virtio")
        .arg("-enable-kvm")
        .arg("-nographic")
        .arg("-serial")
        .arg(format!(
            "file:../workdir/test{}.txt",
            i
        ))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("fail to spawn qemu");
    child
}
