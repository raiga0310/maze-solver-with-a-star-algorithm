use dungeon::{Dungeon, display_dungeon, search_dungeon};

mod dungeon;

fn main() {
    let dungeon = Dungeon::new();
    display_dungeon(&dungeon.dungeon);
    search_dungeon(dungeon);
}
