use crate::cli::Args;

pub struct Passes {
    args: Args,
}

impl Passes {
    pub fn new(args: Args) -> Passes {
        Passes {
            args,
        }
    }

    pub fn blah(&mut self) {

    }

    pub fn blah_blah(&mut self) {
        panic!("{:?}", self.args);
    }
}
