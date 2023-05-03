use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        todo!()
        //use new method of vec
    }

    fn push_val(&mut self, i: i32) {
        todo!()
        //use push method of vec
    }

    fn top_val(&self) -> Option<&i32> {
        todo!()
        //either pop an element, save it then push and return(seems a bit brutish)
        //or get lenth with len then return element at last index
    }

    fn pop_val(&mut self) -> Option<i32> {
        todo!()
        //use pop method of vec
    }

    fn is_empty(&self) -> bool {
        todo!()
        //use isempty method of vec
    }
}

#[derive(Debug)]
pub enum ListStack {
    Val(i32, Option<Box<ListStack>>),
    Nil,
}

use ListStack::Nil;
use ListStack::Val;

// Complete implementation of Stack for ListStack
impl Stack for ListStack {
    fn init() -> Self {
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            Val(value, other) => *self = todo!(),
            Nil => *self = todo!(),
        };
    }

    fn top_val(&self) -> Option<&i32> {
        todo!()
    }

    fn pop_val(&mut self) -> Option<i32> {
        match self {
            Val(value, other) => {
                let popped_value = *value;
                match other.take() {
                    None => *self = Nil,
                    Some(other) => todo!(),
                };
                todo!()
            }
            Nil => None,
        }
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::stack::ListStack;
    use crate::Stack;
    use std::fmt::Debug;

    #[test]
    fn vec_fill_and_clear() {
        println! {"Testing Vec<T>"}
        fill_and_clear_impl(Vec::init());
    }

    #[test]
    fn linked_fill_and_clear() {
        println! {"Testing ListStack"}
        fill_and_clear_impl(ListStack::init());
    }

    fn fill_and_clear_impl<T: Stack + Debug>(mut stack: T) {
        stack.push_val(1);
        assert_eq!(stack.top_val(), Some(&1));

        stack.push_val(2);
        assert_eq!(stack.top_val(), Some(&2));

        stack.push_val(-3);
        assert_eq!(stack.top_val(), Some(&-3));

        println!("{:?}", stack);

        let mut comparison = vec![1, 2, -3];
        while let Some(val) = stack.pop_val() {
            assert_eq!(comparison.pop().unwrap(), val);
        }

        assert!(stack.is_empty())
    }
}
