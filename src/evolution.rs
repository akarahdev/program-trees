use ordered_float::OrderedFloat;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::expr::ExprTree;

// S = total size of population
// F = size of population that get to live to next generation
pub struct Universe<const S: usize, const F: usize, const M: i32> {
    members: [UniverseMember; S],
}

pub struct UniverseMember {
    tree: ExprTree,
    fitness: OrderedFloat<f64>,
}

impl<const S: usize, const F: usize, const M: i32> Universe<S, F, M> {
    pub fn new() -> Self {
        Universe {
            members: std::array::from_fn(|_| UniverseMember {
                tree: ExprTree::random::<M>(0),
                fitness: 0.0.into(),
            }),
        }
    }

    pub fn sort(mut self, objective_function: fn(&ExprTree) -> f64) -> Self {
        let mut new_members: Vec<_> = self
            .members
            .into_par_iter()
            .map(|x| UniverseMember {
                fitness: objective_function(&x.tree).into(),
                tree: x.tree,
            })
            .collect();
        new_members.sort_by_cached_key(|x| x.fitness);
        new_members.reverse();
        let mut iter = new_members.into_iter();
        self.members = std::array::from_fn(|_| iter.next().unwrap());
        self
    }

    pub fn step(mut self, objective_function: fn(&ExprTree) -> f64) -> Self {
        let mut new_members = self
            .members
            .into_iter()
            .map(|x| {
                (
                    UniverseMember {
                        tree: x.tree.clone(),
                        fitness: 0.0.into(),
                    },
                    UniverseMember {
                        tree: x.tree.mutated(),
                        fitness: 0.0.into(),
                    },
                )
            })
            .flat_map(|(a, b)| [a, b])
            .take(F);
        self.members = std::array::from_fn(|_| {
            new_members.next().unwrap_or_else(|| UniverseMember {
                tree: ExprTree::random::<3>(0),
                fitness: 0.0.into(),
            })
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

    use crate::values::{Number, ValTree};

    use super::Universe;

    #[test]
    pub fn approximate_sqrt() {
        let mut universe: Universe<100, 5, 2> = Universe::default();
        let mut last_best = 0.0;
        loop {
            universe = universe.step(|expr| {
                let test_values: [f64; 128] = std::array::from_fn(|idx| idx as f64);

                let result: f64 = test_values
                    .iter()
                    .map(|&x| {
                        let expected = x.sqrt();
                        let candidate_result =
                            expr.eval(&[ValTree::Number(Number::new(x))]).as_f64();
                        let error = (expected - candidate_result).abs();
                        1.0 / (1.0 + error)
                    })
                    .sum();
                if result.is_nan() { 0.0 } else { result }
            });

            let new_curr = universe.members[0].fitness;

            if new_curr != last_best {
                println!("Best: {:?}", universe.members[0].fitness);
                println!("{:?}", universe.members[0].tree);
                last_best = *new_curr;
            }
        }
    }
}
