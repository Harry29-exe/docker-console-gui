fn main() {
    println!("Hello, world!");
    let pyramid = Pyramid {
        height: 4
    };
    pyramid.draw();
}

struct Pyramid {
    height: i32,

}

impl Pyramid {
    fn draw(&self) {
        let width = self.height * 2 - 1;

        for row in 0..self.height {
            let filled = row * 2 + 1;
            let empty = (width - filled) / 2;

            for col in 0..empty {
                print!("_");
            }

            for col in 0..filled {
                print!("#");
            }

            for col in 0..empty {
                print!("_");
            }

            println!();
        }
    }
}