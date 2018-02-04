use piston::input::GenericEvent;
use board::Board;
use graphics::types::Color;
use board::CellState;
pub struct BoardController {
    pub board: Board,
    pub teban: [Color;2],
    pub teban_index: usize,
    pub selected_cell: Option<[usize; 2]>,
    pub last_teban_cell: Option<[usize; 2]>,
    pub ended: bool,
    pub cursor_pos: [f64; 2],
}


impl BoardController {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            teban: [[240.0 / 255.0 , 45.0 / 255.0, 125.0 / 255.0, 1.0], [ 25.0 / 255.0 , 215.0 / 255.0, 25.0 / 255.0,1.0]],
            teban_index: 0,
            selected_cell: None,
            last_teban_cell: None,
            ended: false,
            cursor_pos: [0.0;2],
        }
    }

    pub fn the_game_ended(&self) -> bool {
        self.ended
    }

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Key, Button, MouseButton};

        if let Some(Button::Keyboard(Key::Return)) = e.release_args() {
            self.ended = false;
            self.board = Board::with_walls(self.board.width, self.board.height, self.board.walls.clone());
            self.teban_index = 0;
            self.selected_cell = None;
            self.last_teban_cell = None;
            return;
        }
        
        if let Some(mouse_pos) = e.mouse_cursor_args() {
            self.cursor_pos = mouse_pos;
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            let aspect = self.board.width as f64 / self.board.height as f64;

            if x>= 0.0 && x < size && y >= 0.0 && y < size / aspect {
                let cell_x = (x / size * self.board.width as f64) as usize;
                let cell_y = (y / (size / aspect)* self.board.height as f64) as usize;

                self.selected_cell = Some([cell_x, cell_y]);
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            if let Some(cell) = self.selected_cell {
                use board::{Direction};
                if self.board.cells[cell[1]][cell[0]] == CellState::Empty {
                    self.last_teban_cell = Some(cell);
                    self.board.paint_cells(cell, self.teban[self.teban_index], Direction::None);
                    self.teban_index = {
                        (self.teban_index + 1) % 2
                    };
                }
            }
        }
        let ended = {
            let mut c = true;
            for row in &self.board.cells {
                for cell in row.iter() {
                    if cell == &CellState::Empty {
                        c = false;
                    }
                }
            }
            c
        };

        self.ended = ended;
    }
}