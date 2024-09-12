/*!****************************************************************************\

  \file   loader.rs
  \author Dominic Severe
  \date   9/5/2024

  \par            email:  dominicsevere\@gmail.com
  \par          project:  smarty-plants

  \brief
      A container wraper used in place of a vector to allow reuse of allocated memory
  
    Responsibility's:
        Structs:
            Loader
        Traits: 
            Store
            behavior
  
    Notes:
       
\******************************************************************************/
/**********************************************************\
 *    mods 
\**********************************************************/
use num::Integer;
use std::fmt;
use std::collections::VecDeque;
use std::marker::PhantomData; //Used to allow for trackers that care not for the data itself
//use std::time;

/**********************************************************\
 *    traits 
\**********************************************************/
/// Stores Data somewhere else. (like a data base)
pub trait Store<ID, DATA> {
    fn load(&mut self, _:ID) -> DATA; //needs to be changed to multiple
    fn unload(&mut self, _:DATA);
}

/// Tracks what to get rid of.
pub trait Tracker<ID, DATA> {
    fn update(&mut self, id: ID);        //Updates the last check of a node.
    fn least_used(&self) -> Vec<ID>; //Tells Loader what to get rid of. 

    //It might be fun to do a binary analysis to see if this effects run time.
}
//Define what data must be restricted by? probably need to knowing rust...
//pub trait IndexedChildren ?...

/*********************************************************************\
 *
 *        Structs:
 *
\*********************************************************************/
struct Loader<ID: Integer + Default, DATA>
{
    raw: Vec<DATA>,                      //Data containing the raw data.
    lookup:  Vec<ID>,                    //
    store: Box<dyn Store<ID, DATA>>,     //The encapsulation of load and unload.
    tracker: Box<dyn Tracker<ID, DATA>>, //Tracks what to get rid of.
    idcount: ID,                         //Running count of id values used.
    mem_limit: Option<ID>                //Limit of memory consumption. set or found.

    //Some things are set to id because it is our limiting factor. 
}

/*==/\/\/\/\/\/\/\/\++++/\/\/\/\/\/\/\/\==*\
 *  TODO: Does ID mean what it should mean MOST of the time? then its a bad name. I should change it. 
\*==\/\/\/\/\/\/\/\/++++\/\/\/\/\/\/\/\/==*/


//struct DataWraper //unused idea during development.
/*==/\/\/\/\/\/\/\/\++++/\/\/\/\/\/\/\/\==*\
 *  TODO: Read the following...
\*==\/\/\/\/\/\/\/\/++++\/\/\/\/\/\/\/\/==*/

/* Functions used in suffix_tree for loadContainer to replace vec 
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
    pub fn new(loader: impl Store<ID, DATA> + 'static, tracker: impl Tracker<ID, DATA> + 'static) -> Loader<ID, DATA> 
    {
        Self {
            raw: Vec::<DATA>::new(),    
            lookup: Vec::<ID>::new(), //layer of abstraction between raw index and ID
            store: Box::new(loader),
            tracker: Box::new(tracker),
            idcount: Default::default(), // 0 is the result.
            mem_limit: None
        }
        //set behavior to default sense rust doesn't have parameter function identity.
    }

    //Adds an element to the loader, stores data if needed.
    pub fn push(&self, obj: DATA){

    }
}

//manual impl of trait Debug sense the debug macro can't create something for Box, so we print the pointer.
impl<ID: Integer + Default + fmt::Debug, DATA: fmt::Debug> fmt::Debug for Loader<ID, DATA> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Loader")
            .field("raw", &self.raw)
            .field("lookup", &self.lookup)
            .field("idcount", &self.idcount)
            // Print the pointer address of the Box, probably will remove later.
            .field("store", &format_args!("{:p}", &*self.store)) 
            .finish()
    }
}

/*********************************************************************\
 *
 *        Trackers:
 *
\*********************************************************************/
struct DequeTrack<ID, DATA>{
    deq: VecDeque<ID>,
    phantom: PhantomData<DATA>, //pretend were using data at all.   
}

impl <ID, DATA> DequeTrack<ID, DATA> {
    pub fn new() -> DequeTrack<ID, DATA> {
        Self {
            deq: VecDeque::<ID>::new(),

            //this is that zero sized type. takes no memory, 
            //just makes the compiler happy I guess.
            phantom: PhantomData::<DATA>, 
        }
    }
}

impl<ID, DATA> Tracker<ID, DATA> for DequeTrack<ID, DATA>{
    fn update(&mut self, id: ID) {
        //Yea this isn't going to work at all. 
        self.deq.push_back(id);
    }

    fn least_used(&self) -> Vec<ID> { 
        //returns the bottom half?
        unimplemented!()
    }
}

/*********************************************************************\
 *
 *        Tests
 *
\*********************************************************************/
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /**********************************************************\
     *    Preliminary 
    \**********************************************************/
    #[derive(Debug)]
    struct TestStore {
        map: HashMap<u32,i32>,
    }

    //test loader trait implementer
    impl TestStore {
        pub fn new() -> TestStore {
            Self {
                map: HashMap::new()
            }
        }
    }

    impl Store<u32, i32> for TestStore {
        fn load(&mut self, id: u32) -> i32 {
            unimplemented!()
        }

        fn unload(&mut self, Data: i32) {
            unimplemented!()
        }
    }

    #[test]
    fn init() {
        let loader = Loader::new(TestStore::new(), DequeTrack::new());
        println!("{:?}", loader);

    }

}
