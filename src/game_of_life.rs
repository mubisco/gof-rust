pub struct GameOfLife {
    board: Vec<Vec<u64>>
}

impl GameOfLife {
    pub fn new(width: u8, height: u8) -> GameOfLife {
        let board = vec![vec![0; width as usize]; height as usize];
        GameOfLife {board}
    }

    pub fn status(&self) -> Vec<Vec<u64>> {
        self.board.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_init() {
        let result = GameOfLife::new(1, 1);
        let expected: Vec<Vec<u64>> = vec![vec![0]];
        assert_eq!(result.status(), expected)
    }

    #[test]
    fn it_should_init_with_bigger_data() {
        let result = GameOfLife::new(3, 3);
        let expected: Vec<Vec<u64>> = [
            [0, 0, 0].to_vec(),
            [0, 0, 0].to_vec(),
            [0, 0, 0].to_vec()
        ].to_vec();
        assert_eq!(result.status(), expected)
    }
}
