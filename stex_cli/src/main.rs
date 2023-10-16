use stex_common::threadpool::{Executor, ThreadPool};

struct CliExecutor;
type ExecutorParam = &'static str;

impl Executor<ExecutorParam> for CliExecutor {
    fn execute(command: ExecutorParam) {
        println!("Executing command: {}", command);
    }
}

fn main() {
    println!("Hello, world!");
    let pool = ThreadPool::<CliExecutor, ExecutorParam>::new(4);
    pool.push("");
}
