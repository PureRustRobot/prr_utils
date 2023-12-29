use colored::{self, Colorize};
use std::process::{Command, Stdio};

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
        println!("{}{}", "[LaunchSystem]Start ".bright_blue(), bin_name.bright_blue());
    }
    else
    {
        println!("{}{}", "[LaunchSystem]Failed to Start ".bright_red(), bin_name.bright_red());
    }
}