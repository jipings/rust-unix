extern crate libc;

use std::time::Instant;

use libc::{pid_t};

#[link(name = "c")]
extern {
    fn getpid() -> pid_t;
}

fn main() {
    let now = Instant::now();
    let x = unsafe {        
        getpid()
    };
    let elapsed_time = now.elapsed();
    println!("Running fun took {} seconds.", elapsed_time.as_secs_f32());
    println!("Process PID is {}", x);

}
