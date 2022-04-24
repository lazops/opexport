mod consts;
mod export;
mod format;
mod model;
mod op;

use std::io;

use model::Model;

fn main() -> io::Result<()> {
    rustea::run(Model::new())
}
