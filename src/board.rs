use graphics::types::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Empty,
    Hole,
    Ink(Color),
}

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}
pub struct Board {
    pub cells: Vec<Vec<CellState>>,
    pub width:usize,
    pub height:usize,
    pub walls:Vec<(usize,usize)>
}

impl Board {
    pub fn randomized(width:usize, height:usize) -> Self {
        use rand;
        use rand::distributions::{IndependentSample, Range};
        let between = Range::new(0, 10);
        let mut rng = rand::thread_rng();
        let mut walls = Vec::new();
        for w in 0..width {
            for h in 0..height {
                let a = between.ind_sample(&mut rng);
                if a < 2 {
                    walls.push((h+1,w+1))
                }
            }
        }
        Self::with_walls(width, height, walls)
    }

    // wall(y,x)
    pub fn with_walls(width:usize, height:usize, walls:Vec<(usize, usize)>) -> Self {
        let mut cells = vec![vec![CellState::Empty; width]; height];
        for &(h, v) in &walls {
            cells[(h as isize -1).max(0) as usize][(v as isize -1).max(0) as usize] = CellState::Hole;
        }
        Self {cells,width,height, walls}
    }

    pub fn paint_cells(&mut self, index: [usize; 2], color: Color, dir: Direction) {
        if index[0] < self.width && index[1] < self.height {
            match self.cells[index[1]][index[0]] {
                CellState::Empty | CellState::Ink(_) => {
                    self.cells[index[1]][index[0]] = CellState::Ink(color);
                    match dir {
                        Direction::None => {
                            self.paint_cells([index[0] + 1, index[1]], color, Direction::Right);
                            if index[0] != 0 {
                                self.paint_cells([index[0] - 1, index[1]], color, Direction::Left);
                            }
                            if index[1] != 0 {
                                self.paint_cells([index[0], index[1] - 1], color, Direction::Up);
                            }
                            self.paint_cells([index[0], index[1] + 1], color, Direction::Down);
                        }
                        Direction::Up => if index[1] != 0 {
                            self.paint_cells([index[0], index[1] - 1], color, dir);
                        },
                        Direction::Down => {
                            self.paint_cells([index[0], index[1] + 1], color, dir);
                        }
                        Direction::Left => if index[0] != 0 {
                            self.paint_cells([index[0] - 1, index[1]], color, dir);
                        },
                        Direction::Right => {
                            self.paint_cells([index[0] + 1, index[1]], color, dir);
                        }
                    }
                }
                CellState::Hole => {
                    return;
                }
            }
        }
    }

    pub fn scores(&self, color_1: Color, color_2: Color) -> (usize, usize) {
        let mut score_1 = 0;
        let mut score_2 = 0;
        for row in &self.cells {
            for col in row.iter() {
                if let CellState::Ink(color) = *col {
                    if color == color_1 {
                        score_1 += 1;
                    }
                    if color == color_2 {
                        score_2 += 1;
                    }
                }
            }
        }
        (score_1, score_2)
    }
}
