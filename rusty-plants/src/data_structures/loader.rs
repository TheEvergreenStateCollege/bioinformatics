//loadContainer is a container that is used in place of a vector to allow reuse of alocated memory. using a loader
use num::Integer;
use std::fmt;

pub trait Store<ID, DATA> {
    fn load(&mut self, _:ID) -> DATA;
    fn unload(&mut self, _:DATA);
}

//#[derive(Debug)]
struct Loader<ID: Integer + Default, DATA>
{
    raw: Vec<DATA>,                    //data container
    index: Vec<ID>,                    //stores index values. hash map?
    loader: Box<dyn Store<ID, DATA>>, //The encapsulation of load and unload.
    idcount: ID                        //running count of id values
}

/* Functions used in suffixtrees for loadContainer to replace vec 
 *
 *   explicitly used:
 * 
 *     * .push       adds thing. increments id
 *     * []          checks if thing, returns if found. grabs from store if not
 *     * .len()      returns idcount (I think)
 *     * .iter_mut()
 *     * .itter()
 *
 */

impl<ID: Integer + Default, DATA> Loader<ID, DATA> {
    pub fn new(loader: impl Store<ID, DATA> + 'static) -> Loader<ID, DATA> 
    {
        Self {
            raw: Vec::<DATA>::new(),    
            index: Vec::<ID>::new(),
            loader: Box::new(loader),
            idcount: Default::default() // 0 is the result.
        }
    }

}

//manual impl of trait Debug sense the debug macro can't create something for Box, so we print the pointer.
impl<ID: Integer + Default + fmt::Debug, DATA: fmt::Debug> fmt::Debug for Loader<ID, DATA> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Loader")
            .field("raw", &self.raw)
            .field("index", &self.index)
            .field("idcount", &self.idcount)
            // Print the pointer address of the Box, probably will remove later.
            .field("loader", &format_args!("{:p}", &*self.loader)) 
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct TestStore {
        map: HashMap<u32,i32>,
    }

    //test loader trait implimentor
    impl TestStore {
        pub fn new() -> TestStore {
            Self {
                map: HashMap::new()
            }
        }
    }

    impl Store<u32, i32> for TestStore{
        fn load(&mut self, id: u32) -> i32 {
            unimplemented!()
        }

        fn unload(&mut self, Data: i32) {
            unimplemented!()
        }
    }

    #[test]
    fn init() {
        let loader = Loader::new(TestStore::new());
        println!("{:?}", loader);

    }

}
