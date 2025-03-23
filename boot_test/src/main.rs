use std::{
  io::Read,
  os::fd::AsRawFd,
  process::{Command, Stdio},
  time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(1);

fn main() {
  let args = std::env::args().collect::<Vec<String>>();

  let mut exec = Command::new(args[1].clone())
    .args(&args[2..])
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute qemu");

  let mut stdout = exec.stdout.take().expect("failed to get stdout");
  let start = Instant::now();

  // Don't block, so that the below loop works.
  let mut nonblocking = true as libc::c_int;
  unsafe { libc::ioctl(stdout.as_raw_fd(), libc::FIONBIO, &mut nonblocking) };

  let mut buf = String::new();

  while start.elapsed() < TIMEOUT {
    let mut b = [0; 1024];
    match stdout.read(&mut b) {
      Ok(0) => break,
      Ok(n) => buf.push_str(&String::from_utf8_lossy(&b[..n])),
      Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
      Err(e) => panic!("failed to read stdout: {}", e),
    }

    if buf.contains("HELLO WORLD!") {
      return;
    }
  }

  match exec.kill() {
    Ok(_) => (),
    Err(e) => panic!("failed to kill qemu: {}", e),
  }

  panic!("failed to match the search string. output:\n{}", buf);
}
