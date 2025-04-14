use rand::Rng;

#[derive(Clone)]
pub struct Bin {
    pub width: i32,
    pub length: i32,
    pub height: i32,
    pub volume: Vec<Cell>
}

impl Bin {
    pub fn new(w: i32, l: i32, h: i32) -> Bin {
        Bin {
            width: w,
            length: l,
            height: h,
            volume: vec![Cell::EMPTY; (w*l*h) as usize]
        }
    }

    pub fn get_cell(&mut self, x: i32, y: i32, z: i32) -> Cell {
        self.volume[(x + y * self.width + z * self.width * self.height) as usize].clone()
    }

    pub fn set_cell(&mut self, x: i32, y: i32, z: i32, value: Cell) {
        self.volume[(x + y * self.width + z * self.width * self.height) as usize] = value;
    }

    pub fn fit_item(&mut self, item: Item) -> bool {
        for y in 0..self.height-item.height {
            for x in 0..self.width-item.width {
                for z in 0..self.length-item.length {
                    // For every position
                    let mut free = true;
                    'check: for xi in x..item.width+x {
                        for yi in y..item.height+y {
                            for zi in z..item.length+z {
                                // For every cell inside the item check space
                                if self.get_cell(xi, yi, zi).filled {
                                    free = false;
                                    break 'check;
                                }
                            }
                        }
                    }
                    
                    if free {
                        let mut rng = rand::thread_rng();
                        let value = Cell {
                            filled: true,
                            color: Color::new(
                                rng.gen_range(0..255), 
                                rng.gen_range(0..255), 
                                rng.gen_range(0..255)
                            )
                        };
                        for xi in x..item.width+x {
                            for yi in y..item.height+y {
                                for zi in z..item.length+z {
                                    self.set_cell(xi, yi, zi, value.clone());
                                }
                            }
                        }

                        return true
                    }
                }
            }
        }

        return false // Could not find a fit
    }

    pub fn ply_export(&mut self) -> String {
        let mut points: Vec<Point> = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                for z in 0..self.length {
                    if self.get_cell(x, y, z).filled {
                        points.push(Point {
                            x: x,
                            y: y,
                            z: z,
                            color: self.get_cell(x, y, z).color
                        })
                    }
                }
            }
        }
        points.push(Point { x: 0, y: 0, z: 0, color: Color::BLACK});
        points.push(Point { x: self.width, y: 0, z: 0, color: Color::BLACK});
        points.push(Point { x: self.width, y: self.height, z: 0, color: Color::BLACK});
        points.push(Point { x: self.width, y: self.height, z: self.length, color: Color::BLACK});
        points.push(Point { x: 0, y: self.height, z: self.length, color: Color::BLACK});
        points.push(Point { x: 0, y: 0, z: self.length, color: Color::BLACK});
        points.push(Point { x: 0, y: self.height, z: 0, color: Color::BLACK});
        points.push(Point { x: self.width, y: 0, z: self.length, color: Color::BLACK});

        let mut str = String::from(
            "ply
format ascii 1.0
comment Output from spatial-bin-packing-rust
element vertex "
        );
        str.push_str(points.len().to_string().as_str());
        str.push('\n');
        str.push_str(
            "property int x
property int y
property int z
property uchar red
property uchar green
property uchar blue
element face 0
end_header\n"
        );
        

        for point in points {
            str.push_str(format!("{} {} {} {} {} {}\n", point.x, point.y, point.z, point.color.red, point.color.green, point.color.blue).as_str())
        }

        return str;
    }
}

#[derive(Clone)]
pub struct Item {
    pub width: i32,
    pub length: i32,
    pub height: i32
}

impl Item {
    pub const fn new(w: i32, l: i32, h: i32) -> Item {
        Item {
            width: w,
            length: l,
            height: h
        }
    }
}

#[derive(Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub const BLACK: Color = Color { red: 0, blue: 0, green: 0 };

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { red: r, green: g, blue: b }
    }
}

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: Color
}

#[derive(Clone)]
pub struct Cell {
    pub filled: bool,
    pub color: Color 
}

impl Cell {
    pub const EMPTY: Cell = Cell {
        filled: false,
        color: Color::BLACK
    };
}