use colored::{self, Colorize};

pub fn log_info(bin_name:&str, msg:String)
{
    println!("[{}]:{}", bin_name.blue(), msg.blue());
}
pub fn log_error(bin_name:&str, msg:String)
{
    println!("[{}]:{}", bin_name.red(), msg.red());
}
