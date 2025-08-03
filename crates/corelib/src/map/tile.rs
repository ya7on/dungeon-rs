/// Heap allocated array of tiles.
pub(crate) type Tiles = Box<[Tile]>;

#[derive(Debug, Clone)]
#[repr(u8)]
pub(crate) enum Tile {
    Empty = 0,
}
