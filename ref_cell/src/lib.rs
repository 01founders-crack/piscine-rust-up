pub mod messenger;

use messenger::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>, // how many times the msg got sent
    max: usize,
}

impl Tracker {
    // Initializes a new `Tracker` with a maximum quota.
    pub fn new(max: usize) -> Self {
        Self {
            value: Default::default(),
            max,
            messages: Default::default(),
        }
    }

    // Sets the tracker's value based on the reference count of the provided `Rc`.
    // It checks if the reference count exceeds the maximum quota and
    // pushes messages to the `messages` vector accordingly.
    // use error msg from the messenger mod
    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        if count > self.max {
            self.messages.borrow_mut().push(error_msg());
            return;
        }

        self.value.replace(count);
        let percentage_of_max = convert_percentage(self.max, count);

        if percentage_of_max >= 70 {
            self.messages
                .borrow_mut()
                .push(warning_msg(percentage_of_max));
        }
    }

    /// Peeks at the usage of the provided `Rc` and reports it to the messages vector.
    pub fn peek(&self, value: &Rc<usize>) {
        let percentage_of_max = convert_percentage(self.max, Rc::strong_count(value));
        self.messages.borrow_mut().push(info_msg(percentage_of_max));
    }
}

// helper function
fn convert_percentage(max: usize, v: usize) -> usize {
    (100 * v) / max
}

#[test]
fn test_one() {
    let expected_messages = [
        "Info: This value would use 40% of your quota",
        "Warning: You have used up over 80% of your quota!",
        "Warning: You have used up over 100% of your quota!",
        "Error: You can't go over your quota!",
    ];

    let value = Rc::new(42);

    let track = Tracker::new(5);
    let _v = Rc::clone(&value);
    track.peek(&value); // 40%
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    track.set_value(&value); // 80%
    let _v = Rc::clone(&value);
    track.set_value(&value); // 100%
    let _v = Rc::clone(&value);
    track.set_value(&value); // >100%

    assert_eq!(track.messages.borrow().as_slice(), expected_messages);
}

#[test]
fn test_two() {
    let value = Rc::new(100);

    let track = Tracker::new(12);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);

    track.set_value(&value);

    let _v = Rc::clone(&value);

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 83% of your quota!"
    );

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 83% of your quota"
    );

    let _v = Rc::clone(&value);
    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 91% of your quota"
    );

    let _v = Rc::clone(&value);
    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 100% of your quota!"
    );

    let _v = Rc::clone(&value);

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 108% of your quota"
    );

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Error: You can't go over your quota!"
    );
}
