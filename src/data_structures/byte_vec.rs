use std::mem;
use std::ptr::NonNull; 
use std::alloc::{self, Layout};

use std::ops::Deref;
use std::ops::DerefMut;

//Our slightly smaller vec
pub struct ByteVec<T> {
    ptr: NonNull<T>,
    cap: u8,
    len: u8,
}

//to make an interator, notice this isnt an include (use::...)
pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: u8,
    start: *const T,
    end:   *const T,
}

impl<T> ByteVec<T> {

    //public functions
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "we can't handle ZSTs");
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, elem: T){
        //check if we have space
        if self.len == self.cap {
            //if not, grow
            self.gorw();
        }
        //doms first unsafe rust, witness its glorry.
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
            //*(ptr + len) = elm
        }
        //increments self
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        //cant pop if it doesnt exist
        if self.len == 0 {
            None
        }
        //we decriment first???
        self.len -= 1;
        //yea, because we index at 0.
        unsafe{
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            // ret = *(ptr + len); 
            // does this actually remove the value... 
            // or does it remove our ability to see it?
        }
    }

    pub fn insert(&mut self, index: u8, elem: T) {
        // `<=` because it's valid to insert after everything
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap { self.grow(); }
        //We only grow if we are exactly at our cap.. odd
        //I think this is if were inserting into an empty thing
        //but I dont think that garentees this can place... 
        //sorta a weird double check of if it can insert. 
        //Makes it the same behavior as push from empty.
        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            // *(ptr + index) = elm;
        }
        self.len += 1;
    }

    //this isnt a function I would expect to have to be honest.
    //removes an index and then copys everything back. 
    pub fn remove(&mut self, index: u8) -> T {
        // `<` because it's *not* valid to remove after everything
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    //private functions
    //allocates!
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            //layout seem to describe exactly what a block of memory looks like.
            let new_layout = Layout::array::<T>(new_cap as usize).unwrap();
            (new_cap, new_layout)
        };

        //isize is actually way larger then our tiny vec, should fix.
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap as usize).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
            // realoc()
        };
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}
//dealocate
impl<T> Drop for ByteVec<T> {
    fn drop(&mut self){
        //if cap is zero theres nothing to dealocate
        if self.cap != 0 {
            //removes all elements until pop returns None.
            //I think this is effectivly us promising it cannot be acssessed. 
            while let Some(_) = self.pop() {/*do nothing with it?*/} 
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
                //notice this spessifies how many bytes were freeing
                //in C, this is handled with meta data behind the memory address.
                //is as *mut u8 correct?
            }
        }
    }
}

//acssess functions
//the idea of needing to design this is funny to me.
impl<T> Deref for ByteVec<T> {
    type Target = [T]; //Not totally sure what this is...
    fn deref(&self) -> &[T]{
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
            //notice this is no different from mut. 
        }
    }
}

impl<T> DerefMut for ByteVec<T> {
    fn deref_mut(&mut self) -> &mut [T]{
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }    
}

//Iterators
//Allocates!
impl<T> IntoIterator for ByteVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let vec = ManuallyDrop::new(self);
        //this is saying were going to handle this..
        
        // Can't destructure Vec since it's Drop
        let ptr = vec.ptr;
        let cap = vec.cap;
        let len = vec.len;

        //actual creation.
        IntoIter {
            buf: ptr,
            cap,
            start: ptr.as_ptr(),
            end: if cap == 0 {
                // can't offset off this pointer, it's not allocated!
                ptr.as_ptr()
            } else {
                //if it does exist, point to it.
                unsafe { ptr.as_ptr().add(len) }
            },
        }
    }
}

//Dealocate!
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // drop any remaining elements
            // again because it us promising we cant touch it after.
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}

//forward
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }
    //Not sure if these need to be changed yet... not even sure what it is..
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize)
                  / mem::size_of::<T>();
        (len, Some(len))
    }
}

//backwards
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

