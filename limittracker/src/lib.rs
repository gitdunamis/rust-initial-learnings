trait Messanger {
    fn send(&self, message: &str);
}

struct LimitTracker<'a, T: Messanger> {
    messanger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messanger{
    fn new(messanger: &'a T, max: usize) -> Self {
        LimitTracker { messanger, value: 0, max }
    }

    fn set_value(&mut self, new_value: usize) {
        self.value = new_value;

        let percent_change = self.value as f64 / self.max as f64;

        if percent_change >=1.0 {
            self.messanger.send("ERROR: You have used up your quota")
        } else if percent_change >= 0.9 {
            self.messanger.send("URGENT: You are above 90% of your quota")
        } else if percent_change >= 0.75 {
            self.messanger.send("WARNING: You are above 75% usage")
        }
        
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    
    struct TestMessanger {
        messages: RefCell<Vec<String>>
    }

    impl TestMessanger {
        fn new() -> Self {
            TestMessanger { messages: RefCell::new(vec![]) }
        }
    }

    impl Messanger for TestMessanger {
        fn send(&self, message: &str) {
            self.messages.borrow_mut().push(String::from(message))
        }
    }

    #[test]
    fn it_works() {
        let mut messanger = TestMessanger::new();
        
        let mut tracker = LimitTracker::new(&mut messanger, 100);
        tracker.set_value(95);
        
        assert_eq!(1, messanger.messages.borrow().len());
    }
}
