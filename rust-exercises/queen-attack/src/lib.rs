#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            return Some(ChessPosition(rank, file));
        }
        None
    }

    pub fn sub(&self, other: &ChessPosition) -> ChessPosition {
        return ChessPosition(self.0 - other.0, self.1 - other.1);
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.0 == other.position.0 {
            return true;
        }
        if self.position.1 == other.position.1 {
            return true;
        }
        let aux: ChessPosition = self.position.sub(&other.position);
        if aux.0.abs() == aux.1.abs() {
            return true;
        }
        false
    }
}
