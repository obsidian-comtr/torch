#[path="./handlers/ConfigHandler.rs"]
mod config_handler;

#[path="./handlers/StructureHandler.rs"]
mod structure_handler;

fn main() {
    if structure_handler::init() {
        config_handler::check();
    }
}
