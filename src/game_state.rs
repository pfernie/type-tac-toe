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

pub type TurnResult<'gs> = Result<ValidMove<'gs>, InvalidMove<'gs>>;
pub trait Player {
    fn next_move<'gs>(&mut self, state: Turn<'gs>) -> ValidMove<'gs>;
}

pub struct ValidMove<'gs> {
    id: CellId,
    m: Cell,
    gs: &'gs mut GameState,
}

pub struct InvalidMove<'gs>(pub Turn<'gs>);

impl<'a> ValidMove<'a> {
    pub fn apply(self) {
        self.gs.set_cell(self.id, self.m);
    }
}

pub struct Turn<'a>(&'a mut GameState);

impl<'a> Turn<'a> {
    pub fn verify(self, id: CellId, m: Cell) -> TurnResult<'a> {
        match self.0.cells[id.0] {
            Cell::Empty => {
                Ok(ValidMove {
                    id: id,
                    m: m,
                    gs: self.0,
                })
            }
            _ => Err(InvalidMove(self)),
        }
    }
}

pub struct GameState {
    cells: [Cell; 9],
}

impl<'a> std::fmt::Display for Turn<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        for r in self.0.cells.windows(3) {
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

    pub fn take_turn(&mut self) -> Turn {
        Turn(self)
    }

    fn set_cell(&mut self, c: CellId, t: Cell) {
        println!("setting {:?} to {}", c, t);
        self.cells[c.0] = t;
    }
}
