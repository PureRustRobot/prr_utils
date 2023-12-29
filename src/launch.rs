use std::process::{Command, Stdio};
use crate::logger::*;

pub fn bin_launch(bin_name:&str, param_file:&str)
{
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(bin_name)
        .arg(param_file)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run node");

    let status = child.wait().expect("Failed to wait");

    if status.success()
    {
        log_info(bin_name, "Start".to_string())
    }
    else
    {
        log_error(bin_name, "Failed to Start".to_string());
    }
}