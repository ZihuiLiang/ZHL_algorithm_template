#[derive(Clone, Debug)]
pub struct StringHash {
    hash: Vec<u64>,
    pow: Vec<u64>,
    base: u64,
    mod_: u64,
}

pub struct KStringHash {
    string_hashs: Vec<StringHash>,  
}