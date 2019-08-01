use structopt::StructOpt;

mod app;
mod error;
mod filesystem;
mod grid;

use app::{App, Options};

fn main() {
    let options = Options::from_args();
    let app = App::new(&options);
    if let Err(e) = app.run() {
        println!("{}", e);
    };
}
