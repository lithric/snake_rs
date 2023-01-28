use crate::Size;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum Pos {
    XYZ(i32,i32,i32),
    XY(i32,i32),
    XZ(i32,i32),
    X(i32),
    Y(i32),
    Z(i32),
    None
}

#[allow(dead_code)]
impl Pos {
    pub fn xyz(&self) -> (i32,i32,i32) {
        match self {
            Pos::XYZ(x,y,z) => {
                (*x,*y,*z)
            },
            Pos::XY(x,y) => {
                (*x,*y,0)
            },
            Pos::XZ(x,z) => {
                (*x,0,*z)
            },
            Pos::X(x) => {
                (*x,0,0)
            },
            Pos::Y(y) => {
                (0,*y,0)
            },
            Pos::Z(z) => {
                (0,0,*z)
            },
            Pos::None => {
                (0,0,0)
            }
        }
    }
    pub fn xy(&self) -> (i32,i32) {
        let xyz = self.xyz();
        (xyz.0,xyz.1)
    }
    pub fn xz(&self) -> (i32,i32) {
        let xyz = self.xyz();
        (xyz.0,xyz.2)
    }
    pub fn x(&self) -> i32 {
        self.xyz().0
    }
    pub fn y(&self) -> i32 {
        self.xyz().1
    }
    pub fn z(&self) -> i32 {
        self.xyz().2
    }
    pub fn index(&self, subject: &Size) -> Option<usize> {
        let xyz = self.xyz();
        let index = xyz.0 + ( xyz.1 * subject.width_i32() );
        if index < 0 {
            None
        }
        else {
            Some(index as usize)
        }
    }
}

impl std::cmp::Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.xy().cmp(&other.xy())
    }
}


impl std::cmp::PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.xy() == other.xy()
    }
}

impl std::cmp::PartialOrd<Size> for Pos {
    fn partial_cmp(&self, other: &Size) -> Option<std::cmp::Ordering> {
        Some(self.xy().cmp(&other.tuple_i32()))
    }
}

impl std::cmp::PartialEq<Size> for Pos {
    fn eq(&self, other: &Size) -> bool {
        self.xy() == other.tuple_i32()
    }
}

impl std::cmp::Eq for Pos {}

impl std::ops::Add for Pos {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        let xyz = self.xyz();
        let xyz1 = other.xyz();
        Self::XYZ(
            xyz.0+xyz1.0,
            xyz.1+xyz1.1,
            xyz.2+xyz1.2
        )
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Pos::XYZ(x,y,z) => {
                x.add_assign(rhs.x());
                y.add_assign(rhs.y());
                z.add_assign(rhs.z());
            },
            Pos::XY(x,y) => {
                x.add_assign(rhs.x());
                y.add_assign(rhs.y());
            },
            Pos::XZ(x,z) => {
                x.add_assign(rhs.x());
                z.add_assign(rhs.z());
            },
            Pos::X(x) => {
                x.add_assign(rhs.x());
            },
            Pos::Y(y) => {
                y.add_assign(rhs.y());
            },
            Pos::Z(z) => {
                z.add_assign(rhs.z());
            },
            Pos::None => {}
        }
    }
}

impl std::ops::Add<Size> for Pos {
    type Output = Self;

    fn add(self, other: Size) -> Self {
        let xyz = self.xyz();
        Self::XYZ(
            xyz.0+other.width_i32(),
            xyz.1+other.height_i32(),
            xyz.2
        )
    }
}