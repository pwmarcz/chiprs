pub const DISPLAY_W: usize = 64;
pub const DISPLAY_H: usize = 32;

pub struct Display {
    pub pixels: [bool; DISPLAY_W * DISPLAY_H],
}

impl Display {
    pub fn new() -> Display {
        return Display {
            pixels: [false; DISPLAY_W * DISPLAY_H],
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, data: &[u8]) -> bool {
        let mut collision = false;

        for (i, b) in data.iter().enumerate() {
            for j in 0..8 {
                let xp = (x + j) % DISPLAY_W;
                let yp = (y + i) % DISPLAY_H;
                let index = yp * DISPLAY_W + xp;
                let val = (b & (1 << (7 - j))) != 0;

                if self.pixels[index] && val {
                    collision = true;
                }
                self.pixels[index] ^= val;
            }
        }
        collision
    }

    pub fn dump(&self) {
        for i in 0..DISPLAY_H {
            for j in 0..DISPLAY_W {
                let index = i * DISPLAY_W + j;
                print!("{}", if self.pixels[index] { "#" } else { "." });
            }
            println!();
        }
    }
}
