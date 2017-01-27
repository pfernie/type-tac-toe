mod game_state;

use game_state::*;

struct RandomPlayer(Cell);
impl Player for RandomPlayer {
    fn next_move<'gs>(&mut self, turn: Turn<'gs>) -> ValidMove<'gs> {
        println!("{}", turn);
        let z = CellId::new(0usize).expect("cannot create CellId @ 0");
        let z2 = CellId::new(1usize).expect("cannot create CellId @ 1");
        // turn.0.set_cell(z, self.0); // not permissible; private
        if let Ok(m) = turn.verify(z, self.0)
            .or_else(|inv| inv.0.verify(z2, self.0)) {
            m
        } else {
            panic!("no valid move!!!");
        }
    }
}

fn main() {
    let mut gs = GameState::new();
    let mut r1 = RandomPlayer(Cell::Cross);
    let mut r2 = RandomPlayer(Cell::Circle);
    {
        let m1 = r1.next_move(gs.take_turn());
        m1.apply();
    }
    let m2 = r2.next_move(gs.take_turn());
    m2.apply();
}
