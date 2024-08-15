use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

/**

*/

struct Stack<T> {
    storage: VecDeque<T>,
}

impl<T> Stack<T> {

    pub fn new() -> Self {
        Stack {
            storage: VecDeque::new()
        }
    }

    pub fn push(&mut self, element: T) {
        self.storage.push_back(element);
    }

    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.storage.pop_back()
    }
}


pub fn is_valid(input: String) -> bool {

    let mut stack: Stack<char> = Stack::new();

    for char in input.chars() {
        match char {
            ')' => {
                if stack.is_empty() {
                    return false
                }
                if let Some(value) = stack.pop() {
                    if value != '(' {
                        return false;
                    }
                }
            },

            '}' => {
                if stack.is_empty() {
                    return false
                }
                if let Some(value) = stack.pop() {
                    if value != '{' {
                        return false;
                    }
                }
            },
            ']' => {
                if stack.is_empty() {
                    return false
                }
                if let Some(value) = stack.pop() {
                    if value != '[' {
                        return false;
                    }
                }
            },
            '(' | '{' | '[' => stack.push(char),
            _ => panic!("In valid input")
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_happy_path() {
        let input = "()";
        let actual_result = is_valid(input.to_string());

        assert_eq!(true, actual_result);
    }
    #[test]
    fn verify_happy_path_two() {
        let input = "()[]{}";
        let actual_result = is_valid(input.to_string());

        assert_eq!(true, actual_result);

    }
    #[test]
    fn verify_happy_path_three() {
        let input = "(]";

        let actual_result = is_valid(input.to_string());
        assert_eq!(false, actual_result);
    }
}
