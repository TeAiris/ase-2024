pub struct RingBuffer<T> {
    // TODO: fill this in.
    buffer: Vec<T>, // Internal buffer for storing values
    read_index: usize, // Index for reading values
    write_index: usize, // Index for writing values
}

impl<T: Copy + Default> RingBuffer<T> {
    pub fn new(length: usize) -> Self {
        // Create a new RingBuffer with `length` slots and "default" values.
        // Hint: look into `vec!` and the `Default` trait.
        Self {
            buffer: vec![T::default(); length],
            read_index: 0,
            write_index: 0,
        }
    }

    pub fn reset(&mut self) {
        // Clear internal buffer and reset indices.
        self.buffer = vec![T::default(); self.buffer.len()];
        self.read_index = 0;
        self.write_index = 0;
    }

    pub fn put(&mut self, value: T) {
        // `put` and `peek` write/read without advancing the indices.
        self.buffer[self.write_index] = value;
    }

    pub fn peek(&self) -> T {
        // `put` and `peek` write/read without advancing the indices.
        self.buffer[self.read_index]
    }

    pub fn get(&self, offset: usize) -> T {
        // `get` reads at an offset from the read index without advancing the indices.
        self.buffer[(self.read_index + offset) % self.buffer.len()]
    }

    pub fn push(&mut self, value: T) {
        // `push` and `pop` write/read and advance the indices.
        self.put(value);
        self.write_index = (self.write_index + 1) % self.buffer.len();
    }

    pub fn pop(&mut self) -> T {
        // `push` and `pop` write/read and advance the indices.
        let value = self.peek();
        self.read_index = (self.read_index + 1) % self.buffer.len();
        value
    }

    pub fn get_read_index(&self) -> usize {
        // Returns the current read index.
        self.read_index
    }

    pub fn set_read_index(&mut self, index: usize) {
        // Sets the read index to a new value.
        self.read_index = index % self.buffer.len();
    }

    pub fn get_write_index(&self) -> usize {
        // Returns the current write index.
        self.write_index
    }

    pub fn set_write_index(&mut self, index: usize) {
        // Sets the write index to a new value.
        self.write_index = index % self.buffer.len();
    }

    pub fn len(&self) -> usize {
        // Return number of values currently in the buffer.
        if self.write_index >= self.read_index {
            self.write_index - self.read_index
        } else {
            self.buffer.len() - self.read_index + self.write_index
        }
    }

    pub fn capacity(&self) -> usize {
        // Return the length of the internal buffer.
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::RingBuffer;

    #[test]
    fn test_new() {
        let buffer: RingBuffer<i32> = RingBuffer::new(10);
        assert_eq!(buffer.capacity(), 10);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_push_pop() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(10);
        buffer.push(1);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.pop(), 1);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_peek() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(10);
        buffer.push(1);
        assert_eq!(buffer.peek(), 1);
        assert_eq!(buffer.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(10);
        buffer.push(1);
        buffer.push(2);
        assert_eq!(buffer.get(0), 1);
        assert_eq!(buffer.get(1), 2);
    }

    #[test]
    fn test_reset() {
        let mut buffer: RingBuffer<i32> = RingBuffer::new(10);
        buffer.push(1);
        buffer.reset();
        assert_eq!(buffer.len(), 0);
    }
}
