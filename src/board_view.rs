use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use board_controller::BoardController;

pub struct BoardViewSetting {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub border_color: Color,
    pub board_edge_color: Color,
    pub cell_edge_color: Color,
    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,

    pub selected_cell_color: Color,
    pub text_color: Color,
}

impl BoardViewSetting {
    pub fn new() -> Self {
        Self {
            position: [56.0; 2],
            size: 400.0,
            background_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.0, 1.0],
            board_edge_color: [0.0,0.0,0.0, 1.0],
            cell_edge_color: [0.0, 0.0, 0.0, 1.0],
            cell_edge_radius: 1.0,
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            selected_cell_color: [0.7, 0.7, 0.7, 0.5],
            text_color: [0.0,0.0,0.0,1.0]
        }
    }
}

pub struct BoardView {
    pub setting: BoardViewSetting,
}

impl BoardView {
    pub fn new(setting: BoardViewSetting) -> Self {
        Self{setting}
    }
    pub fn draw_score<G: Graphics,C>(&self,ended: bool, controller: &BoardController,glyphs: &mut C, c: &Context, g: &mut G) where C: CharacterCache<Texture = G::Texture>{
        use graphics::{Image, Transformed};
        let (score1, score2) = controller.board.scores(controller.teban[0], controller.teban[1]);
        let score1_image = Image::new_color(controller.teban[0]);
        let score2_image = Image::new_color(controller.teban[1]);
        let teban_image = Image::new_color([0.0, 0.0, 0.0, 1.0]);

        let score_text = score1.to_string();
        let score2_text = score2.to_string();
         let mut fixed = 0.0;
        for ch in score_text.chars() {
            let character = glyphs.character(60, ch);
            let pos = [
                self.setting.position[0] + fixed,
                self.setting.position[1] + self.setting.size + 50.0 - character.top(),
            ];
            fixed += character.width();
            let ch_x = pos[0] + character.left();
            let ch_y = pos[1];
            score1_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
        }
        fixed = 0.0;
        for ch in score2_text.chars() {
            let character = glyphs.character(60, ch);
            let pos = [
                self.setting.position[0] + self.setting.size + fixed - 60.0,
                self.setting.position[1] + self.setting.size + 50.0 - character.top(),
            ];
            fixed += character.width();
            let ch_x = pos[0] - character.left();
            let ch_y = pos[1];
            score2_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
        }

        let mut fixed = 0.0;
        let teban_text =  if let Some(cell) = controller.last_teban_cell {
                let cellx_str = (cell[1] + 1).to_string();
                let celly_str = (cell[0] + 1).to_string();
                let mut teban_text = if controller.teban_index % 2 == 1 { 
                    "先 手 ("
                } else {
                    "後 手 ("
                }.to_string();
                teban_text.push_str(&cellx_str);
                teban_text.push_str(", ");
                teban_text.push_str(&celly_str);
                teban_text.push_str(")");
                teban_text
            } else {
                "".to_string()
            };
        let result_text = if ended {
            if score1 == score2 {
                "引き分け".to_string()
            } else if score1 < score2 {
                "後手が勝利しました".to_string()
            } else {
                "先手が勝利しました".to_string()
            }
        } else {
            "".to_string()
        };

        for ch in teban_text.chars() {
            let character = glyphs.character(30, ch);
            let pos = [
                0.0 + fixed,
                40.0 - character.top() 
            ];
            let ch_x = pos[0];
            let ch_y = pos[1];
            if controller.teban_index % 2 == 1 {
                score1_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
            } else {
                score2_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
            }
            fixed += character.width();
        }
        fixed = 0.0;
        for ch in result_text.chars() {
            let character = glyphs.character(24, ch);
            let pos = [
                self.setting.position[0] + self.setting.size * 0.24 + fixed,
                self.setting.position[1] + self.setting.size + 50.0 - character.top()
            ];
            let ch_x = pos[0];
            let ch_y = pos[1];

            if score1 == score2 {
                teban_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
            } else if score1 < score2 {
                score2_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
            } else {
                score1_image.draw(character.texture, &c.draw_state, c.transform.trans(ch_x, ch_y), g);
            }
            fixed += character.width(); 
        }

    }
    pub fn draw<G: Graphics>(&self, controller: &BoardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};
        let settings = &self.setting;
        let width = controller.board.width;
        let height = controller.board.height;
        let board_rect = [
            settings.position[0], settings.position[1], settings.size, settings.size * (height as f64 / width as f64)
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);


        let cell_size = settings.size / width as f64;
        for x in 0..width {
            for y in 0..height {
                use board::CellState;
                let pos = [x as f64 * cell_size, y as f64 * cell_size];
                let cell_rect = [
                    settings.position[0] + pos[0], settings.position[1] + pos[1], cell_size, cell_size
                ];
                match controller.board.cells[y][x] {
                    CellState::Ink(color) => {
                        Rectangle::new(color).draw(cell_rect, &c.draw_state, c.transform, g);
                    }
                    CellState::Hole => {
                        Rectangle::new([0.0,0.0,0.0,1.0]).draw(cell_rect, &c.draw_state, c.transform, g);
                    }
                    _=>{}
                }
            }
        }

        if let Some(ind) = controller.selected_cell {
            use board::CellState;
            if controller.board.cells[ind[1]][ind[0]] == CellState::Empty {
                let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
                let cell_rect = [
                    settings.position[0] + pos[0], settings.position[1] + pos[1], cell_size, cell_size
                ];
                let tbn = controller.teban[controller.teban_index];
                Rectangle::new([tbn[0], tbn[1], tbn[2], 0.5]).draw(cell_rect, &c.draw_state, c.transform, g);
            }
        }

        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let aspect = width as f64 / height as f64;
        for i in 0..width{
            let x = settings.position[0] + i as f64 / width as f64 * settings.size;
            let y2 = settings.position[1] + settings.size /aspect;
            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);
        }

        for i in 0..height {
            let y = settings.position[1] + i as f64 / height as f64 * settings.size / aspect;
            let x2 = settings.position[0] + settings.size;
            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }


        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(board_rect, &c.draw_state, c.transform, g);
    }
}

