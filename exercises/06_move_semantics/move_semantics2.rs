// 修复了编译错误
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88); // 现在可以修改 vec
    vec
}

fn main() {
    // 可选：你可以在此处运行并测试代码
    let vec0 = vec![22, 44, 66];
    let vec1 = fill_vec(vec0.clone());
    println!("{:?}", vec1); // 输出 [22, 44, 66, 88]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
