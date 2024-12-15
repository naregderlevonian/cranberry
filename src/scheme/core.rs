use crate::dictionary;
use crate::function::jakovlev;
use crate::map::Map;

pub fn new() -> Map {
    Map::new(dictionary::alphabet::new(), Box::new(jakovlev::parse))
}
