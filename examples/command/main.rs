use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
};

fn main() {
    let mut child = Command::new("npm")
        .arg("login")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let mut out = child.stdout.take().unwrap();
    let mut err: std::process::ChildStderr = child.stderr.take().unwrap();
    let mut stdin = child.stdin.take().unwrap();

    let (tx, rx) = mpsc::channel();

    let stdout_thread = thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            let size = out.read(&mut buf).unwrap();
            let str = String::from_utf8_lossy(&buf[..size]);
            println!("{str}");

            if str.ends_with("Username: ") {
                tx.send("username").unwrap();
            } else if str.ends_with("Password: ") {
                tx.send("password").unwrap();
            } else if str.ends_with("Email: (this IS public) ") {
                tx.send("email").unwrap();
            } else {
                tx.send("exit").unwrap();
                break;
            }
        }
    });

    let stdin_thread = thread::spawn(move || {
        for received in rx {
            match received {
                "username" => {
                    stdin.write_all(b"npm\n").unwrap();
                }
                "password" => {
                    stdin.write(b"test1\n").unwrap();
                }
                "email" => {
                    stdin.write(b"npm@gmail.com\n").unwrap();
                }
                _ => {
                    return;
                }
            }
        }
    });

    let stderr_thread = thread::spawn(move || loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let size = err.read(&mut buf).unwrap();
        if size == 0 {
            return;
        }
        let str = String::from_utf8(buf.to_vec()).unwrap();
        print!("{str}");

        // if str.contains(pat)
    });

    stdout_thread.join().unwrap();
    stdin_thread.join().unwrap();
    stderr_thread.join().unwrap();
}
