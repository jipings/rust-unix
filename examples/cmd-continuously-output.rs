use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind};

/**
 * 连续处理，子进程的输出
 * 
 * 在运行外部命令，并处理 stdout， stdout 的处理会在外部 Command 完成之后执行。
 * 调用 Stdio::piped 创建一个管道，并在 BufReader 每次更新，都读取 stdout (连续)
 */
fn main() -> Result<(), Error> {
    let stdout = Command::new("journalctl")
    .stdout(Stdio::piped())
    .spawn()?
    .stdout
    .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}