use crate::Color;
#[allow(dead_code)]
pub enum AnsiiChunk {
    Text(String,Color),
    Void(String,Color),
    Empty(String,Color)
}

#[allow(dead_code)]
impl AnsiiChunk {
    fn len(&self) -> usize {
        match self {
            AnsiiChunk::Text(v,_) |
            AnsiiChunk::Void(v,_) => {
                v.len()
            },
            AnsiiChunk::Empty(_,_) => {
                0usize
            }
        }
    }
    fn color(&self) -> Color {
        match self {
            AnsiiChunk::Text(_,c) |
            AnsiiChunk::Void(_,c) |
            AnsiiChunk::Empty(_,c) => {
                *c
            }
        }
    }
    fn text(&self) -> &String {
        match self {
            AnsiiChunk::Text(v,_) |
            AnsiiChunk::Void(v,_) |
            AnsiiChunk::Empty(v,_) => {
                v
            }
        }
    }
    fn split_off(&mut self,mut index: usize) -> Self {
        if self.len() == 0 {
            return AnsiiChunk::Empty(String::new(),self.color());
        }
        if index >= self.len() {
            index = self.len()-1;
        }
        let remain = match self {
            AnsiiChunk::Text(v,c) => {
                AnsiiChunk::Text(v.split_off(index),*c)
            },
            AnsiiChunk::Void(v,c) => {
                AnsiiChunk::Void(v.split_off(index),*c)
            },
            AnsiiChunk::Empty(_,c) => {
                AnsiiChunk::Empty(String::new(),*c)
            }
        };
        if self.len() == 0 {
            *self = AnsiiChunk::Empty(String::new(),self.color());
        };
        remain
    }
}

impl std::fmt::Display for AnsiiChunk {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AnsiiChunk::Text(v,c) |
            AnsiiChunk::Void(v,c) => {
                write!(out, "{}", format!("{c}{v}"))
            },
            AnsiiChunk::Empty(_,_) => {
                write!(out,"")
            }
        }
    }
}