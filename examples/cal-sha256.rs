extern crate walkdir;
extern crate ring;
extern crate num_cpus;
extern crate threadpool;

use walkdir::WalkDir;
use std::fs::File;
use std::io::{BufReader, Read, Error};
use std::path::Path;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use ring::digest::{Context, Digest, SHA256};
use std::time;

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
  let mut buf_reader = BufReader::new(File::open(&filepath)?);
  let mut context = Context::new(&SHA256);
  let mut buffer = [0; 1024];

  loop {
    let count = buf_reader.read(&mut buffer)?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }
  Ok((context.finish(), filepath))
}

fn is_pdf(path: &Path) -> bool {
  match path.extension() {
    Some(name) => name == "pdf",
    _ => false
  }
}

fn main() -> Result<(), Error> {
  let time = time::SystemTime::now();

  let pool = ThreadPool::new(num_cpus::get());
  
  let (tx, rx) = channel();

  let dirs = WalkDir::new("/user/home/Downloads")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| !e.path().is_dir() && is_pdf(e.path()));

  for entry in dirs {
    let path = entry.path().to_owned();
    let tx = tx.clone();
    pool.execute(move || {
      let digest = compute_digest(path);
      tx.send(digest).expect("Could not send data!");
    });
  }
  drop(tx);
  for t in rx.iter() {
    let (sha, path) = t?;
    println!("{:?} {:?}", sha, path);
  }
  println!("time: {}", time.elapsed().unwrap().as_secs_f32());
  Ok(())
}

// 对当前目录中，具有 PDF 扩展名的每个文件，合计它们的 SHA256。线程池生成的线程数，等于系统(CPU)核心数，这个能通过num_cpus::get获取。
// Walkdir::new迭代当前目录，并调用execute执行读取和计算 SHA1 哈希的操作。
// TODO: 改写成异步处理的方式