pub struct PPM {
    width: i32,
    height: i32,
    max: i32,
    pixels: Vec<i32>
}

impl PPM {
    pub fn new(w: i32, h: i32, m: i32) -> PPM {
        return PPM {width: w, height: h, max: m, pixels: Vec::new()};
    }

    #[allow(dead_code)]
    pub fn write_color(&self) {
        panic!("Not implemented!");
    }

    pub fn write_out(&self) {
        println!("Writing out image with:\nwidth {}\nheight {}\ndepth {}", self.width, self.height, self.max);
    }
}