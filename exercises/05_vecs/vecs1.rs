fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array
    let v = vec![10, 20, 30, 40]; // Create a vector with the same elements

    (a, v)
}

fn main() {
    let (a, v) = array_and_vec();
    println!("Array: {:?}", a);
    println!("Vector: {:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]); // Compare the array and the slice of the vector
    }
}
