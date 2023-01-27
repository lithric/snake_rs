/*
    goal:
        make a console game with console pixels and stuff.
        make a moveable character and all that.
        Possibly make snake.
    
    structure:
        make everything that you could call "different" a different type.
        This helps to reinforce code that is resilliant to error.
        This also makes it easier to implement custom interactions between 
        certaint types of values.
*/

/*
    the most efficient display must have these properties:
        - uses one print statement per frame
        - contains a single ansii end block
        - does not repeat colors on a single line
        - does not repeat colors over successive lines
        
    this is the most efficient display possible.
    What data structure would fit best when writing instructions
    for this display?
    
    first, there is a machine code level for this display.
    There are restrictions for this display.
    first: 
        the display must be rectangular.
    second: 
        the only ansii end must only occur at the end of the display.
    third: 
        every character that is not an ansii instruction is defined to be
        a pixel.
    fourth:
        characters are written in ansii colored chuncks of pixels with no new
        line characters.
    fifth:
        if a character is written to the frame, it cannot be changed afterwards.
    sixth:
        the display must be defined to be write only to the program.
        It cannot be read by any circumstances.
        It must contain it's own method for displaying output to the
        console.
    seventh:
        the display must exist for the entire length of the program,
        it must be gauranteed to exist
    
    I've derived some elements that need to exist for the machine code to work.
    these are absolutely defined:
        - AnsiiChunk
        - AnsiiEnd
    these are abstractly defined:
        - Display
    
    
    needs something that takes in an AnsiiChunk and writes it to DisplayFrame
    and splits up the AnsiiChunk to fit the frame.
    
    Vec<AnsiiChunk>
*/




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
    fn rgb(&self) -> Color {
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
    fn rgb38(&self) -> Color {
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
    fn rgb48(&self) -> Color {
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
    fn rgbc(&self) -> Color {
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
    fn rgba(&self) -> Color {
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
    fn rgba38(&self) -> Color {
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
    fn rgba48(&self) -> Color {
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
    fn rgbac(&self) -> Color {
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

#[allow(dead_code)]
enum AnsiiChunk {
    Text(String,Color),
    Void(String,Color),
    Empty
}

impl AnsiiChunk {
    fn len(&self) -> usize {
        match self {
            AnsiiChunk::Text(v,_) => {
                v.len()
            },
            AnsiiChunk::Void(v,_) => {
                v.len()
            },
            AnsiiChunk::Empty => {
                0usize
            }
        }
    }
    fn color(&self) -> Color {
        match self {
            AnsiiChunk::Text(_,c) => {
                *c
            },
            AnsiiChunk::Void(_,c) => {
                *c
            },
            AnsiiChunk::Empty => {
                Color::None
            }
        }
    }
    fn text(&self) -> Result<&String,()> {
        match self {
            AnsiiChunk::Text(v,_) => {
                Ok(&v)
            },
            AnsiiChunk::Void(v,_) => {
                Ok(&v)
            },
            AnsiiChunk::Empty => {
                Err(())
            }
        }
    }
    fn split_off(&mut self,index: usize) -> Self {
        if index >= self.len() {
            return AnsiiChunk::Empty;
        }
        match self {
            AnsiiChunk::Text(v,c) => {
                AnsiiChunk::Text((*v).split_off(index),*c)
            },
            AnsiiChunk::Void(v,c) => {
                AnsiiChunk::Void((*v).split_off(index),*c)
            },
            AnsiiChunk::Empty => {
                AnsiiChunk::Empty
            }
        }
    }
}

impl std::fmt::Display for AnsiiChunk {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = self.color();
        let binding = "".to_string();
        let text = match self.text() {
            Ok(s) => s,
            Err(()) => &binding
        };
        write!(out, "{}", format!("{c}{text}"))
    }
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
enum Size {
    WH(usize,usize),
    None
}

impl Size {
    fn area(&self) -> usize {
        match self {
            Size::WH(w,h) => {
                w*h
            },
            Size::None => {
                0usize
            }
        }
    }
    fn width(&self) -> usize {
        match self {
            Size::WH(w,_) => {
                *w
            },
            Size::None => {
                0usize
            }
        }
    }
    fn width_i32(&self) -> i32 {
        self.width() as i32
    }
    fn height(&self) -> usize {
        match self {
            Size::WH(_,h) => {
                *h
            },
            Size::None => {
                0usize
            }
        }
    }
    fn height_i32(&self) -> i32 {
        self.height() as i32
    }
    fn tuple(&self) -> (usize,usize) {
        match self {
            Size::WH(w,h) => {
                (*w,*h)
            },
            Size::None => {
                (0usize,0usize)
            }
        }
    }
    fn tuple_i32(&self) -> (i32,i32) {
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


#[derive(Copy, Clone)]
#[allow(dead_code)]
enum Pos {
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
    fn xyz(&self) -> (i32,i32,i32) {
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
    fn xy(&self) -> (i32,i32) {
        let xyz = self.xyz();
        (xyz.0,xyz.1)
    }
    fn xz(&self) -> (i32,i32) {
        let xyz = self.xyz();
        (xyz.0,xyz.2)
    }
    fn x(&self) -> i32 {
        self.xyz().0
    }
    fn y(&self) -> i32 {
        self.xyz().1
    }
    fn z(&self) -> i32 {
        self.xyz().2
    }
    fn index(&self, subject: &Size) -> Option<usize> {
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

// impl std::cmp::Ord<Size> for XYZ {
//     fn cmp(&self, other: &Size) -> std::cmp::Ordering {
//         (self.x, self.y).cmp((other.width as i32, other.height as i32))
//     }
// }

struct RenderQuery {
    query: Vec<RenderInstruction>
}

impl RenderQuery {
    fn get_index(&self,index: usize) -> Option<&RenderInstruction> {
        return self.query.iter().find(|instruct| instruct.start == index);
    }
    fn insert(&mut self, new_instruct: RenderInstruction) {
        let test_index = self.query.iter().position(|instruct| instruct.end < new_instruct.start+1);
        if test_index.is_none() {
            self.query.push(new_instruct);
            return;
        }
        let index = test_index.unwrap();
        self.query.insert(index,new_instruct);
    }
    fn add(&mut self, mut new_instruct: RenderInstruction) {
        let mut i = 0;
        while i < self.query.len() {
            let instruct = &mut self.query[i];
            if new_instruct.z >= instruct.z {
                let test_extra_instruct = instruct.split_gap(new_instruct.start,new_instruct.end);
                if instruct.start >= instruct.end {
                    self.query.remove(i);
                    continue;
                }
                if test_extra_instruct.is_none() {
                    i += 1;
                    continue;
                }
                let extra_instruct = test_extra_instruct.unwrap();
                self.insert(extra_instruct);
            }
            else {
                new_instruct.split_gap(instruct.start,instruct.end);
                if new_instruct.start >= new_instruct.end {
                    return;
                }
            }
            i += 1;
        }
        self.insert(new_instruct);
        // (0,100,red) -> (20,27,green) (30,40,cyan) (50,100,blue)
        // (0,19,red) (20,27,green) (28,29,red) 
    }
}

#[derive(Debug)]
struct RenderInstruction {
    start: usize,
    end: usize,
    z: i32,
    color: Color
}

impl RenderInstruction {
    fn split_gap(&mut self, start: usize, end: usize) -> Option<RenderInstruction> {
        if start > end {
            return None;
        }
        if start > self.end {
            return None;
        }
        if end < self.start {
            return None;
        }
        let mut first_index = start;
        let mut last_index = end;
        if first_index < self.start {
            first_index = self.start;
        }
        if last_index > self.end {
            last_index = self.end;
        }
        if first_index == self.start && last_index == self.end {
            self.start = 0;
            self.end = 0;
            return None;
        }
        if first_index == self.start {
            self.start = last_index;
            return None;
        }
        if last_index == self.end {
            self.end = first_index;
            return None;
        }
        // all none cases returned
        let self_end = self.end;
        self.end = first_index;
        return Some(RenderInstruction{
            start: last_index,
            end: self_end,
            z: self.z,
            color: self.color
        });
    }
}

impl std::fmt::Display for RenderInstruction {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        let start = self.start;
        let end = self.end;
        write!(out, "{}", format!("start:{start},end:{end}"))
    }
}

struct DisplayObject {
    size: Size,
    pos: Pos,
    color: Color
    //children: Vec<DisplayObject>,
    //parent: Option<Box<DisplayObject>>,
}

#[allow(dead_code)]
impl DisplayObject {
    fn render(&self) -> String {
        let mut render = String::new();
        render += self.color.to_string().as_str();
        for _ in 0..self.size.height() {
            render += &"  ".repeat(self.size.width());
            render += "\n";
        }
        render.pop();
        render += (Color::None).to_string().as_str();
        return render;
    }
    const fn create(
        size: Size,
        pos: Pos,
        color: Color,
    ) -> DisplayObject {
        return DisplayObject {
            size,
            pos,
            color
            //children:vec![],
            //parent:None
        }
    }
    /*fn starting_index(&self, subject: &Size) -> Option<usize> {
        self.xyz.index(subject)
    }
    fn ending_index(&self, subject: &Size) -> Option<usize> {
        ( self.xyz + self.size ).index(subject)
    }*/
}

struct DisplayFrame {
    size: Size,
    color: Color,
    body: Vec<DisplayObject>
}

impl DisplayFrame {
    fn render(&self) -> String {
        // vec[(0,9,red),(10,19,blue),(20,29,red)]
        let mut r_query = RenderQuery{query:vec![]};
        r_query.add(RenderInstruction{
            start: 0,
            end: self.size.area(),
            z: -1,
            color: self.color
        });
        for object in &self.body {
            // if contained
            let is_within_bounds = if object.pos + object.size > self.size {
                false
            }
            else {
                true
            };
            if is_within_bounds {
                for i in 0..object.size.height() {
                    let coords = object.pos + Pos::Y(i as i32);
                    let start = coords.index(&self.size).unwrap();
                    let end = start + object.size.width();
                    let color = object.color;
                    let instruct = RenderInstruction {
                        start: start,
                        end: end,
                        z: object.pos.z(),
                        color: color
                    };
                    r_query.add(instruct);
                }
            }
        }
        let mut render = String::new();
        render += self.color.to_string().as_str();
        for i in 0..self.size.height() {
            for k in 0..self.size.width() {
                let index = i*self.size.width() + k;
                match r_query.get_index(index) {
                    Some(instruct) => {
                        render += instruct.color.to_string().as_str();
                        render += "  ";
                    }
                    None => {
                        render += "  ";
                    }
                }
            }
            render += "\n";
        }
        render.pop();
        render += (Color::None).to_string().as_str();
        return render;
    }
    const fn create(
        size: Size,
        color: Color
    ) -> DisplayFrame {
        return DisplayFrame {
            size,
            color,
            body: vec![]
        }
    }
    fn add(&mut self,object: DisplayObject) {
        self.body.push(object);
    }
}

const RED: Color = Color::RGB(255,0,0);
const OBJECT_COLOR: Color = Color::RGB(0,255,255);
const BACKGROUND_COLOR: Color = Color::RGB(50,50,50);

const FRAME_SIZE: Size = Size::WH(20,10);

fn main() {
    let mut frame = DisplayFrame::create(
        FRAME_SIZE,
        BACKGROUND_COLOR
    );
    let object = DisplayObject::create(
        Size::WH(5,5),
        Pos::XYZ(2,2,0),
        OBJECT_COLOR
    );
    frame.add(object);
    println!("{}",frame.render());
    let mut col = AnsiiChunk::Void("  ".repeat(14),RED);
    println!("{}",col.split_off(14));
    println!("{}",col);
    print!("{}",Color::None);
}
