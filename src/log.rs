use colored::Colorize;

pub fn info_log(msg: String) {
    let info = "[INFO]".truecolor(0, 255, 0);
    println!("{} {}", info, msg);
}
