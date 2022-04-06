fn main() {
    // println!("\x1b[1;4;92mHello\x1b[0m");
    // for i in 0..150 {
    //     println!("\x1b[{}m{}\x1b[0m", i, i);
    // }

    let a = Vec2D(1, 2);
    let b = Vec2D(4, 5);

    println!("{:?}", a);
    println!("{:?}", b);

    let c = a + b;

    println!("{:?}", c);
}

#[derive(Debug)]
struct Vec2D(u32, u32);

impl std::ops::Add for Vec2D {
    type Output = Vec2D;
    fn add(self, other: Self) -> Self::Output {
        Vec2D(self.0 + other.0, self.1 + other.1)
    }
}
