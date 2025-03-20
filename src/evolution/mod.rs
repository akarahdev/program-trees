use ordered_float::OrderedFloat;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{expr::ExprTree, values::ValTree};

// S = total size of population
// F = size of population that get to live to next generation
pub struct Universe<const S: usize, const F: usize, const M: i32> {
    members: [ExprTree; S],
}

impl<const S: usize, const F: usize, const M: i32> Universe<S, F, M> {
    pub fn new() -> Self {
        Universe {
            members: std::array::from_fn(|_| ExprTree::random_expr::<M>()),
        }
    }

    pub fn sort(mut self, objective_function: fn(ValTree) -> f64) -> Self {
        let mut new_members: Vec<_> = self
            .members
            .into_par_iter()
            .map(|x| (x.eval(), x))
            .map(|x| (OrderedFloat::from(objective_function(x.0)), x.1))
            .collect();
        new_members.sort_by_cached_key(|x| x.0);
        new_members.reverse();
        let mut iter = new_members.into_iter().map(|x| x.1);
        self.members = std::array::from_fn(|_| iter.next().unwrap());
        self
    }

    pub fn step(mut self, objective_function: fn(ValTree) -> f64) -> Self {
        let mut new_members = self
            .members
            .into_iter()
            .map(|x| (x.clone(), x.mutated()))
            .flat_map(|(a, b)| [a, b])
            .take(F);
        self.members = std::array::from_fn(|_| {
            new_members
                .next()
                .unwrap_or_else(ExprTree::random_expr::<3>)
        });
        self.sort(objective_function)
    }
}

impl<const S: usize, const F: usize, const M: i32> Default for Universe<S, F, M> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Universe;

    #[test]
    pub fn approximate_max() {
        let mut universe: Universe<10000, 50, 2> = Universe::default();
        loop {
            universe = universe.step(|x| {
                let val = x.as_f64();
                if val.is_infinite() || val.is_nan() {
                    return f64::MIN;
                }
                val
            });
            println!(
                "Generation best: {:#?}",
                universe.members[0].eval().as_f64()
            );
        }
    }
}
