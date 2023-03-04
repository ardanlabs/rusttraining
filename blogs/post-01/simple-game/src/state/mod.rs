//! State contains all the game state and logic.

const TOP_SCREEN_PIXEL: i32 = 8;
const BOX_HEIGHTWIDTH: i32 = 5;
const GROUND_PIXEL: i32 = 45;
const GROUND_WIDTH: i32 = 80;
const GAME_WINDOW: i32 = 50;
const GROUND_COLLISION: i32 = GROUND_PIXEL - BOX_HEIGHTWIDTH;

/// Moving represents the set of possible moving options.
enum Moving {
    Not,
    Up,
    Down,
}

// =============================================================================

/// State represents the game state for the game.
pub struct State {
    box_y: i32,         // Box's vertical position.
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
impl rltk::GameState for State {
    fn tick(&mut self, bterm: &mut rltk::BTerm) {
        self.keyboard_input(bterm);
        self.render(bterm);
    }
}

/// Method set for the State type.
impl State {
    /// keyboard_input handles the processing of keyboard input.
    fn keyboard_input(&mut self, rltk: &mut rltk::Rltk) {
        match rltk.key {
            None => {}
            Some(rltk::VirtualKeyCode::Space) => {
                if self.box_y == GROUND_COLLISION {
                    self.box_moving = Moving::Up;
                }
            }
            _ => {}
        };
    }

    /// render takes the current game state and renders the screen.
    fn render(&mut self, bterm: &mut rltk::BTerm) {
        bterm.cls_bg(rltk::RGB::named(rltk::WHITE));

        bterm.draw_bar_horizontal(
            0,                              // x
            TOP_SCREEN_PIXEL,               // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            rltk::RGB::named(rltk::YELLOW), // foreground color
            rltk::RGB::named(rltk::YELLOW), // background color
        );

        bterm.draw_bar_horizontal(
            0,                              // x
            GROUND_PIXEL,                   // y
            GROUND_WIDTH,                   // width
            GAME_WINDOW,                    // n
            GAME_WINDOW,                    // max
            rltk::RGB::named(rltk::YELLOW), // foreground color
            rltk::RGB::named(rltk::YELLOW), // background color
        );

        bterm.draw_box_double(
            10,                          // x
            self.box_y,                  // y
            BOX_HEIGHTWIDTH,             // width
            BOX_HEIGHTWIDTH,             // height
            rltk::RGB::named(rltk::RED), // foreground color
            rltk::RGB::named(rltk::RED), // background color
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
            rltk::RGB::named(rltk::WHITE),
            rltk::RGB::named(rltk::BLACK),
        );

        bterm.printer(
            55,
            1,
            &format!("#[pink]FPS: #[]{}", bterm.fps),
            rltk::TextAlign::Right,
            None,
        );

        bterm.printer(
            55,
            5,
            &format!("#[pink]Frame Time: #[]{} ms", bterm.frame_time_ms),
            rltk::TextAlign::Right,
            None,
        );
    }
}
