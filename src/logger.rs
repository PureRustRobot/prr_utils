use colored::{self, Colorize};

pub fn log_info(bin_name:&str, msg:String)
{
    println!("{}:{}", bin_name, msg.bright_blue());
}
pub fn log_error(bin_name:&str, msg:String)
{
    println!("{}:{}", bin_name, msg.bright_red());
}