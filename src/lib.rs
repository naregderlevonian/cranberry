mod alias;
mod alphabet;
mod engine;
mod handler;

pub enum Scheme {
    Cranberry,
    Soviet1,
    Soviet2,
    Soviet3,
    ISO1954,
    ISO1968Base,
    ISO1968Alt1,
    ISO1968Alt2,
    ISO1995,
    ALALC,
}

impl Scheme {
    pub fn init(&self) -> engine::Engine {
        match self {
            Scheme::Cranberry => engine::Engine::smart(alphabet::cranberry::get(), handler::cranberry::process),
            Scheme::Soviet1 => engine::Engine::smart(alphabet::soviet1::get(), handler::soviet::process),
            Scheme::Soviet2 => engine::Engine::smart(alphabet::soviet2::get(), handler::soviet::process),
            Scheme::Soviet3 => engine::Engine::smart(alphabet::soviet3::get(), handler::soviet::process),
            Scheme::ISO1954 => engine::Engine::basic(alphabet::iso1954::get()),
            Scheme::ISO1968Base => engine::Engine::basic(alphabet::iso1968base::get()),
            Scheme::ISO1968Alt1 => engine::Engine::basic(alphabet::iso1968alt1::get()),
            Scheme::ISO1968Alt2 => engine::Engine::basic(alphabet::iso1968alt2::get()),
            Scheme::ISO1995 => engine::Engine::basic(alphabet::iso1995::get()),
            Scheme::ALALC => engine::Engine::basic(alphabet::alalc::get()),
        }
    }
}
