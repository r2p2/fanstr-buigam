#![allow(dead_code)]

#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone,
}

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human,
}

struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    beings: Option<Being>,
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>,
}

enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(PartialEq, Debug)]
enum MovementError {
    NoBeingInSquare,
}

impl Grid {
    fn move_being_in_coord(&self, coord: (usize, usize), dir: Direction) -> Result<(usize, usize), MovementError> {
        let square = self.squares.get(coord.0 * self.size.0 + coord.1).expect("index out of bounds trying to get beeing");
        match square.beings {
            Some (_) => Ok((0,0)),
            None => Err(MovementError::NoBeingInSquare),
        }
    }

    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> =
            Vec::with_capacity(number_of_squares);
        for _ in 0..number_of_squares {
            squares.push(
                Square {
                    ground: TerrainGround::Soil,
                    block: None,
                    beings: None,
                });
        }

        Grid {
            size: (size_x, size_y),
            squares: squares,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = ::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, ::TerrainGround::Soil);
            assert_eq!(square.block, None);
            assert_eq!(square.beings, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), 5*13);
        assert_eq!(number_of_squares, 5*13);
    }

    #[test]
    fn test_move_being_without_beging_in_square() {
        let grid = ::Grid::generate_empty(3, 3);
        assert_eq!(grid.move_being_in_coord((0,0), ::Direction::West), Err(::MovementError::NoBeingInSquare));
    }
}
