extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate docopt;
use piston::window::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{Filter, GlGraphics, OpenGL, TextureSettings};
use opengl_graphics::glyph_cache::GlyphCache;
mod board;
mod board_controller;
mod board_view;

use board::Board;
use board_controller::BoardController;
use board_view::{BoardView, BoardViewSetting};
use docopt::Docopt;

static USAGE: &'static str = "
Usage:
    cross --size <width> <height> [--random]
    cross (-h | --help)
Options:
    --random    Randomized Walls
    -h, --help  Show this message.
";

#[derive(Deserialize, Debug)]
struct Args {
    arg_width: usize,
    arg_height: usize,
    flag_random: Option<bool>,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build().expect("could not create window");
    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let board = if let Some(_random) = args.flag_random {
        
        Board::randomized(args.arg_width, args.arg_height)
    } else {
        Board::with_walls(args.arg_width, args.arg_height, vec![])
    };
    let mut controller = BoardController::new(board);

    let view_setting = BoardViewSetting::new();
    let view = BoardView::new(view_setting);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = &mut GlyphCache::new("resources/mplus-2c-regular.ttf", texture_settings)
        .expect("could not load font");
    while let Some(e) = events.next(&mut window) {
        controller.event(view.setting.position, view.setting.size, &e);
        let ended = controller.the_game_ended();
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                clear([1.0; 4], g);
                view.draw(&controller, &c, g);
                view.draw_score(ended, &controller, glyphs, &c, g);
            });
        }
    }
}
