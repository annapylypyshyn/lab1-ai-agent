fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    println!("Hello, world!");
    println!("2 + 2 = {}", add(2, 2)); // Використовуємо функцію
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    #[allow(non_snake_case)]
    fn BasicAddition() {
        assert_eq!(add(2, 3), 5);
    }
}