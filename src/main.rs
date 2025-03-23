#[derive(Debug, Clone)]
struct Cell {
    alive: bool,
}

struct Board {
    height: usize,
    width: usize,
    board: Vec<Vec<Cell>>
}

impl Board {
    fn new(height: usize, width: usize) -> Self {
        let default_cell = Cell { alive: false};
        Self {
            width: width,
            height: height,
            board: vec![vec![default_cell; width]; height],
        }
    }

    fn display(&self) {
        for x in 0..self.height {
            print!("\n");
            for y in 0..self.width {
                if !self.board[x][y].alive {
                    print!("_ ");
                } else {
                    print!("* ");
                }
            } 
        }
    }
}

fn main() {
    let board = Board::new(10, 10);
    board.display();
    print!("hello");
}
