use std;

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Circle,
    Cross,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f,
               "{}",
               match *self {
                   Cell::Empty => ".",
                   Cell::Circle => "O",
                   Cell::Cross => "X",
               })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CellId(usize);

impl CellId {
    pub fn new(i: usize) -> Option<CellId> {
        if i < 9 { Some(CellId(i)) } else { None }
    }
}

pub trait Player {
    fn next_move<'gs>(&mut self, state: Turn<'gs>) -> MoveResult<'gs>;
}

pub enum MoveResult<'gs> {
    Valid(Move<'gs>),
    GiveUp(Turn<'gs>),
}

#[must_use]
pub struct Move<'gs> {
    id: CellId,
    m: Cell,
    gs: &'gs mut GameState,
}

impl<'gs> Move<'gs> {
    pub fn end_turn(self) {
        self.gs.set_cell(self.id, self.m);
    }

    pub fn undo(self) -> Turn<'gs> {
        Turn(self.gs)
    }
}

pub enum VerifyResult<'gs> {
    Valid(Move<'gs>),
    Invalid(Turn<'gs>),
}

impl<'gs> VerifyResult<'gs> {
    pub fn or<F: FnOnce(Turn<'gs>) -> VerifyResult<'gs>>(self, f: F) -> VerifyResult<'gs> {
        match self {
            VerifyResult::Invalid(t) => f(t),
            v => v,
        }
    }
}

pub struct Turn<'gs>(&'gs mut GameState);

impl<'gs> Turn<'gs> {
    pub fn verify(self, id: CellId, m: Cell) -> VerifyResult<'gs> {
        match self.0.cells[id.0] {
            Cell::Empty => {
                VerifyResult::Valid(Move {
                    id: id,
                    m: m,
                    gs: self.0,
                })
            }
            _ => VerifyResult::Invalid(self),
        }
    }
}

pub struct GameState {
    cells: [Cell; 9],
}

impl<'gs> std::fmt::Display for Turn<'gs> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for r in self.0.cells.chunks(3) {
            for c in r {
                write!(f, " {} ", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState { cells: [Cell::Empty; 9] }
    }

    pub fn start_turn(&mut self) -> Turn {
        Turn(self)
    }

    fn set_cell(&mut self, c: CellId, t: Cell) {
        println!("setting {:?} to {}", c, t);
        self.cells[c.0] = t;
    }
}
