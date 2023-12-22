use crate::get_xml;
use colored::{self, Colorize};
use std::process::{Command, Stdio};

pub fn wait_command()
{
    let cmd_arg = std::env::args().nth(0).unwrap();

    let file_name = std::env::args().nth(1).unwrap();

    if "launch" != cmd_arg
    {
        return;
    }
    else 
    {
        launch_system(file_name);
    }
}

fn launch_system(file_name:String)
{

}

async fn bin_launch(bin_name:&str)
{
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg(bin_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run node");

    let status = child.wait().expect("Failed to wait");

    if status.success()
    {
        print!("{}{}", "[LaunchSystem]Start ".bright_blue(), bin_name.bright_blue());
    }
}