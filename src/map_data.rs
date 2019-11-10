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

pub fn load_tiled_map<P: AsRef<std::path::Path>>(map_path: P) -> std::io::Result<MapData> {
    let content = std::fs::read_to_string(map_path)?;
    let parsed = json::parse(content.as_str()).unwrap();

    let width = parsed["width"].as_usize().unwrap();
    let height = parsed["height"].as_usize().unwrap();
    let raw_data = &parsed["layers"][0]["data"];

    let mut data = MapData::new(width, height);

    for ix in 0..width {
        for iy in 0..height {
            let tid = raw_data[iy * width + ix].as_u8().unwrap() - 1;
            data.set_tile(ix, iy, tid);
        }
    }

    Ok(data)
}