const GAME_SOURCE: &str = "src/**/*.rs";

fn main() {
    sedona_ecs::build_ecs(GAME_SOURCE);
}
