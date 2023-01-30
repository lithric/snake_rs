

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
#![allow(dead_code,unused)]
pub mod color;
pub mod ansii_chunk;
pub mod size;
pub mod pos;
pub mod ansii_string;

use crate::color::Color;
use crate::ansii_chunk::AnsiiChunk;
use crate::ansii_string::AnsiiString;
use crate::size::Size;
use crate::pos::Pos;
use device_query::{DeviceQuery, DeviceState, Keycode};
use my_proc_macros_lib::create_object;
use std::cmp::{min,max};

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
    color: Color
}

impl RenderInstruction {
    fn split_gap(&mut self, mut start: usize, mut end: usize) -> Option<RenderInstruction> {
        // (0..10),(5..12)
        // (0,12)
        // (10-0)+(12-5)-12
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
    pos: Pos,
    color: Color,
    _children: Vec<DisplayObject>
}

impl DisplayObject {
    fn _render(&self) -> String {
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
            color,
            _children:vec![]
        }
    }
}

struct DisplayFrame {
    size: Size,
    color: Color,
    body: Vec<DisplayObject>
}

impl DisplayFrame {
    fn render(&self) -> String {
        return String::new();
        /*
        // add the frame to render query
        let mut r_query = RenderQuery{query:vec![
            RenderInstruction{
                start: 0,
                end: self.size.area(),
                z: -1,
                color: self.color
            }
        ]};
        for object in &self.body {
            // if contained
            let off_left = min(object.pos.x(),0) as usize;
            let off_right = -min((self.size.width() as i32)-object.pos.x()-(object.size.width() as i32),0) as usize;
            let off_top = min(object.pos.y(),0) as usize;
            let off_bottom = -min((self.size.height() as i32)-object.pos.y()-(object.size.height() as i32),0) as usize;
            let off_screen_x = max(off_left,off_right);
            let off_screen_y = max(off_top,off_bottom);
            let avail_width = if off_screen_x < object.size.width() {
                object.size.width() - off_screen_x
            } else {
                0
            };
            let avail_height = if off_screen_y < object.size.height() {
                object.size.height() - off_screen_y
            } else {
                0
            };
            let moved_x = object.pos.x() + (off_left as i32);
            let moved_y = object.pos.y() + (off_top as i32);
            let xyz = Pos::XYZ(moved_x,moved_y,object.pos.z());
            if avail_width*avail_height == 0 {continue};
            for i in 0..avail_height {
                let coords = xyz + Pos::Y(i as i32);
                let start = coords.index(&self.size).unwrap();
                let end = start + avail_width;
                let z = xyz.z();
                let color = object.color;
                let instruct = RenderInstruction{start,end,z,color};
                r_query.add(instruct);
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
        */
    }
    fn create(
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
const BLUE: Color = Color::RGB(0,0,255);
const GREEN: Color = Color::RGB(0, 255, 0);
const CYAN: Color = Color::RGB(0,255,255);

const BACKGROUND_COLOR: Color = Color::RGB(50,50,50);
const OBJECT_COLOR: Color = Color::RGB(0,255,255);

fn main() {
    let mut main_frame = DisplayFrame::create(
        Size::WH(20,10),
        BACKGROUND_COLOR
    );
    let object = DisplayObject::create(
        Size::WH(5,5),
        Pos::XYZ(0,0,0),
        OBJECT_COLOR
    );
    /* 
        create_object! {
            * "  x  x  " GREEN "        "      *
            * "  xxxx  " RED   "        "      *
            * "xxxxxxxx" BLUE  " hello! " CYAN *
            * "xxxxxxxx" BLUE  "        "      *
            * "  x  x  " BLUE  "        "      *
        }
        let x = AnsiiString::new();
        x.push(AnsiiChunk::Void)
    */
    main_frame.add(object);
    let test = create_object! {
        * "  x  x  " GREEN "        "     *
        * "  xxxx  " RED   "        "     *
        * "xxxxxxxx" BLUE  "        "     *
        * "xxxxxxxx" BLUE  " hello! " RED *
        * "xxxxxxxx" BLUE  "        "     *
        * "  x  x  " RED   "        "     *
    };
    println!("{test}");
    /* 
    let time = std::time::Duration::from_millis(10);
    let mut max_time = 100*10;
    let device_state = DeviceState::new();
    while max_time > 0 {
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::A) {
            main_frame.body[0].pos += Pos::X(-1);
        }
        if keys.contains(&Keycode::S) {
            main_frame.body[0].pos += Pos::Y(1);
        }
        if keys.contains(&Keycode::W) {
            main_frame.body[0].pos += Pos::Y(-1);
        }
        if keys.contains(&Keycode::D) {
            main_frame.body[0].pos += Pos::X(1);
        }
        render_frame(&main_frame);
        std::thread::sleep(time);
        max_time -= 1;
    }
    */
}

fn render_frame(frame: &DisplayFrame) {
    print!("\x1B[2J\x1B[1;1H");
    println!("{}",frame.render());
}

/*
    
*/