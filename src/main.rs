fn test() -> &'static str {
   "Andrzej Głuszak"
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(test() == "Andrzej Głuszak");
    }
}
