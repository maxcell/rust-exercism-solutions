pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
                // if let Some(top) = stack.last() {
                //     if *top == '[' {
                //         stack.pop();
                //     } else {
                //         return false;
                //     }
                // } else {
                //     return false;
                // }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
                // if let Some(top) = stack.last() {
                //     if *top == '(' {
                //         stack.pop();
                //     } else {
                //         return false;
                //     }
                // } else {
                //     return false;
                // }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
                // if let Some(top) = stack.last() {
                //     if *top == '{' {
                //         stack.pop();
                //     } else {
                //         return false;
                //     }
                // } else {
                //     return false;
                // }
            }
            _ => {}
        }
    }
    // Stack approach?
    //  Read in the character
    //   - if we see a leftside operator, add it to the stack
    //   - if we  see a right hand operator, check the top of the stack
    //      - if it doesn't match the top of the stack, return false
    //      - if it matches the top, pop the top off and move to the
    //        next character

    stack.is_empty()
}
