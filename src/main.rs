use std::vec;
use std::{thread, time::Duration};

#[derive(Debug, Clone)]
struct Cell {
    alive: bool,
}

struct Board {
    height: usize,
    width: usize,
    board: Vec<Vec<Cell>>,
}

impl Board {
    fn new(height: usize, width: usize) -> Self {
        let default_cell = Cell { alive: false };
        Self {
            width: width,
            height: height,
            board: vec![vec![default_cell; width]; height],
        }
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for y in 0..self.height {
            print!("\n");
            for x in 0..self.width {
                if !self.board[x][y].alive {
                    print!("_ ");
                } else {
                    print!("* ");
                }
            }
        }
    }

    fn spawn(&mut self, x: usize, y: usize) {
        if y < self.height && x < self.width {
            self.board[x][y].alive = true;
        }
    }

    fn update(&mut self) {
        let mut new_board: Vec<Vec<Cell>> = self.board.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbor_cnt = count_neighbors(self, x, y);

                if !self.board[x][y].alive {
                    if neighbor_cnt == 3 {
                        new_board[x][y].alive = true;
                    }
                } else {
                    if neighbor_cnt < 2 || neighbor_cnt > 3 {
                        new_board[x][y].alive = false;
                    }
                }
            }
        }
        self.board = new_board;
    }
}

fn count_neighbors(board: &mut Board, x: usize, y: usize) -> i32 {
    let directions: [(isize, isize); 8] = [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
    let mut neighbor_cnt = 0;
    for (dx, dy) in directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || nx >= board.width as isize || ny < 0 || ny >= board.height as isize {
            continue;
        }
        if board.board[nx as usize][ny as usize].alive {
            neighbor_cnt += 1;
        }
    }
    return neighbor_cnt
}

fn main() {
    let mut board = Board::new(50, 50);
    board.spawn(1, 1);
    board.spawn(2, 2);
    board.spawn(3, 2);
    board.spawn(3, 1);
    board.spawn(3, 0);

    loop {
        board.display();
        board.update();
        thread::sleep(Duration::from_millis(1000));
    }    
}
