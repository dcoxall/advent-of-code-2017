pub struct CircularBuffer {
    inner: Vec<usize>,
    cursor: usize,
    step: usize,
}

impl CircularBuffer {
    pub fn new(step: usize) -> Self {
        CircularBuffer{
            inner: vec![0],
            cursor: 0,
            step,
        }
    }

    pub fn push(&mut self, element: usize) {
        self.cursor = (self.cursor + self.step + 1) % self.inner.len();
        self.inner.insert(self.cursor, element);
    }

    pub fn peek(&self) -> usize {
        self.inner[(self.cursor + 1) % self.inner.len()]
    }
}

pub fn fill_buffer(buffer: &mut CircularBuffer, count: usize) {
    for i in 0..count {
        buffer.push(i + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut buffer = CircularBuffer::new(3);
        fill_buffer(&mut buffer, 2017);
        assert_eq!(buffer.peek(), 638);
    }
}
