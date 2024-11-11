mod game_of_life;

use game_of_life::GameOfLife;

pub fn status(item: GameOfLife) -> Vec<Vec<u64>> {
    item.status()
}

pub fn init(width: u8, height: u8) -> GameOfLife {
    GameOfLife::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sut = init(1, 1);
        let result = status(sut);
        let expected: Vec<Vec<u64>> = vec![vec![0]];
        assert_eq!(result, expected)
    }
}
