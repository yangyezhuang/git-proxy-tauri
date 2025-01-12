use std::os::windows::process::CommandExt;
use std::process::Command;

pub fn change_proxy(mode: &str) -> () {
    match mode {
        "default" => {
            unset_proxy("http");
            unset_proxy("https");
        }
        "http" => {
            let addr = "127.0.0.1:7890";
            set_proxy("http", &format!("http://{}", addr));
            set_proxy("https", &format!("http://{}", addr));
        }
        "socks" => {
            let addr = "127.0.0.1:80";
            set_proxy("http", &format!("socks5://{}", addr));
            set_proxy("https", &format!("socks5://{}", addr));
        }
        _ => {
            // return Err("Invalid proxy mode".into());
        }
    }
}

fn unset_proxy(mode: &str) -> () {
    let result = Command::new("git")
        .creation_flags(0x08000000)
        .args(&["config", "--global", "--unset", &format!("{}.proxy", mode)])
        .status();
    match result {
        Ok(status) => {
            if !status.success() {
                println!("Failed to unset git proxy");
            }
        }
        Err(e) => {
            println!("Error running git command: {}", e);
        }
    }
}

fn set_proxy(mode: &str, addr: &str) -> () {
    let result = Command::new("git")
        .creation_flags(0x08000000)
        .args(&["config", "--global", &format!("{}.proxy", mode), addr])
        .status();
    match result {
        Ok(status) => {
            if !status.success() {
                println!("Failed to set git proxy");
            }
        }
        Err(e) => {
            println!("Error running git command: {}", e);
        }
    }
}
