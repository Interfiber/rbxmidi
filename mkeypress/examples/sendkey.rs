use mkeypress::*;

fn main(){
    // test threading
    std::thread::Builder::new()
        .name("test".to_string())
        .spawn(|| send_key_wrap('%'))
        .expect("Failed to spawn thread");
    std::thread::sleep(std::time::Duration::from_secs(2));
}