
#[derive(Debug, PartialEq, Eq)]
pub enum Rock {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl Rock {
    pub fn win(self) -> i32 {
        if self == Rock::A {
            return 8;
        } else if self == Rock::B {
            return 9;
        } else {
            return 7;
        }
    }

    pub fn lose(self) -> i32 {
        if self == Rock::A {
            return 3;
        } else if self == Rock::B {
            return 1;
        } else {
            return 2;
        }
    }

    pub fn draw(self) -> i32 {
        if self == Rock::A {
            return 4;
        } else if self == Rock::B {
            return 5;
        } else {
            return 6;
        }
    }
}