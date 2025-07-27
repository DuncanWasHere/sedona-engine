use chrono::Local;
use game_ecs::BigBerg;
use sedona_app::App;
use std::fs::File;
use std::io::Write;

fn main() {
    let target = Box::new(File::create("log.txt").expect("Can't create file"));

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Warn)
        .filter_module("sedona", log::LevelFilter::Debug)
        .filter_module("game", log::LevelFilter::Debug)
        .init();

    let game = BigBerg::default();

    let mut app = App::new(game);

    app.run();
}
