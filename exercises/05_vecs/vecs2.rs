fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // 每个元素乘以2
        *element *= 2;
    }

    // 返回修改后的 Vec
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // 每个元素乘以2，返回一个新的 Vec
        element * 2
    }).collect()
}

fn main() {
    // 可选：你可以在此处运行你的代码
    let v = vec![2, 4, 6, 8, 10];
    let result = vec_loop(v.clone());
    println!("Result from vec_loop: {:?}", result);

    let result_map = vec_map(&v);
    println!("Result from vec_map: {:?}", result_map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        // 确保结果与预期相符
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        // 确保结果与预期相符
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
