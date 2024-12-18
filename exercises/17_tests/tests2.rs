// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // 可以在这里添加更多代码进行不同的操作，比如手动输入数值来测试函数等
    let num = 5;
    let result = power_of_2(num);
    println!("2 to the power of {} is {}", num, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // 测试边界值0
        assert_eq!(power_of_2(0), 1);

        // 测试较小的正整数
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
        assert_eq!(power_of_2(4), 16);
        assert_eq!(power_of_2(5), 32);
        assert_eq!(power_of_2(6), 64);
        assert_eq!(power_of_2(7), 128);
        assert_eq!(power_of_2(8), 256);

        // 测试稍大一些的正整数
        assert_eq!(power_of_2(10), 1024);
        assert_eq!(power_of_2(15), 32768);

        // 可以根据需要继续添加更多不同的n值进行更全面的测试
    }
}