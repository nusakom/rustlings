// 定义一个名为 `call_me` 的函数，函数没有参数，也没有返回值
fn call_me() {
    for i in 0..5 { // 使用 0..5 作为范围进行循环
        println!("Hello from call_me! Iteration: {}", i);
    }
}

fn main() {
    call_me(); // 调用 `call_me` 函数
}
