#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T:Clone> {
    buffer: Vec<Option<T>>,
    size: usize,
    next: usize,
}

impl<T:Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![None; size],
            size: 0,
            next: 0,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        // [ ][ ][0][1] - next=0, size=2, b.len=4, idx=2 | (next-size+b.len)%b.len = (0-2+4)%4 = 2
        // [1][ ][ ][0] - next=1, size=2, b.len=4, idx=3 | (1-2+4)%4 = 2
        // [ ][0][1][2][ ] - next=4, size=3, b.len=5, idx=1 | (4-3+5)%5 = 1
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        let idx: usize = (self.buffer.len() + self.next - self.size) % self.buffer.len();
        let Some(result) = self.buffer[idx].clone() else {panic!()};
        self.size -= 1;
        Ok(result)
    }

    pub fn write(&mut self, byte: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }
        self.buffer[self.next] = Some(byte);
        self.size += 1;
        self.next = (self.next + 1) % self.buffer.len();
        Ok(())
    }

    pub fn clear(&mut self) {
        let size = self.buffer.len();
        *self = CircularBuffer::new(size);
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.buffer.len()
    }
}
