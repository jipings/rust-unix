use std::fs::File;
use std::io::Error;
use std::process::{Command, Stdio};

/**
 * 将子进程的 stdout 和 stderr, 重定向同一文件
 * 
 * 生成（Spawns）一个子进程，并重定向 stdout 和 stderr 到同一个文件。它与运行管道的外部命令差不多的想法，但会用 process::Stdio,
 * 把输入写入到指定的文件。 File::try_clone 引用相同文件的 stdout 和 stderr 控制 (Handle)。它将确保两个 Handles 使用相同的光标位置写入
 * 相当于运行 unix shell 命令： ls . oops > out.txt 2 > &1 
 */
fn main() -> Result<(), Error> {
    let outputs = File::create("out.txt")?;
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args(&[".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;
    
    Ok(())
}