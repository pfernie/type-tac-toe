mod game_state;

use game_state::*;

struct RandomPlayer(String, Cell);
impl Player for RandomPlayer {
    fn next_move<'gs>(&mut self, turn: Turn<'gs>) -> MoveResult<'gs> {
        println!("{}", turn);
        let cell0 = CellId::new(0usize).expect("cannot create CellId @ 0");
        let cell1 = CellId::new(1usize).expect("cannot create CellId @ 1");
        // turn.0.set_cell(z, self.0); // not permissible; private

        // Player cannot .end_turn() the Move, since then have no means to return a MoveResult:
        // - cannot return MoveResult::Valid, as Move was consumed
        // - cannot return MoveResult::GiveUp, as Turn was consumed
        // But, Player can verify() a move, getting a TurnResult::Valid(Move), and then call
        // Move::undo() to turn this back into a Turn and try something else...
        match turn.verify(cell0, self.1)
            .or(|t| t.verify(cell1, self.1)) {
            VerifyResult::Valid(m) => MoveResult::Valid(m),
            VerifyResult::Invalid(t) => MoveResult::GiveUp(t),
        }
    }
}

fn run_turn(p: &mut RandomPlayer, gs: &mut GameState) {
    match p.next_move(gs.start_turn()) {
        MoveResult::Valid(m) => m.end_turn(),
        MoveResult::GiveUp(_) => panic!("{} gave up!", p.0),
    }
}

fn main() {
    let mut gs = GameState::new();
    let mut r1 = RandomPlayer("Player 1".to_owned(), Cell::Cross);
    let mut r2 = RandomPlayer("Player 2".to_owned(), Cell::Circle);
    // explicit calls here to show borrowck behavior
    // should succeed on Cell 0
    match r1.next_move(gs.start_turn()) {
        MoveResult::Valid(m) => {
            //r2.next_move(gs.start_turn()); // illegal: there is a live Move
            m.end_turn(); // Move is marked must_use, so will warn if we forget to .end_turn()
        }
        MoveResult::GiveUp(_) => panic!("{} gave up!", r1.0),
    };
    // should succeed on Cell 1
    match r2.next_move(gs.start_turn()) {
        MoveResult::Valid(m) => {
            let t = m.undo(); // changed our minds, let's try again.. (although RandomPlayer choose Cell 1 again...)
            match r2.next_move(t) {
                MoveResult::Valid(m) => m.end_turn(),
                MoveResult::GiveUp(_) => panic!("{} gave up!", r2.0),
            }
        }
        MoveResult::GiveUp(_) => panic!("{} gave up!", r2.0),
    }
    loop {
        // should give up when r1 tries both Cells 0 & 1
        run_turn(&mut r1, &mut gs);
        run_turn(&mut r2, &mut gs);
    }
}
