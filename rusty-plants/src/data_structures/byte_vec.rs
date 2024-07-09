use std::alloc::{self, Layout};
use std::mem;
use std::ptr; //Rust analizer says so....
use std::ptr::NonNull;

use std::marker::PhantomData;

use std::ops::Deref;
use std::ops::DerefMut;

//Our slightly smaller vec
#[derive(Debug)]
pub struct ByteVec<T> {
    buf: RawVec<T>,
    //cap: u8, // now held in RawVec...
    len: u8,
}

#[derive(Debug)]
struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

//our internal actual vector I guess...
#[derive(Debug)]
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: u8,
}

#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    // Need to bound the lifetime here, so we do it with `&'a mut Vec<T>`
    // because that's semantically what we contain. We're "just" calling
    // `pop()` and `remove(0)`.
    vec: PhantomData<&'a mut Vec<T>>,
    iter: RawValIter<T>,
}

//to make an interator, notice this isnt an include (use::...)
#[derive(Debug)]
pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> ByteVec<T> {
    //public functions
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "we can't handle ZSTs");
        Self {
            buf: RawVec::new(),
            len: 0,
        }
        /* Pre rawVec code.
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
        */
    }

    pub fn push(&mut self, elem: T) {
        //check if we have space
        if self.len == self.cap() {
            //if not, grow
            self.buf.grow();
        }
        //doms first unsafe rust, witness its glorry.
        unsafe {
            ptr::write(self.ptr().add(self.len.into()), elem);
            //*(ptr + len) = elm
        }
        //increments self
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        //cant pop if it doesnt exist
        if self.len == 0 {
            return None;
        }
        //we decriment first???
        self.len -= 1;
        //yea, because we index at 0.
        unsafe {
            Some(ptr::read(self.ptr().add(self.len.into())))
            // ret = *(ptr + len);
            // does this actually remove the value...
            // or does it remove our ability to see it?
        }
    }

    //private functions
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> u8 {
        self.buf.cap
    }

    pub fn insert(&mut self, index: u8, elem: T) {
        // `<=` because it's valid to insert after everything
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.buf.grow();
        }
        //We only grow if we are exactly at our cap.. odd
        //I think this is if were inserting into an empty thing
        //but I dont think that garentees this can place...
        //sorta a weird double check of if it can insert.
        //Makes it the same behavior as push from empty.
        unsafe {
            // ptr::copy(src, dest, len): "copy from src to dest len elems"
            ptr::copy(
                self.ptr().add(index.into()),
                self.ptr().add((index + 1) as usize), //not sure why this one can 'as'
                (self.len - index).into(),
            );
            ptr::write(self.ptr().add(index.into()), elem);
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
            let result = ptr::read(self.ptr().add(index.into()));
            ptr::copy(
                self.ptr().add((index + 1) as usize),
                self.ptr().add(index.into()),
                (self.len - index).into(),
            );
            result
        }
    }
}

impl<T> Drop for ByteVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}

//acssess functions
//the idea of needing to design this is funny to me.
impl<T> Deref for ByteVec<T> {
    type Target = [T]; //Not totally sure what this is...
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len.into())
            //notice this is no different from mut.
        }
    }
}

impl<T> DerefMut for ByteVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), (self.len).into())
            //notice that sneaky mut in the function name?
        }
    }
}

impl<T> IntoIterator for ByteVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        let (iter, buf) = unsafe { (RawValIter::new(&self), ptr::read(&self.buf)) };

        mem::forget(self);

        IntoIter { iter, _buf: buf }
    }
}

//drain impl, like itterators but it eats data.
impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> ByteVec<T> {
    pub fn drain(&mut self) -> Drain<T> {
        let iter = unsafe { RawValIter::new(&self) };

        // this is a mem::forget safety thing. If Drain is forgotten, we just
        // leak the whole Vec's contents. Also we need to do this *eventually*
        // anyway, so why not do it now?
        self.len = 0;

        Drain {
            iter,
            vec: PhantomData,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> RawValIter<T> {
    // unsafe to construct because it has no associated lifetimes.
    // This is necessary to store a RawValIter in the same struct as
    // its actual allocation. OK since it's a private implementation
    // detail.
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if slice.len() == 0 {
                // if `len = 0`, then this is not actually allocated memory.
                // Need to avoid offsetting because that will give wrong
                // information to LLVM via GEP.
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

//This will probably need the most amount of editing.
impl<T> RawVec<T> {
    fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO: implement ZST support");
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    fn grow(&mut self) {
        // This can't overflow because we ensure self.cap <= isize::MAX.
        //says the original code.. but that is not true for us as of right now.
        let new_cap: u8 = if self.cap == 0 {
            1
        } else {
            2 * self.cap //into maybe not needed?
        };

        // Layout::array checks that the number of bytes is <= usize::MAX,
        // but this is redundant since old_layout.size() <= isize::MAX,
        // so the `unwrap` should never fail.
        let new_layout = Layout::array::<T>(new_cap.into()).unwrap();

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        // we dont want isize::MAX per say.. not sure how to fix that. its too large
        assert!(
            new_layout.size() <= u8::MAX as usize,
            "Allocation too large"
        ); // test if u8::MAX works.. probs wont.

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap.into()).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap.into()).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ByteVec;

    #[test]
    fn create() {
        let mut vector: ByteVec<u32> = ByteVec::new();
        println!("{:?}", vector);

        vector.push(1);
        vector.push(2);
        vector.push(3);
        vector.push(4);
        println!("{:?}", vector);
    }
}
