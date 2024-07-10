use std::{collections::HashMap, hash::Hash};

use crate::token::Token;



pub struct Lookup_table{
    reserved_characters: HashMap<char, Token>
}

impl Lookup_table{
    pub fn new() -> Self{
        Lookup_table { reserved_characters: HashMap::from_iter([
            
        ].into_iter()) }
    }
}