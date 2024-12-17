fn animal_habitat(animal: &str) -> &str {
    // 修改为统一的返回类型 &str
    let identifier = if animal == "crab" {
        "1"
    } else if animal == "gopher" {
        "2"
    } else if animal == "snake" {
        "3"
    } else {
        "Unknown"
    };

    // 根据 identifier 的值返回相应的栖息地
    if identifier == "1" {
        "Beach"
    } else if identifier == "2" {
        "Burrow"
    } else if identifier == "3" {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
