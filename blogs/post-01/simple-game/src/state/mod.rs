//! State contains all the game state and logic.

use bracket_lib::prelude as blib;

const TOP_SCREEN_PIXEL: isize = 8;
const BOX_HEIGHTWIDTH: isize = 5;
const GROUND_PIXEL: isize = 45;
const GROUND_WIDTH: isize = 80;
const GAME_WINDOW: isize = 50;
const GROUND_COLLISION: isize = GROUND_PIXEL - BOX_HEIGHTWIDTH;

/// Moving represents the set of possible moving options.
enum Moving {
    Not,
    Up,
    Down,
}

// =============================================================================

/// State represents the game state for the game.
pub struct State {
    box_y: isize,       // Box's vertical position.
    box_moving: Moving, // Direction the box is moving.
}

/// new constructs a new game state.
pub fn new() -> State {
    return State {
        box_y: GROUND_COLLISION,
        box_moving: Moving::Not,
    };
}

/// State implementation of the GameState trait.
impl blib::GameState for State {
    fn tick(&mut self, bterm: &mut blib::BTerm) {
        self.keyboard_input(bterm);
        self.render(bterm);
    }
}

/// Method set for the State type.
impl State {
    /// keyboard_input handles the processing of keyboard input.
    fn keyboard_input(&mut self, rltk: &mut blib::BTerm) {
        match rltk.key {
            None => {}
            Some(blib::VirtualKeyCode::Space) => {
                if self.box_y == GROUND_COLLISION {
                    self.box_moving = Moving::Up;
                }
            }
            _ => {}
        };
    }

    /// render takes the current game state and renders the screen.
    fn render(&mut self, bterm: &mut blib::BTerm) {
        bterm.cls_bg(blib::RGB::named(blib::WHITE));

        bterm.draw_bar_horizontal(
            0,                              // x
            TOP_SCREEN_PIXEL,               // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            blib::RGB::named(blib::YELLOW), // foreground color
            blib::RGB::named(blib::YELLOW), // background color
        );

        bterm.draw_bar_horizontal(
            0,                              // x
            GROUND_PIXEL,                   // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            blib::RGB::named(blib::YELLOW), // foreground color
            blib::RGB::named(blib::YELLOW), // background color
        );

        bterm.draw_box_double(
            10,                          // x
            self.box_y,                  // y
            BOX_HEIGHTWIDTH,             // width
            BOX_HEIGHTWIDTH,             // height
            blib::RGB::named(blib::RED), // foreground color
            blib::RGB::named(blib::RED), // background color
        );

        match self.box_moving {
            Moving::Down => {
                self.box_y += 1;
                if self.box_y == GROUND_COLLISION {
                    self.box_moving = Moving::Not;
                }
            }
            Moving::Up => {
                self.box_y -= 1;
                if self.box_y == TOP_SCREEN_PIXEL {
                    self.box_moving = Moving::Down;
                }
            }
            _ => {}
        }

        bterm.draw_box(
            36,
            0,
            20,
            3,
            blib::RGB::named(blib::WHITE),
            blib::RGB::named(blib::BLACK),
        );

        bterm.printer(
            55,
            1,
            &format!("#[pink]FPS: #[]{}", bterm.fps),
            blib::TextAlign::Right,
            None,
        );

        bterm.printer(
            55,
            5,
            &format!("#[pink]Frame Time: #[]{} ms", bterm.frame_time_ms),
            blib::TextAlign::Right,
            None,
        );
    }
}
