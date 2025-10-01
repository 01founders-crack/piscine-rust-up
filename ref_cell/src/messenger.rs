pub fn error_msg() -> String {
    "Error: You can't go over your quota!".to_owned()
}

pub fn warning_msg(percentage: usize) -> String {
    format!("Warning: You have used up over {percentage}% of your quota!")
}

pub fn info_msg(percentage: usize) -> String {
    format!("Info: This value would use {percentage}% of your quota")
}
