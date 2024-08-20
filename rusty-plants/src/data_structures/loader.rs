//loadContainer is a container that is used in place of a vector to allow reuse of alocated memory. using a loader
use num::Integer;

pub trait Loader<ID, DATA> {
    fn load(&mut self, _:ID) -> DATA;
    fn unload(&mut self, _:DATA);
}

//#[derive(Debug)]
struct LoadContainer<ID: Integer + Default, DATA>
{
    raw: Vec<DATA>,                    //data container
    index: Vec<ID>,                    //stores index values. hash map?
    loader: Box<dyn Loader<ID, DATA>>, //The encapsulation of load and unload.
    idcount: ID                        //running count of id values
}

impl<ID: Integer + Default, DATA> LoadContainer<ID, DATA> {
    pub fn new(loader: impl Loader<ID, DATA> + 'static) -> LoadContainer<ID, DATA> 
    {
        Self {
            raw: Vec::<DATA>::new(),    
            index: Vec::<ID>::new(),
            loader: Box::new(loader),
            idcount: Default::default() // 0 is the result.
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct TestLoader {
        map: HashMap<u32,i32>,
        idcount: u32
    }

    impl TestLoader {
        pub fn new() -> TestLoader {
            Self {
                map: HashMap::new(),
                idcount: 0
            }
        }
    }

    impl Loader<u32, i32> for TestLoader{
        fn load(&mut self, id: u32) -> i32 {
            unimplemented!()
        }

        fn unload(&mut self, Data: i32) {

        }
    }

    #[test]
    fn init() {

        //let loader = LoadContainer::new(load_test_closer, unload_test_closer);

    }

}
