#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (rank >= 0 && rank <= 7) && (file >= 0 && file <= 7) {
            return Some(ChessPosition{rank, file});
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = (self.position.rank - other.position.rank).abs();
        let dy = (self.position.file - other.position.file).abs();
        if self.position.rank == other.position.rank || self.position.file == other.position.file || dx == dy {
            return true;
        }
        false
    }
}
