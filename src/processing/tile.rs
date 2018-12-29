#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Tile {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Tile {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Tile {
        Tile {
            x,
            y,
            width,
            height,
        }
    }

    pub fn split(&self) -> Vec<Tile> {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        vec![
            Tile::new(self.x, self.y, half_width, half_height),
            Tile::new(self.x + half_width, self.y, self.width - half_width, half_height),
            Tile::new(self.x, self.y + half_height, half_width, self.height - half_height),
            Tile::new(self.x + half_width, self.y + half_height, self.width - half_width, self.height - half_height),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let tile = Tile::new(0, 0, 17, 15);
        let tiles = tile.split();

        assert_eq!(tiles[0], Tile::new(0, 0, 8, 7));
        assert_eq!(tiles[1], Tile::new(8, 0, 9, 7));
        assert_eq!(tiles[2], Tile::new(0, 7, 8, 8));
        assert_eq!(tiles[3], Tile::new(8, 7, 9, 8));
    }
}
