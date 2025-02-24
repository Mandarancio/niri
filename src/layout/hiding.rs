use super::scrolling::ColumnWidth;
use super::tile::Tile;
use super::{LayoutElement, RemovedTile};
use crate::animation::Clock;

/// Space for floating windows.
#[derive(Debug)]
pub struct HidingSpace<W: LayoutElement> {
    /// Tiles in top-to-bottom order.
    tiles: Vec<Tile<W>>,

    /// Clock for driving animations.
    clock: Clock,
}

impl<W: LayoutElement> HidingSpace<W> {
    pub fn new(clock: Clock) -> Self {
        Self {
            tiles: Vec::new(),
            clock,
        }
    }

    pub fn update_shaders(&mut self) {
        for tile in &mut self.tiles {
            tile.update_shaders();
        }
    }

    pub fn advance_animations(&mut self) {
        for tile in &mut self.tiles {
            tile.advance_animations();
        }
    }

    pub fn are_animations_ongoing(&self) -> bool {
        self.tiles.iter().any(Tile::are_animations_ongoing)
    }

    pub fn are_transitions_ongoing(&self) -> bool {
        self.tiles.iter().any(Tile::are_transitions_ongoing)
    }

    fn idx_of(&self, id: &W::Id) -> Option<usize> {
        self.tiles.iter().position(|tile| tile.window().id() == id)
    }

    fn contains(&self, id: &W::Id) -> bool {
        self.idx_of(id).is_some()
    }
    pub fn has_window(&self, id: &W::Id) -> bool {
        self.tiles.iter().any(|tile| tile.window().id() == id)
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    pub fn add_tile(&mut self, tile: Tile<W>) {
        self.add_tile_at(0, tile);
    }

    pub fn tiles(&self) -> impl Iterator<Item = &Tile<W>> + '_ {
        self.tiles.iter()
    }

    fn add_tile_at(&mut self, idx: usize, tile: Tile<W>) {
        self.tiles.insert(idx, tile);
    }

    pub fn remove_tile(&mut self, id: &W::Id) -> RemovedTile<W> {
        let idx = self.idx_of(id).unwrap();
        self.remove_tile_by_idx(idx)
    }

    fn remove_tile_by_idx(&mut self, idx: usize) -> RemovedTile<W> {
        let tile = self.tiles.remove(idx);

        let width = ColumnWidth::Fixed(tile.tile_expected_or_current_size().w);
        RemovedTile {
            tile,
            width,
            is_full_width: false,
            is_floating: true,
        }
    }
}
