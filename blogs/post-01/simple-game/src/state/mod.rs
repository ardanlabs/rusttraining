//! State contains all the game state and logic.

use bracket_lib::prelude as blib;

const GAME_WINDOW_HEIGHT: isize = 50;
const GAME_WINDOW_WIDTH: isize = 80;
const BOX_HEIGHT: isize = 5;
const BOX_WIDTH: isize = 5;
const CEILING_POS: isize = 5;
const FLOOR_POS: isize = 45;
const CEILING_COLLISION: isize = CEILING_POS + 1;
const GROUND_COLLISION: isize = FLOOR_POS - BOX_HEIGHT - 1;

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
    fn keyboard_input(&mut self, bterm: &mut blib::BTerm) {
        match bterm.key {
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
            CEILING_POS,                    // y
            GAME_WINDOW_WIDTH,              // width
            GAME_WINDOW_HEIGHT,             // n
            GAME_WINDOW_HEIGHT,             // max
            blib::RGB::named(blib::YELLOW), // foreground color
            blib::RGB::named(blib::YELLOW), // background color
        );

        bterm.draw_bar_horizontal(
            0,                              // x
            FLOOR_POS,                      // y
            GAME_WINDOW_WIDTH,              // width
            GAME_WINDOW_HEIGHT,             // n
            GAME_WINDOW_HEIGHT,             // max
            blib::RGB::named(blib::YELLOW), // foreground color
            blib::RGB::named(blib::YELLOW), // background color
        );

        bterm.draw_box_double(
            10,                          // x
            self.box_y,                  // y
            BOX_WIDTH,                   // width
            BOX_HEIGHT,                  // height
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
                if self.box_y == CEILING_COLLISION {
                    self.box_moving = Moving::Down;
                }
            }
            _ => {}
        }
    }
}
