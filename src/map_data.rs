pub struct MapData {
    width: usize,
    height: usize,
    tiles: Vec<u8>,
}

impl MapData {
    pub fn new(width: usize, height: usize) -> MapData {
        MapData {
            width,
            height,
            tiles: vec![0; width * height],
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_tile(&mut self, x: usize, y: usize, value: u8) {
        self.tiles[y * self.width + x] = value;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> u8 {
        self.tiles[y * self.width + x]
    }
}