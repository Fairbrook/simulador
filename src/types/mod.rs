pub mod global;
pub mod process;

pub fn fill_str(string: &String, len: usize, filler: char) -> String {
    if string.len() < len {
        let mut dif = len - string.len();
        let mut res = String::new();
        while dif > 0 {
            res += &String::from(filler);
            dif -= 1;
        }
        res += string;
        return res;
    }
    string.clone()
}

pub fn seconds_to_str(seconds: u32) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;
    let seconds = seconds % 60;
    (if hours > 0 {
        hours.to_string() + ":"
    } else {
        String::new()
    }) + &fill_str(&minutes.to_string(), 2, '0')
        + ":"
        + &fill_str(&seconds.to_string(), 2, '0')
}
