#[macro_use]
extern crate error_chain;

use std::process::{Command, Stdio};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
    }
}
// 显示，当前工作目录中，10个最大的文件和子目录。
// 它相当于运行： du -ah . | sort -hr | head -n 10
// Command 们代表一个过程。一个子进程输出用 Stdio::piped, 在父级和子级之间捕获。
fn run() -> Result<()> {
    let directory = std::env::current_dir()?;

    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;


    if let Some(du_output) = du_output_child.stdout.take() {
        let mut sort_output_child = Command::new("sort")
            .arg("-hr")
            .stdin(du_output)
            .stdout(Stdio::piped())
            .spawn()?;
        
        du_output_child.wait()?;
    
        if let Some(sort_output) = sort_output_child.stdout.take() {
            let head_output_child = Command::new("head")
                .args(&["-n", "10"])
                .stdin(sort_output)
                .stdout(Stdio::piped())
                .spawn()?;
            
            let head_stdout = head_output_child.wait_with_output()?;

            sort_output_child.wait()?;

            println!(
                "Top 10 biggest files and directories in '{}':\n{}",
                directory.display(),
                String::from_utf8(head_stdout.stdout).unwrap()
            );
        }
    }

    Ok(())
}


fn main() {
    let _err = run();
}