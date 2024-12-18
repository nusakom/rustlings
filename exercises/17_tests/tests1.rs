fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 这里可以根据自己的需求添加更多代码进行测试或者其他操作
    let num = 6;
    if is_even(num) {
        println!("{} is an even number.", num);
    } else {
        println!("{} is an odd number.", num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        // 测试偶数情况
        assert!(is_even(4));
        assert!(is_even(10));
        assert!(is_even(0));

        // 测试奇数情况
        assert!(!is_even(3));
        assert!(!is_even(7));
        assert!(!is_even(1));
    }
}