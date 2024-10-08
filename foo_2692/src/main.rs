const AB: &str = "AB";
const CD: &str = "CD";

struct Solution {}

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut string: String = s;
        let mut flag: bool = true;
        while flag {
            println!("{}", string);
            if let Some(index) = string.find(AB) {
                string.remove(index);
                string.remove(index);
                continue;
            }
            if let Some(index) = string.find(CD) {
                string.remove(index);
                string.remove(index);
                continue;
            }
            flag = false;
        }
        return string.len() as i32;
    }
}

pub fn main(){
    let input: String = String::from("ABFCACDB");
    println!("{}", Solution::min_length(input));
}