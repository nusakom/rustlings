fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // 修复函数调用，传递一个 u8 类型的参数
    call_me(3);  // 传递一个 u8 值 3
}
