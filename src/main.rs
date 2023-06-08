use crypto::md5::Md5;
use crypto::digest::Digest;
fn main() {
    let args: Vec<String>=std::env::args().collect();
    if args.len()>1 {
       
       let   text=args[1].clone();
        let mut hasher=Md5::new();
        hasher.input_str(&text);
        println!("src:{}\nmd5:{}\nMD5:{}",text,hasher.result_str(),hasher.result_str().to_uppercase());
    }
   
}
