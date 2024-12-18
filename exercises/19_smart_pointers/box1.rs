// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Create an empty cons list.
fn create_empty_list() -> List {
    List::Nil
}

// Create a non-empty cons list.
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        let non_empty_list = create_non_empty_list();
        assert_ne!(create_empty_list(), non_empty_list);

        // Check the structure of the non-empty list.
        if let List::Cons(head, tail) = non_empty_list {
            assert_eq!(head, 1);
            if let List::Cons(next, next_tail) = *tail {
                assert_eq!(next, 2);
                assert_eq!(*next_tail, List::Nil);
            } else {
                panic!("The second element is not correct");
            }
        } else {
            panic!("The list should not be empty");
        }
    }
}
