use speedy2d::dimen::Vec2;
use std::collections::VecDeque;

pub struct History {
    pub buffer: VecDeque<Option<Vec2>>,
    size: usize,
}

impl History {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: VecDeque::from(vec![None; size]),
            size,
        }
    }

    pub fn push(&mut self, value: Option<Vec2>) {
        self.buffer.push_front(value);
        if self.buffer.len() >= self.size {
            self.buffer.pop_back();
        }
    }
}

impl<'a> IntoIterator for &'a History {
    type Item = Option<Vec2>;
    type IntoIter = HistoryIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        HistoryIterator {
            buffer: self,
            index: 0,
        }
    }
}

pub struct HistoryIterator<'a> {
    buffer: &'a History,
    index: usize,
}

impl<'a> Iterator for HistoryIterator<'a> {
    type Item = Option<Vec2>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.buffer.size {
            Some(self.buffer.buffer[self.index])
        } else {
            None
        };
        self.index += 1;
        result
    }
}
