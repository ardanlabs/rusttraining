pub mod state;

fn main() {
    let result = rltk::BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build();

    match result {
        Ok(bterm) => {
            let _ = rltk::main_loop(bterm, state::new());
        }
        Err(err) => {
            println!("An error has occurred: {err:?}");
            println!("Aborting");
        }
    }
}
