use crate::Stack;

// TODO Complete implementation
impl Stack for Vec<i32> {
    fn init() -> Self {
        //use new method of vec
        let vec: Vec<i32>= Vec::new();
        vec
    }

    fn push_val(&mut self, i: i32) {    
        //use push method of vec
        self.push(i);
    }

    fn top_val(&self) -> Option<&i32> {
        //tests if vecor is empty, otherwise returns the highest index element of the vector
        let len:usize = self.len();
        if self.is_empty(){
            None
        }
        else{
            Some(&self[len])
        }
    }

    fn pop_val(&mut self) -> Option<i32> {
        //use pop method of vec
        self.pop()
    }

    fn is_empty(&self) -> bool {     
        //use isempty method of vec
        self.is_empty()
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
        //simply initializes by returning the Nil variant
        Nil
    }

    fn push_val(&mut self, i: i32) {
        match self {
            //case 1: ListStack still has a pointer, keep following
            Val(_, Some(next)) => next.push_val(i),
            //case 2: End of List or empty List
            _ => {
                //init new ListStack
                let new_val = Val(i,Some(Box::new(Nil)));
                match self{
                    //empty List -> add new_val as Element
                     Nil => *self = new_val,
                    //end of List -> give previous Element the pointer to new_val
                      Val(_,next) => *next = Some(Box::new(new_val)),
                 }
            }
        };
    }

    fn top_val(&self) -> Option<&i32> {
        match self{
            Nil => None,
            Val(value, None) => Some(value),
            Val(_,Some(next)) => next.top_val(),
        }
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
