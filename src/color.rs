#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    RGB(u8,u8,u8),
    RGBA(u8,u8,u8,u8),
    RGBC((u8,u8,u8),(u8,u8,u8)),
    RGBAC((u8,u8,u8,u8),(u8,u8,u8,u8)),
    RGB38(u8,u8,u8),
    RGBA38(u8,u8,u8,u8),
    RGB48(u8,u8,u8),
    RGBA48(u8,u8,u8,u8),
    None
}

#[allow(dead_code)]
impl Color {
    pub fn rgb(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB38(r,g,b) |
            Color::RGB48(r,g,b) => {
                Color::RGB(*r,*g,*b)
            },
            Color::RGBA(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB(*r,*g,*b)
            },
            Color::RGBA38(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB(*r,*g,*b)
            },
            Color::RGBA48(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB(*r,*g,*b)
            },
            Color::RGBC((r1,g1,b1),(r2,g2,b2)) if {
                (r1,g1,b1)==(r2,g2,b2)
            } => {
                Color::RGB(*r1,*g1,*b1)
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (r1,g1,b1,a1)==(r2,g2,b2,a2)&&*a1==255
            } => {
                Color::RGB(*r1,*g1,*b1)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgb38(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB38(r,g,b) |
            Color::RGBC(_,(r,g,b)) => {
                Color::RGB38(*r,*g,*b)
            },
            Color::RGBA(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB38(*r,*g,*b)
            },
            Color::RGBA38(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB38(*r,*g,*b)
            },
            Color::RGBAC(_,(r,g,b,a)) if {
                *a == 255
            } => {
                Color::RGB38(*r,*g,*b)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgb48(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB48(r,g,b) |
            Color::RGBC((r,g,b),_) => {
                Color::RGB48(*r,*g,*b)
            },
            Color::RGBA(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB48(*r,*g,*b)
            },
            Color::RGBA48(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGB48(*r,*g,*b)
            },
            Color::RGBAC(_,(r,g,b,a)) if {
                *a == 255
            } => {
                Color::RGB48(*r,*g,*b)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgbc(&self) -> Color {
        match self {
            Color::RGB(r,g,b) => {
                Color::RGBC((*r,*g,*b),(*r,*g,*b))
            },
            Color::RGBA(r,g,b,a) if {
                *a == 255
            } => {
                Color::RGBC((*r,*g,*b),(*r,*g,*b))
            },
            Color::RGBC((r1,g1,b1),(r2,g2,b2)) => {
                Color::RGBC((*r1,*g1,*b1),(*r2,*g2,*b2))
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (*a1,*a2)==(255,255)
            } => {
                Color::RGBC((*r1,*g1,*b1),(*r2,*g2,*b2))
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgba(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB38(r,g,b) |
            Color::RGB48(r,g,b) => {
                Color::RGBA(*r,*g,*b,255)
            },
            Color::RGBA(r,g,b,a) |
            Color::RGBA38(r,g,b,a) |
            Color::RGBA48(r,g,b,a) => {
                Color::RGBA(*r,*g,*b,*a)
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (r1,g1,b1,a1)==(r2,g2,b2,a2)
            } => {
                Color::RGBA(*r1,*g1,*b1,*a1)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgba38(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB38(r,g,b) |
            Color::RGBC(_,(r,g,b)) => {
                Color::RGBA38(*r,*g,*b,255)
            },
            Color::RGBA(r,g,b,a) |
            Color::RGBA38(r,g,b,a) |
            Color::RGBAC(_,(r,g,b,a)) => {
                Color::RGBA38(*r,*g,*b,*a)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgba48(&self) -> Color {
        match self {
            Color::RGB(r,g,b) |
            Color::RGB48(r,g,b) |
            Color::RGBC((r,g,b),_) => {
                Color::RGBA48(*r,*g,*b,255)
            },
            Color::RGBA(r,g,b,a) |
            Color::RGBA48(r,g,b,a) |
            Color::RGBAC((r,g,b,a),_) => {
                Color::RGBA48(*r,*g,*b,*a)
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn rgbac(&self) -> Color {
        match self {
            Color::RGB(r,g,b) => {
                Color::RGBAC((*r,*g,*b,255),(*r,*g,*b,255))
            },
            Color::RGBA(r,g,b,a) => {
                Color::RGBAC((*r,*g,*b,*a),(*r,*g,*b,*a))
            },
            Color::RGB38(r,g,b) => {
                Color::RGBAC((0,0,0,0),(*r,*g,*b,255))
            },
            Color::RGB48(r,g,b) => {
                Color::RGBAC((*r,*g,*b,255),(0,0,0,0))
            },
            Color::RGBA38(r,g,b,a) => {
                Color::RGBAC((0,0,0,0),(*r,*g,*b,*a))
            },
            Color::RGBA48(r,g,b,a) => {
                Color::RGBAC((*r,*g,*b,*a),(0,0,0,0))
            },
            Color::RGBC((r1,g1,b1),(r2,g2,b2)) => {
                Color::RGBAC((*r1,*g1,*b1,255),(*r2,*g2,*b2,255))
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) => {
                Color::RGBAC((*r1,*g1,*b1,*a1),(*r2,*g2,*b2,*a2))
            },
            _ => {
                Color::None
            }
        }
    }
    pub fn make_rgbc(c1:Color,c2:Color) -> Color {
        if let Color::RGB48(r1,g1,b1) = c1 {
            if let Color::RGB38(r2,g2,b2) = c2 {
                Color::RGBC((r1,g1,b1),(r2,g2,b2))
            }
            else {
                Color::None
            }
        }
        else {
            Color::None
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::RGB(r,g,b) => {
                write!(out, "{}", format!("\x1b[48;2;{r};{g};{b}m\x1b[38;2;{r};{g};{b}m"))
            },
            Color::RGB38(r,g,b) => {
                write!(out, "{}", format!("\x1b[38;2;{r};{g};{b}m"))
            },
            Color::RGB48(r,g,b) => {
                write!(out, "{}", format!("\x1b[48;2;{r};{g};{b}m"))
            },
            Color::RGBC((r1,g1,b1),(r2,g2,b2)) => {
                write!(out, "{}", format!("\x1b[48;2;{r1};{g1};{b1}m\x1b[38;2;{r2};{g2};{b2}m"))
            },
            Color::RGBA(r,g,b,a) if {
                *a == 255
            } => {
                write!(out, "{}", format!("\x1b[48;2;{r};{g};{b}m\x1b[38;2;{r};{g};{b}m"))
            },
            Color::RGBA38(r,g,b,a) if {
                *a == 255
            } => {
                write!(out, "{}", format!("\x1b[38;2;{r};{g};{b}m"))
            },
            Color::RGBA48(r,g,b,a) if {
                *a == 255
            } => {
                write!(out, "{}", format!("\x1b[48;2;{r};{g};{b}m"))
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (*a1,*a2)==(255,255)
            } => {
                write!(out, "{}", format!("\x1b[48;2;{r1};{g1};{b1}m\x1b[38;2;{r2};{g2};{b2}m"))
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (*a1,*a2,*r2,*g2,*b2)==(255,0,0,0,0)
            } => {
                write!(out, "{}", format!("\x1b[48;2;{r1};{g1};{b1}m"))
            },
            Color::RGBAC((r1,g1,b1,a1),(r2,g2,b2,a2)) if {
                (*a1,*r1,*g1,*b1,*a2)==(0,0,0,0,255)
            } => {
                write!(out, "{}", format!("\x1b[38;2;{r2};{g2};{b2}m"))
            },
            Color::None => {
                write!(out, "{}", "\x1b[0m")
            },
            _ => {
                write!(out, "{}", "".to_string())
            }
        }
    }
}