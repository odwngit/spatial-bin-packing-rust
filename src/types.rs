use rand::Rng;

#[derive(Clone)]
pub struct Bin {
    pub width: i32,
    pub length: i32,
    pub height: i32,
    pub volume: Vec<u8>
}

impl Bin {
    pub fn new(w: i32, l: i32, h: i32) -> Bin {
        Bin {
            width: w,
            length: l,
            height: h,
            volume: vec![0; (w*l*h) as usize]
        }
    }

    pub fn get_cell(&mut self, x: i32, y: i32, z: i32) -> u8 {
        self.volume[(x + y * self.width + z * self.width * self.height) as usize]
    }

    pub fn set_cell(&mut self, x: i32, y: i32, z: i32, value: u8) -> u8 {
        self.volume[(x + y * self.width + z * self.width * self.height) as usize] = value;
        return self.get_cell(x, y, z)
    }

    pub fn fit_item(&mut self, item: Item) -> bool {
        for x in 0..self.width-item.width+1 {
            for y in 0..self.height-item.height+1 {
                for z in 0..self.length-item.length+1 {
                    // For every position
                    let mut free = true;
                    'check: for xi in x..item.width+x {
                        for yi in y..item.height+y {
                            for zi in z..item.length+z {
                                // For every cell inside the item check space
                                if self.get_cell(xi, yi, zi) > 0 {
                                    free = false;
                                    break 'check;
                                }
                            }
                        }
                    }
                    
                    if free {
                        println!("Fit a box!");
                        let w = item.width;
                        let h = item.height;
                        let l = item.length;
                        println!("[({x},{z},{y}),({w},{l},{h})]");
                        let value = rand::thread_rng().gen_range(1..255);
                        for xi in x..item.width {
                            for yi in y..item.height {
                                for zi in z..item.length {
                                    self.set_cell(xi, yi, zi, value);
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
                    if self.get_cell(x, y, z) > 0 {
                        points.push(Point {
                            x: x,
                            y: y,
                            z: z,
                            value: self.get_cell(x, y, z)
                        })
                    }
                }
            }
        }
        points.push(Point { x: 0, y: 0, z: 0, value: 0 });
        points.push(Point { x: self.width, y: 0, z: 0, value: 0 });
        points.push(Point { x: self.width, y: self.height, z: 0, value: 0 });
        points.push(Point { x: self.width, y: self.height, z: self.length, value: 0 });
        points.push(Point { x: 0, y: self.height, z: self.length, value: 0 });
        points.push(Point { x: 0, y: 0, z: self.length, value: 0 });
        points.push(Point { x: 0, y: self.height, z: 0, value: 0 });
        points.push(Point { x: self.width, y: 0, z: self.length, value: 0 });

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
            str.push_str(format!("{} {} {} 0 0 {}\n", point.x, point.y, point.z, point.value).as_str())
        }

        return str;
    }
}

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

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub value: u8
}