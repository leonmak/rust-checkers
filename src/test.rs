#[cfg(test)]
mod tests {
    use board::{Coordinate, Move};
    use game::GameEngine;
    use ::{get_piece, move_piece};

    #[test]
    fn jumps_target() {
        let targets = Coordinate(2, 2).jump_targets_from();
        assert_eq!(targets.count(), 4)
    }

    #[test]
    fn moves_target() {
        let targets = Coordinate(1, 1).move_targets_from();
        assert_eq!(targets.count(), 4)
    }

    #[test]
    fn test_move() {
        let mut engine = GameEngine::new();
        let mv = Move::new((0, 5), (1, 4));
        let res = engine.move_piece(&mv);
        assert!(res.is_ok())
    }

    #[test]
    fn test_move_err() {
        let mut engine = GameEngine::new();
        let mv = Move::new((1, 4), (2, 3));
        let res = engine.move_piece(&mv);
        assert!(res.is_err())
    }

}