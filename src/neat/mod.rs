mod dot;
mod genome;
mod individual;
mod nodes;
mod population;
mod species;

use population::Population;

pub fn neat() {
    let population = Population::new(50, 0.5, 4, 2);
}
