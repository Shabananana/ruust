// testcase: List

use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference `self` and create a reference to `vec` via destructuring
        let List(ref vec) = *self;

        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            try!(write!(f, "{}: ", count));
            try!(write!(f, "{}", v));
            if count != (vec.len() - 1) {
                try!(write!(f, ", "));
            }
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
