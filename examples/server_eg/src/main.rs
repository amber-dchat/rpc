use rpc_server::{bootstrap, IResponse};

struct Window;

impl IResponse for Window {
    fn send_listener(&self, data: &str) -> () {
        println!("Sent {data}");
    }

    fn submit(&self, success: bool) -> () {
        println!("Report Success: {success}");
    }
}

fn main() {
    bootstrap(Window);

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1000));
    }
}
