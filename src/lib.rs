mod cpu;
mod disasm;
mod fields;
mod handlers;
pub mod instruction;
mod operands;
pub mod simulator;

use std::iter::Peekable;

pub use disasm::{decode_8086, write_8086};

pub struct EnumeratePeekable<I: Iterator> {
    iter: Peekable<I>,
    count: usize,
}

impl<I: Iterator> EnumeratePeekable<I> {
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            count: 0,
        }
    }

    fn peek(&mut self) -> Option<&I::Item> {
        self.iter.peek()
    }

    fn next_with_index(&mut self) -> Option<(usize, I::Item)> {
        self.iter.next().map(|i| {
            self.count += 1;
            (self.count - 1, i)
        })
    }

    fn vended_count(&self) -> usize {
        self.count
    }
}

impl<I: Iterator> Iterator for EnumeratePeekable<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next();
        if next.is_some() {
            self.count += 1;
        }
        next
    }
}

type ByteStream<'a> = EnumeratePeekable<std::slice::Iter<'a, u8>>;
