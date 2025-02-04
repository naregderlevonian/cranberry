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
    ISO1954,
    ISO1968Base,
    ISO1968Alt1,
    ISO1968Alt2,
    ISO1995,
}

impl Engine {
    pub fn init(&self) -> scheme::Scheme {
        match self {
            Engine::Cranberry => scheme::Scheme::create(alphabet::cranberry::get(), handle::cranberry::translate),
            Engine::Soviet1 => scheme::Scheme::create(alphabet::soviet1::get(), handle::soviet::translate),
            Engine::Soviet2 => scheme::Scheme::create(alphabet::soviet2::get(), handle::soviet::translate),
            Engine::Soviet3 => scheme::Scheme::create(alphabet::soviet3::get(), handle::soviet::translate),
            Engine::ISO1954 => scheme::Scheme::create(alphabet::iso1954::get(), handle::direct::translate),
            Engine::ISO1968Base => scheme::Scheme::create(alphabet::iso1968base::get(), handle::direct::translate),
            Engine::ISO1968Alt1 => scheme::Scheme::create(alphabet::iso1968alt1::get(), handle::direct::translate),
            Engine::ISO1968Alt2 => scheme::Scheme::create(alphabet::iso1968alt2::get(), handle::direct::translate),
            Engine::ISO1995 => scheme::Scheme::create(alphabet::iso1995::get(), handle::direct::translate),
        }
    }
}
