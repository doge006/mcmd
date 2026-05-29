fn main() {
    let amount = std::env::args().nth(1).expect("no amount given");
    let amount: i32 = amount.parse().unwrap();
    
    use std::process::Command;
    
    for _ in 0..amount {
        Command::new("cmd")
        .args(["/C", "start", "cmd"])
        .spawn()
        .unwrap();
    }
}
