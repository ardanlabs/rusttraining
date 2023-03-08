//! State contains all the game state and logic.

use bracket_lib::prelude::*;

const GAME_WINDOW_HEIGHT: isize = 50;
const GAME_WINDOW_WIDTH: isize = 80;
const BOX_HEIGHT: isize = 5;
const BOX_WIDTH: isize = 5;
const CEILING_POS: isize = 5;
const FLOOR_POS: isize = 45;
const CEILING_COLLISION: isize = CEILING_POS + 1;
const FLOOR_COLLISION: isize = FLOOR_POS - BOX_HEIGHT - 1;

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
        box_y: FLOOR_COLLISION,
        box_moving: Moving::Not,
    };
}

/// State implementation of the GameState trait.
impl GameState for State {
    fn tick(&mut self, bterm: &mut BTerm) {
        self.keyboard_input(bterm);
        self.render(bterm);
    }
}

/// Method set for the State type.
impl State {
    /// keyboard_input handles the processing of keyboard input.
    fn keyboard_input(&mut self, bterm: &mut BTerm) {
        match bterm.key {
            None => {}
            Some(VirtualKeyCode::Space) => {
                if self.box_y == FLOOR_COLLISION {
                    self.box_moving = Moving::Up;
                }
            }
            _ => {}
        };
    }

    /// render takes the current game state and renders the screen.
    fn render(&mut self, bterm: &mut BTerm) {
        bterm.cls_bg(WHITE);

        bterm.draw_bar_horizontal(
            0,                  // x
            CEILING_POS,        // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );

        bterm.draw_bar_horizontal(
            0,                  // x
            FLOOR_POS,          // y
            GAME_WINDOW_WIDTH,  // width
            GAME_WINDOW_HEIGHT, // n
            GAME_WINDOW_HEIGHT, // max
            YELLOW,             // foreground color
            YELLOW,             // background color
        );

        bterm.draw_box_double(
            (GAME_WINDOW_WIDTH / 2) - 3, // x
            self.box_y,                  // y
            BOX_WIDTH,                   // width
            BOX_HEIGHT,                  // height
            RED,                         // foreground color
            RED,                         // background color
        );

        match self.box_moving {
            Moving::Down => {
                self.box_y += 1;
                if self.box_y == FLOOR_COLLISION {
                    self.box_moving = Moving::Not;
                }
            }
            Moving::Up => {
                self.box_y -= 1;
                if self.box_y == CEILING_COLLISION {
                    self.box_moving = Moving::Down;
                }
            }
            Moving::Not => {}
        }
    }
}
