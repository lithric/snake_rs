use crate::Color;
#[allow(dead_code)]
pub enum AnsiiChunk {
    Text(String,Color),
    Void(String,Color),
    Empty
}

#[allow(dead_code)]
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