pub mod queue {

    pub struct Queue {
        pub(crate) older: Vec<char>,
        pub(crate) younger: Vec<char>,
    }

    impl Queue {
        pub fn new() -> Self {
            Self {
                older: Vec::new(),
                younger: Vec::new(),
            }
        }

        pub fn push(&mut self, c: char) {
            self.younger.push(c);
        }

        pub fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }

                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            self.older.pop()
        }
        pub fn split(self) -> (Vec<char>, Vec<char>) {
            (self.older, self.younger)
        }
    }
}
