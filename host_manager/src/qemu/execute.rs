use std::process::Stdio;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;
use tokio::process::Child;
use tokio::process::Command;

pub async fn execute_linux_vm(i: u32) -> Child {
    // Create a unique command for each thread
    let vm_path = format!("../vm/bullseye{}.img",i);
    let child = Command::new("/usr/bin/qemu-system-x86_64")
        .arg("-m")
        .arg("4G")
        .arg("-smp")
        .arg("2,sockets=2,cores=1")
        .arg("-kernel")
        .arg("/home/jjy/target/linux//arch/x86/boot/bzImage")
        .arg("-append")
        .arg("console=ttyS0 root=/dev/sda earlyprintk=serial net.ifnames=0")
        .arg("-hda")
        .arg(vm_path)
        .arg("-net")
        .arg("user,host=10.0.2.10")
        .arg("-net")
        .arg("nic,model=e1000")
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
