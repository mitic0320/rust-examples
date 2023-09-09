use async_std::{
    io::{ReadExt, WriteExt},
    process::{Command, Stdio},
};
use futures::{select, FutureExt};

#[async_std::main]
async fn main() {
    let mut child = Command::new("npm")
        .arg("login")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();

    loop {
        let mut buf: [u8; 1024] = [0; 1024];
        let mut buf2: [u8; 1024] = [0; 1024];

        select! {
          size = stdout.read(&mut buf).fuse() => {
            let size = size.unwrap();
            let output = String::from_utf8_lossy(&buf[..size]);
            println!("{output}");
            if output.ends_with("Username: ") {
                stdin.write(b"npm_async\n").await.unwrap();
            } else if output.ends_with("Password: ") {
                stdin.write(b"npm_async\n").await.unwrap();
            } else if output.ends_with("Email: (this IS public) ") {
                stdin.write(b"npm_async@gmail.com\n").await.unwrap();
            } else {
                break;
            }
          },
          size = stderr.read(&mut buf2).fuse() => {
            let size = size.unwrap();
            print!("{}", String::from_utf8_lossy(&buf2[..size]));
          }
          default => {
            continue;
          },
        }
    }
}
