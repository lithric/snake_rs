use crate::ansii_chunk::AnsiiChunk;
use std::alloc::{self, Layout};
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, AddAssign};
use std::ptr::{self, NonNull};

struct RawAnsiiString {
    ptr: NonNull<AnsiiChunk>,
    cap: usize
}

unsafe impl Send for RawAnsiiString {}
unsafe impl Sync for RawAnsiiString {}

impl RawAnsiiString {
    fn new() -> Self {
        let cap = if mem::size_of::<AnsiiChunk>() == 0 { !0 } else { 0 };
        RawAnsiiString {
            ptr: NonNull::dangling(),
            cap
        }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<AnsiiChunk>() != 0, "capacity overflow");
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<AnsiiChunk>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            
            let new_layout = Layout::array::<AnsiiChunk>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<AnsiiChunk>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut AnsiiChunk) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout)
        };
        self.cap = new_cap;
    }
}

impl Drop for RawAnsiiString {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<AnsiiChunk>();
        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<AnsiiChunk>(self.cap).unwrap()
                );
            }
        }
    }
}

pub struct AnsiiString {
    buf: RawAnsiiString,
    len: usize
}

impl AnsiiString {
    fn ptr(&self) -> *mut AnsiiChunk {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        AnsiiString { 
            buf: RawAnsiiString::new(),
            len: 0 
        }
    }

    pub fn push(&mut self, elem: AnsiiChunk) {
        if self.len == self.cap() { 
            self.buf.grow(); 
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn push_ansii_string(&mut self,mut other: AnsiiString) {
        for item in other.drain() {
            self.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<AnsiiChunk> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: AnsiiChunk) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len { 
            self.buf.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> AnsiiChunk {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index
            );
            result
        }
    }

    pub fn drain(&mut self) -> Drain {
        unsafe {
            let iter = RawValIter::new(&self);
            self.len = 0;
            Drain {
                iter,
                ansii_string: PhantomData
            }
        }
    }
}

impl Drop for AnsiiString {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl Deref for AnsiiString {
    type Target = [AnsiiChunk];
    fn deref(&self) -> &[AnsiiChunk] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl DerefMut for AnsiiString {
    fn deref_mut(&mut self) -> &mut [AnsiiChunk] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl IntoIterator for AnsiiString {
    type Item = AnsiiChunk;
    type IntoIter = IntoIter;
    fn into_iter(self) -> IntoIter {
        unsafe {
            let iter = RawValIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);
            IntoIter{_buf:buf,iter}
        }
    }
}

impl Display for AnsiiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len == 0 {
            return write!(f,"");
        }
        let bank: Vec<String> = self.iter().map(|x| x.to_string()).collect();
        let res = bank.join("") + "\x1b[0m";
        write!(f,"{}",res)
    }
}

impl AddAssign for AnsiiString {
    fn add_assign(&mut self, rhs: Self) {
        self.push_ansii_string(rhs);
    }
}

impl AddAssign<AnsiiChunk> for AnsiiString {
    fn add_assign(&mut self, rhs: AnsiiChunk) {
        self.push(rhs);
    }
}


struct RawValIter {
    start: *const AnsiiChunk,
    end: *const AnsiiChunk
}

impl RawValIter {
    unsafe fn new(slice: &[AnsiiChunk]) -> Self {
        RawValIter { 
            start: slice.as_ptr(),
            end: if mem::size_of::<AnsiiChunk>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            }
        }
    }
}

impl Iterator for RawValIter {
    type Item = AnsiiChunk;
    fn next(&mut self) -> Option<AnsiiChunk> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<AnsiiChunk>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(ptr::read(NonNull::<AnsiiChunk>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<AnsiiChunk>();
        let len = (self.end as usize - self.start as usize)
        / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl DoubleEndedIterator for RawValIter {
    fn next_back(&mut self) -> Option<AnsiiChunk> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<AnsiiChunk>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(ptr::read(NonNull::<AnsiiChunk>::dangling().as_ptr()))
                }
                else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

pub struct IntoIter {
    _buf: RawAnsiiString,
    iter: RawValIter
}

impl Iterator for IntoIter {
    type Item = AnsiiChunk;
    fn next(&mut self) -> Option<AnsiiChunk> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<AnsiiChunk> { self.iter.next_back() }
}

impl Drop for IntoIter {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Drain<'a> {
    ansii_string: PhantomData<&'a mut AnsiiString>,
    iter: RawValIter
}

impl<'a> Iterator for Drain<'a> {
    type Item = AnsiiChunk;
    fn next(&mut self) -> Option<AnsiiChunk> { self.iter.next() }
    fn size_hint(&self) -> (usize, Option<usize>) { self.iter.size_hint() }
}

impl<'a> DoubleEndedIterator for Drain<'a> {
    fn next_back(&mut self) -> Option<AnsiiChunk> { self.iter.next_back() }
}

impl<'a> Drop for Drain<'a> {
    fn drop(&mut self) { for _ in &mut *self {} }
}




