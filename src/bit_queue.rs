use std::ops::AddAssign;

pub struct BitQueue {
    bits_to_follow: usize,
}

impl BitQueue {
    pub fn new() -> Self {
        BitQueue { bits_to_follow: 0 }
    }

    /// Returns a bit followed by the number of bits to follow with inversed value
    pub fn bit_followed_by_inverted(&mut self, bit: u8) -> Vec<u8> {
        let mut bits = Vec::with_capacity(self.bits_to_follow + 1);
        bits.push(bit);

        let inverted_bit = !bit & 1;
        for _ in 0..self.bits_to_follow {
            bits.push(inverted_bit);
        }

        self.bits_to_follow = 0;

        bits
    }
}

impl AddAssign<usize> for BitQueue {
    fn add_assign(&mut self, rhs: usize) {
        self.bits_to_follow += rhs;
    }
}

#[test]
fn bit_queue_test() {
    let mut bit_queue = BitQueue::new();
    bit_queue += 3;

    // 0 followed by 3 1s
    assert_eq!(vec![0, 1, 1, 1], bit_queue.bit_followed_by_inverted(0));

    bit_queue += 2;

    // 1 followed by 2 0s
    assert_eq!(vec![1, 0, 0], bit_queue.bit_followed_by_inverted(1));
}
