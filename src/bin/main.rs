use thread_pool::ThreadPool;

fn main() {
    let mut pool = ThreadPool::new(10);
    for i in 0..10 {
        pool.spawn(move || {
            println!("Start Job {}", i);
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("End Job {}", i);
        });
    }
    pool.join();
}
