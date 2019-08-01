use crate::app::Options;
use crate::filesystem::File;
use term_grid::{Direction, Filling, Grid, GridOptions};

fn grid() -> Grid {
    Grid::new(GridOptions {
        filling: Filling::Spaces(1),
        direction: Direction::LeftToRight,
    })
}

const LOCK: &str = "🔓";
const OPEN_LOCK: &str = "🔒";

fn format_file(file: &File, ctx: &Options) -> Vec<String> {
    let Options {
        human: readable, ..
    } = ctx;
    let icon = if file.readonly { LOCK } else { OPEN_LOCK };
    let name = format!("{} {}", icon, file.name.to_str().unwrap());
    let size = if *readable {
        format!("{}", file.readable())
    } else {
        format!("{}", file.bytes)
    };

    let path = file.path.to_str().unwrap();

    vec![name, size, path.to_string()]
}

pub fn display_files(files: Vec<File>, ctx: &Options) -> Grid {
    let mut grid = grid();
    let header = vec![
        String::from("Name"),
        String::from("Size"),
        String::from("Path"),
    ];
    let files = files.iter().flat_map(|file| format_file(file, ctx));
    header
        .into_iter()
        .chain(files)
        .for_each(|cell| grid.add(cell.into()));
    grid
}
