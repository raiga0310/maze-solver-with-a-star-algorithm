use dungeon::{create_dungeon, display_dungeon, search_dungeon};

mod dungeon;

fn main() {
    let dungeon = create_dungeon();
    display_dungeon(&dungeon);
    search_dungeon(dungeon);
}
