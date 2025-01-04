mod alias;
mod alphabet;
mod class;
mod iotized;
mod handle;
mod scheme;

pub enum Engine {
    Cranberry,
    Soviet1,
    Soviet2,
    Soviet3,
}

impl Engine {
    pub fn init(&self) -> scheme::Scheme {
        match self {
            Engine::Cranberry => scheme::Scheme::create(alphabet::cranberry::get(), handle::cranberry::translate),
            Engine::Soviet1 => scheme::Scheme::create(alphabet::soviet1::get(), handle::soviet::translate),
            Engine::Soviet2 => scheme::Scheme::create(alphabet::soviet2::get(), handle::soviet::translate),
            Engine::Soviet3 => scheme::Scheme::create(alphabet::soviet3::get(), handle::soviet::translate),
        }
    }
}
