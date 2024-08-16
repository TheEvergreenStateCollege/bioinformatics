//loader is a container that is used in place of a vector to allow reuse of alocated memory. 

//#[derive(Debug)]
struct Loader<ID, DATA, LOAD, UNLOAD>
where 
    LOAD: FnMut(ID) -> DATA,
    UNLOAD: FnMut(DATA),
{
    raw: Vec<DATA>,       //data container
    index: Vec<ID>,       //stores index values. hash map?
    load:  LOAD,          //function (closuer) for loading data in whatever way
    unload: UNLOAD        //function (closuer) for unloading data, depending.
}

impl<ID, DATA, LOAD, UNLOAD> Loader<ID, DATA, LOAD, UNLOAD> 
where 
    LOAD: FnMut(ID) -> DATA,
    UNLOAD: FnMut(DATA),
{
    pub fn new(load: LOAD, unload: UNLOAD) -> Loader<ID, DATA, LOAD, UNLOAD> {
        Self {
            raw: Vec::new(),
            index: Vec::new(),
            load,
            unload
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn init() {
        let mut storeage: HashMap<u32, i32> = HashMap::new();
        let mut idcounter = 0;

        let load_test_closer = |ID: u32| -> i32 
        {
            match storeage.get(&ID)
            {
                Some(i) => *i, 
                None => panic!("ID not found!"),
            }
        };

        let unload_test_closer = |Data: i32 | 
        {
            idcounter = idcounter +1;
            storeage.insert(idcounter, Data);
        };

        let loader = Loader::new(load_test_closer, unload_test_closer);

    }

}
