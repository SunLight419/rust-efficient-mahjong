fn main() {
    let mut v = vec![];
    for i in 0..13 {
        v.push(Tile::from_id(i * 4));
    }
    let mut hand = Hand::new(v.try_into().unwrap(), None);

    println!("{hand:?}");
}

enum Kind {
    Man,
    Pin,
    So,
    Ji,
}
impl Kind {
    fn new(x: u8) -> Kind {
        match x {
            0 => Kind::Man,
            1 => Kind::Pin,
            2 => Kind::So,
            3 => Kind::Ji,
            _ => panic!("Expected less than 4, actual {x}"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Tile {
    id: u8
}

impl Tile {
    fn from_id(id: u8) -> Self {
        Self {
            id
        }
    }
    #[inline]
    fn get_kind(&self) -> Kind {
        Kind::new(self.id >> 2)
    }
    #[inline]
    fn get_number(&self) -> u8 {
        self.id % 9
    }
}

#[derive(Debug)]
struct Hand {
    tiles: [Tile; 13],
    tumo: Option<Tile>,
}

impl Hand {
    fn new(tiles: [Tile; 13], tumo: Option<Tile>) -> Self {
        Self {
            tiles,
            tumo,
        }
    }

    fn sort(&mut self) -> () {
        self.tiles.sort();
    }
}