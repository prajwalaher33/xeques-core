use serde::Serialize;

#[derive(Serialize)]
struct Command {
    device: &'static str,
    action: &'static str,
}

fn main() {
    let cmd = Command { device: "sim-01", action: "SetMode:Idle" };
    println!("Command Verified: true");
    println!("{}", serde_json::to_string(&cmd).unwrap());
}
