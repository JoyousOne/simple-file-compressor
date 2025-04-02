use std::ops::Index;

pub enum Bit {
    ZERO,
    ONE,
}

pub struct CompressedBuffer {
    /// Buffer containing the bytes
    pub buffer: Vec<u8>,
    // current_byte_index: usize,
    current_bit_index: u8,
}

impl CompressedBuffer {
    pub fn new() -> Self {
        CompressedBuffer {
            buffer: Vec::new(),
            // current_byte_index: 0,
            current_bit_index: 7,
        }
    }

    /// set the last bit as a given value
    ///
    // Usefull bitwise cheat sheet: https://togglebit.io/posts/rust-bitwise/
    pub fn push_bit(&mut self, bit: Bit) {
        // append new byte
        if self.current_bit_index == 7 {
            self.buffer.push(0);
        }

        let last_byte = self
            .buffer
            .last_mut()
            .expect("In src/compressed_buffer => push_bit: cannot access last byte");

        match bit {
            Bit::ONE => {
                *last_byte = *last_byte | (1 << self.current_bit_index);
            }
            Bit::ZERO => { /*Val already at 0, no need to set*/ }
        }

        // DEBUG
        // println!("last_byte: {:#010b}", *last_byte);

        self.current_bit_index = if self.current_bit_index == 0 {
            7
        } else {
            self.current_bit_index - 1
        };
    }
}

impl Index<usize> for CompressedBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn pushing_bit() {
        let mut compressed_buffer = CompressedBuffer::new();

        // assuming we are using the following encoding:
        // f: 0
        // c: 100
        // d: 101
        // a: 1100
        // b: 1101
        // e: 111

        // We wish to encode 'faced'
        #[cfg_attr(any(), rustfmt::skip)]
        let encoded: Vec<u8> = vec![
             /*f*/ 0,
             /*a*/ 1, 1, 0, 0,
             /*c*/ 1, 0, 0,
             /*e*/ 1, 1, 1,
             /*d*/ 1, 0, 1];

        // Should be: 0b0110_0100 0b1111_01--

        for bit in encoded {
            match bit {
                0 => compressed_buffer.push_bit(Bit::ZERO),
                1 => compressed_buffer.push_bit(Bit::ONE),
                _ => panic!("should not be possible"),
            }
        }

        // DEBUG
        // println!("compressed_buffer[0]: {:#010b}", compressed_buffer[0]);
        // println!("compressed_buffer[1]: {}", compressed_buffer[1]);

        assert_eq!(compressed_buffer[0], 0b0110_0100);
        assert_eq!(compressed_buffer[1], 0b1111_0100);
    }
}
