mod consts;
mod export;
mod format;
mod model;
mod op;

use std::{env, fs};

use consts::INVALID_ARGUMENTS_MESSAGE;
use format::ux::UXExporter;
use model::Model;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let len = args.len();
    if len == 1 {
        rustea::run(Model::new()).unwrap();
    } else if len == 2 {
        let export_data = UXExporter::new().get_export_data().unwrap();
        let json = export_data.to_json().unwrap();
        let path = &args[1];
        fs::write(path, json).unwrap();
    } else {
        eprintln!("{}", INVALID_ARGUMENTS_MESSAGE);
    }
}
