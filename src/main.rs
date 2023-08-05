use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let a = std::thread::spawn(move || {});
    a.join().unwrap();
    println!("didn''t crash");
}
