pub trait Log {
    fn log(&mut self, organism: &crate::generic_neat::organism::Organism);
}

pub struct DefaultLogger {}

impl Log for DefaultLogger {
    fn log(&mut self, _: &crate::generic_neat::organism::Organism) {}
}

impl Default for DefaultLogger {
    fn default() -> DefaultLogger {
        DefaultLogger {}
    }
}
