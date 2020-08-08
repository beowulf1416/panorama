extern crate serde;

use std::fmt::{Debug};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Source {
    pub type_id: String,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct Sources {
    pub cameras: Vec<Source>
}