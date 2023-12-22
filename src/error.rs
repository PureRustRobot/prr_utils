use colored::{self, Colorize};

pub fn no_arg()
{
    if std::env::args().len() == 1
    {
        println!("{}", "[ERROR]no arg given".on_bright_red());
        std::process::exit(0);
    }
}