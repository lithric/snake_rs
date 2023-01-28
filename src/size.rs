use crate::Pos;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Size {
    WH(usize,usize),
    None
}

impl Size {
    pub fn area(&self) -> usize {
        match self {
            Size::WH(w,h) => {
                w*h
            },
            Size::None => {
                0usize
            }
        }
    }
    pub fn width(&self) -> usize {
        match self {
            Size::WH(w,_) => {
                *w
            },
            Size::None => {
                0usize
            }
        }
    }
    pub fn width_i32(&self) -> i32 {
        self.width() as i32
    }
    pub fn height(&self) -> usize {
        match self {
            Size::WH(_,h) => {
                *h
            },
            Size::None => {
                0usize
            }
        }
    }
    pub fn height_i32(&self) -> i32 {
        self.height() as i32
    }
    pub fn tuple(&self) -> (usize,usize) {
        match self {
            Size::WH(w,h) => {
                (*w,*h)
            },
            Size::None => {
                (0usize,0usize)
            }
        }
    }
    pub fn tuple_i32(&self) -> (i32,i32) {
        let tuple = self.tuple();
        (tuple.0 as i32, tuple.1 as i32)
    }
}

impl std::cmp::Ord for Size {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

impl std::cmp::PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }
}

impl std::cmp::PartialOrd<Pos> for Size {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        Some(self.tuple_i32().cmp(&other.xy()))
    }
}

impl std::cmp::PartialEq<Pos> for Size {
    fn eq(&self, other: &Pos) -> bool {
        self.tuple_i32() == other.xy()
    }
}

impl std::cmp::Eq for Size {}