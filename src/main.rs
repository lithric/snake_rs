/******************************************************************************

                            Online Rust Compiler.
                Code, Compile, Run and Debug Rust program online.
Write your code in this editor and press "Run" button to execute it.

*******************************************************************************/


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

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::cmp::{min,max};

/*
    origin: XY
    [**color**][**pixels**]
*/

struct AnsiiColorChunk(usize,RGB);


enum AnsiiChunk {
    Text(String,RGB,RGB),
    Void(usize,RGB)
}

impl AnsiiChunk {
    fn len(&self) -> usize {
        match self {
            AnsiiChunk::Text => {
                self
            }
        }
    }
    fn rgb(&self) -> RGB {}
}

struct RenderMachineCode(String);

impl std::fmt::Display for RenderMachineCode {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        // code: [(width,height),(color,length),(color,length),(end,length),(color,length)]
        write!(out, "{}", format!("\x1b[48;2;{red};{green};{blue}m"))
    }
}


#[derive(Debug, Copy, Clone)]
struct Size {
    width: usize,
    height: usize
}

impl std::cmp::Ord for Size {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.width * self.height).cmp(&(other.width * other.height))
    }
}

impl std::cmp::PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.width * self.height == other.width * other.height
    }
}

impl std::cmp::PartialOrd<XYZ> for Size {
    fn partial_cmp(&self, other: &XYZ) -> Option<std::cmp::Ordering> {
        Some((self.width as i32, self.height as i32).cmp(&(other.x, other.y)))
    }
}

impl std::cmp::PartialEq<XYZ> for Size {
    fn eq(&self, other: &XYZ) -> bool {
        ( self.width as i32, self.height as i32 ) == ( other.x, other.y )
    }
}

impl std::cmp::Eq for Size {}

#[derive(Debug, Copy, Clone)]
struct XY {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone)]
struct XYZ {
    x: i32,
    y: i32,
    z: i32
}

impl XYZ {
    fn index(&self, subject: &Size) -> Option<usize> {
        let index = self.x + ( self.y * (subject.width as i32) );
        if index < 0 {
            None
        }
        else {
            Some(index as usize)
        }
    }
}

impl std::cmp::Ord for XYZ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl std::cmp::PartialOrd for XYZ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for XYZ {
    fn eq(&self, other: &Self) -> bool {
        (self.x, self.y) == (other.x , other.y)
    }
}

impl std::cmp::PartialOrd<Size> for XYZ {
    fn partial_cmp(&self, other: &Size) -> Option<std::cmp::Ordering> {
        Some((self.x, self.y).cmp(&(other.width as i32, other.height as i32)))
    }
}

impl std::cmp::PartialEq<Size> for XYZ {
    fn eq(&self, other: &Size) -> bool {
        (self.x, self.y) == (other.width as i32, other.height as i32)
    }
}

impl std::cmp::Eq for XYZ {}

impl std::ops::Add for XYZ {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Add<Size> for XYZ {
    type Output = Self;

    fn add(self, other: Size) -> Self {
        Self {
            x: self.x + (other.width as i32),
            y: self.y + (other.height as i32),
            z: self.z
        }
    }
}

// impl std::cmp::Ord<Size> for XYZ {
//     fn cmp(&self, other: &Size) -> std::cmp::Ordering {
//         (self.x, self.y).cmp((other.width as i32, other.height as i32))
//     }
// }

#[derive(Debug, Copy, Clone)]
struct RGB {
    r: u8,
    g: u8,
    b: u8
}

impl std::fmt::Display for RGB {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        let red = self.r;
        let green = self.g;
        let blue = self.b;
        write!(out, "{}", format!("\x1b[48;2;{red};{green};{blue}m"))
    }
}

#[derive(Debug, Copy, Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl std::fmt::Display for RGBA {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        let red = self.r;
        let green = self.g;
        let blue = self.b;
        let alpha = self.a;
        if red == 0 && green == 0 && blue == 0 && alpha == 0 {
            write!(out, "{}","\x1b[0m".to_string())
        }
        else {
            write!(out, "{}", format!("\x1b[48;2;{red};{green};{blue}m"))
        }
    }
}

struct GameColor {}

impl GameColor {
    const fn _none() -> RGBA {RGBA{r:0,g:0,b:0,a:0}}
    const fn object() -> RGBA {RGBA{r:0,g:255,b:255,a:255}}
    const fn background() -> RGB {RGB{r:50,g:50,b:50}}
    const fn _red() -> RGB {RGB{r:255,g:0,b:0}}
    const fn _start() -> &'static str {"\x1b[48;2;"}
    const fn end() -> &'static str {"\x1b[0m"}
}

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
                self.query.insert(i+1,extra_instruct);
                i += 1;
            }
            else {
                let test_extra_instruct = new_instruct.split_gap(instruct.start,instruct.end);
                if new_instruct.start >= new_instruct.end {
                    return;
                }
                if test_extra_instruct.is_none() {
                    i += 1;
                    continue;
                }
                let extra_instruct = test_extra_instruct.unwrap();
                self.query.insert(i+1, extra_instruct);
                i += 1;
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
    color: RGBA
}

impl RenderInstruction {
    fn split_gap(&mut self, mut start: usize, mut end: usize) -> Option<RenderInstruction> {
        // extremely fast
        if start > end {return None};
        if start > self.end {return None};
        if end < self.start {return  None};
        if start < self.start {start = self.start};
        if end > self.end {end = self.end};
        if (start,end) == (self.start,self.end) {
            self.start=0;
            self.end=0;
            return None;
        }
        if start == self.start {self.start=end;return None};
        if end == self.end {self.end=start;return None};
        // all none cases returned
        let self_end = self.end;
        self.end = start;
        return Some(RenderInstruction{
            start: end,
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
    xyz: XYZ,
    color: RGBA,
    _children: Vec<DisplayObject>
}

impl DisplayObject {
    fn _render(&self) -> String {
        let mut render = String::new();
        render += self.color.to_string().as_str();
        for _ in 0..self.size.height {
            render += &"  ".repeat(self.size.width);
            render += "\n";
        }
        render.pop();
        render += GameColor::end();
        return render;
    }
    const fn create(
        size: Option<Size>,
        xyz: Option<XYZ>,
        color: Option<RGBA>,
    ) -> DisplayObject {
        return DisplayObject {
            size:match size {Some(s)=>s,None=>Size{width:0,height:0}},
            xyz:match xyz {Some(v)=>v,None=>XYZ{x:0,y:0,z:0}},
            color:match color {Some(c)=>c,None=>GameColor::object()},
            _children:vec![]
        }
    }
}

struct DisplayFrame {
    size: Size,
    color: RGB,
    render_query: RenderQuery,
    body: Vec<DisplayObject>
}

impl DisplayFrame {
    fn render(&self) -> String {
        // add the frame to render query
        let mut r_query = RenderQuery{query:vec![
            RenderInstruction{
                start: 0,
                end: self.size.width * self.size.height,
                z: -1,
                color: RGBA{r:self.color.r,g:self.color.g,b:self.color.b,a:255}
            }
        ]};
        for object in &self.body {
            // if contained
            let off_left = -min(object.xyz.x,0) as usize;
            let off_right = -min((self.size.width as i32)-object.xyz.x-(object.size.width as i32),0) as usize;
            let off_top = -min(object.xyz.y,0) as usize;
            let off_bottom = -min((self.size.height as i32)-object.xyz.y-(object.size.height as i32),0) as usize;
            let off_screen_x = max(off_left,off_right);
            let off_screen_y = max(off_top,off_bottom);
            let avail_width = if off_screen_x < object.size.width {
                object.size.width - off_screen_x
            } else {
                0
            };
            let avail_height = if off_screen_y < object.size.height {
                object.size.height - off_screen_y
            } else {
                0
            };
            let moved_x = object.xyz.x + (off_left as i32);
            let moved_y = object.xyz.y + (off_top as i32);
            let xyz = XYZ{x:moved_x,y:moved_y,z:object.xyz.z};
            if avail_width*avail_height == 0 {continue};
            for i in 0..avail_height {
                let coords = xyz + XYZ{x:0,y:(i as i32),z:0};
                let start = coords.index(&self.size).unwrap();
                let end = start + avail_width;
                let z = xyz.z;
                let color = object.color;
                let instruct = RenderInstruction{start,end,z,color};
                r_query.add(instruct);
            }
        }
        let mut render = String::new();
        render += self.color.to_string().as_str();
        for i in 0..self.size.height {
            for k in 0..self.size.width {
                let index = i*self.size.width + k;
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
        render += GameColor::end();
        return render;
    }
    fn create(
        size: Option<Size>,
        color: Option<RGB>
    ) -> DisplayFrame {
        let size_ans = match size {Some(s)=>s,None=>Size{width:50,height:50}};
        let color_ans = match color {Some(c)=>c,None=>GameColor::background()};
        return DisplayFrame {
            size: size_ans,
            color: color_ans,
            render_query: RenderQuery { query: vec![] },
            body: vec![]
        }
    }
    fn add(&mut self,object: DisplayObject) {
        self.body.push(object);
    }
}

fn main() {
    let mut main_frame = DisplayFrame::create(
        Some(Size{width:20,height:10}),
        Some(GameColor::background())
    );
    let object = DisplayObject::create(
        Some(Size{width:5,height:5}),
        Some(XYZ{x:0,y:0,z:0}),
        Some(GameColor::object())
    );
    main_frame.add(object);
    let time = std::time::Duration::from_millis(10);
    let mut max_time = 100*10;
    let device_state = DeviceState::new();
    while max_time > 0 {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::A) {
            main_frame.body[0].xyz.x -= 1;
        }
        if keys.contains(&Keycode::S) {
            main_frame.body[0].xyz.y += 1;
        }
        if keys.contains(&Keycode::W) {
            main_frame.body[0].xyz.y -= 1;
        }
        if keys.contains(&Keycode::D) {
            main_frame.body[0].xyz.x += 1;
        }
        render_frame(&main_frame);
        std::thread::sleep(time);
        max_time -= 1;
    }
}

fn render_frame(frame: &DisplayFrame) {
    let render = RenderMachineCode(frame.render());
    print!("\x1B[2J\x1B[1;1H");
    println!("{:?}",&render);
}

/*
    
*/