use crate::generic_neat;
use crate::generic_neat::default::Input as I;
use crate::generic_neat::default::Hidden as H;
use crate::generic_neat::default::Output as O;
use crate::generic_neat::default::Link as L;
use crate::generic_neat::environment::Environment;

pub fn neat(environment: &dyn Environment<I, H, O, L>) {
    generic_neat::neat::<I, H, O, L>(environment);
}
