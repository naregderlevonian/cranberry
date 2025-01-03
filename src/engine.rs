use crate::alphabet;
use crate::handle;
use crate::scheme::Scheme;

pub enum Engine {
    Cranberry,
    Soviet1,
    Soviet2,
    Soviet3,
}

impl Engine {
    pub fn init(&self) -> Scheme {
        match self {
            Engine::Cranberry => Scheme::create(alphabet::cranberry::get(), handle::cranberry::translate),
            Engine::Soviet1 => Scheme::create(alphabet::soviet1::get(), handle::soviet::translate),
            Engine::Soviet2 => Scheme::create(alphabet::soviet2::get(), handle::soviet::translate),
            Engine::Soviet3 => Scheme::create(alphabet::soviet3::get(), handle::soviet::translate),
            _ => Scheme::create(alphabet::cranberry::get(), handle::cranberry::translate),
        }
    }
}
