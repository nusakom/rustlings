struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Example usage:
    let wrapper_u32 = Wrapper::new(42u32);
    let wrapper_str = Wrapper::new("Foo");
    println!("Wrapper with u32: {}", wrapper_u32.value);
    println!("Wrapper with str: {}", wrapper_str.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42u32).value, 42u32);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
