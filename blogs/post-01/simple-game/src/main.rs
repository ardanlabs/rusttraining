use bracket_lib::terminal::{main_loop, BTermBuilder};
mod state;

fn main() {
    let result = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build();

    match result {
        Ok(bterm) => {
            let _ = main_loop(bterm, state::new());
        }
        Err(err) => {
            println!("An error has occurred: {err:?}");
            println!("Aborting");
        }
    }
}
