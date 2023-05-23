// https://nereuxofficial.github.io/posts/bitboard-rust/

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn any(self) -> bool {
        self.0 != 0
    }
    pub fn empty() -> Self {
        Self(0)
    }
}
