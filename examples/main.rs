use toy_rust_async_rt::future;

fn main() {
    future::block_on(async {
        println!("Hello world!");
    })
}
