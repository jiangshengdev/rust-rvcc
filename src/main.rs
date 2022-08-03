use std::env;

fn main() {
    let mut args = env::args();
    let exe_path = args.next().unwrap();
    let src_code;

    match args.next() {
        Some(s) => src_code = s,
        None => panic!("{}: invalid number of arguments", exe_path),
    }

    println!("  .globl main");
    println!("main:");
    println!("  li a0, {}", src_code);
    println!("  ret");
}
