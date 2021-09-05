mod cli;
mod passes;
mod files;

use passes::Passes;
use cli::Args;

fn main() {
    // Useful for dumping args for examination
    //let args: Vec<String> = std::env::args().collect();
    //panic!("args: {:?}", args);

    let mut passes = Passes::new(Args::parse());

    passes.blah();

    passes.blah_blah();
}
