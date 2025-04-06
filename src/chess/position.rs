use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub rank: usize, // 0-7, representing rows (0 = 8th rank, 7 = 1st rank)
    pub file: usize, // 0-7, representing columns (0 = a-file, 7 = h-file)
}

impl Position {
    pub fn new(rank: usize, file: usize) -> Self {
        assert!(rank < 8, "Rank must be between 0 and 7");
        assert!(file < 8, "File must be between 0 and 7");
        Self { rank, file }
    }

    pub fn from_algebraic(notation: &str) -> Option<Self> {
        if notation.len() != 2 {
            return None;
        }

        let chars: Vec<char> = notation.chars().collect();
        let file = chars[0] as usize - 'a' as usize;
        let rank = 8 - (chars[1] as usize - '0' as usize);

        if file < 8 && rank < 8 {
            Some(Self { rank, file })
        } else {
            None
        }
    }

    pub fn to_algebraic(&self) -> String {
        let file_char = (self.file as u8 + b'a') as char;
        let rank_char = (8 - self.rank) as u8 + b'0';
        format!("{}{}", file_char, rank_char as char)
    }

    pub fn is_valid(&self) -> bool {
        self.rank < 8 && self.file < 8
    }

    pub fn offset(&self, rank_offset: isize, file_offset: isize) -> Option<Self> {
        let new_rank = self.rank as isize + rank_offset;
        let new_file = self.file as isize + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            Some(Self::new(new_rank as usize, new_file as usize))
        } else {
            None
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}
