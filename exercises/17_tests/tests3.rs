struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 这里可以根据自己的需求添加更多代码进行测试或者其他操作
    let rect = Rectangle::new(5, 8);
    println!("Created a rectangle with width {} and height {}", rect.width, rect.height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽度是否正确
        assert_eq!(rect.height, 20); // 检查高度是否正确
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}