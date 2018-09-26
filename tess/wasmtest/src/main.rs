extern crate orion;

fn main() {
    use orion::hazardous::chacha20;

    let mut dst_out_pt = [0u8; 15];
    let mut dst_out_ct = [0u8; 15];
    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    let message = "Data to protect".as_bytes();


    chacha20::encrypt(&key, &nonce, 0, message, &mut dst_out_ct);

    chacha20::decrypt(&key, &nonce, 0, &dst_out_ct, &mut dst_out_pt);
    
    println!("{:?}\n{:?}\n密文:{:?}", std::str::from_utf8(&dst_out_pt), std::str::from_utf8(&message), std::str::from_utf8(&dst_out_ct));
    println!("Hello, world!");
}
