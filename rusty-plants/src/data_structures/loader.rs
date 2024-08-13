//loader is a container that is used in place of a vector to allow reuse of alocated memory. 

#[derive(Debug)]
struct Loader<Data, ID>{
    raw: Vec<Data>,       //data container
    index: Vec<ID>,       //stores index values. hash map?
    load: fn(ID) -> Data, //function for loading data in whatever way
    unload: fn(Data)      //function for unloading data, depending.
}

impl<Data, ID> Loader<Data, ID> {
    pub fn new(load: fn(ID) -> Data, unload: fn(Data)) -> Loader<Data, ID> {
        let mut l = Loader{
            raw: Vec::new(),
            index: Vec::new(),
            load: load,
            unload: unload
        };
        l
    }
}
