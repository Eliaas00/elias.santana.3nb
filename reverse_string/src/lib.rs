pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = "hello";
    let reversed = reverse(input);
    println!("String invertida: {}", reversed);
}
