        pub fn brackets_are_balanced(string: &str) -> bool {
            let s = string.chars().filter(|x|x.eq(&')') 
                | x.eq(&'(') 
                | x.eq(&'}') 
                | x.eq(&'{') 
                | x.eq(&'[') 
                | x.eq(&']'));
            let mut c = vec![];

            for i in s {
            match i {
                '(' | ')' => if i == '(' {c.push(i)} else if c.pop() != Some('(') {return false;},
                '[' | ']' => if i == '[' {c.push(i)} else if c.pop() != Some('[') {return false;},
                '{' | '}' => if i == '{' {c.push(i)} else if c.pop() != Some('{') {return false;},
                _   => panic!("panic"),
            }
        }
            if c.len() == 0 {true} else {false}
}
