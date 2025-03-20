use crate::{expr::ExprTree, values::ValTree};

// S = total size of population
// F = size of population that get to live to next generation
pub struct Universe<const S: usize, const F: usize> {
    members: [ExprTree; S],
}

impl<const S: usize, const F: usize> Universe<S, F> {
    pub fn new() -> Self {
        Universe {
            members: std::array::from_fn(|_| ExprTree::random_expr()),
        }
    }

    pub fn step(&mut self, objective_function: fn(ValTree) -> i64) {
        let mut new_members: Vec<(ExprTree, i64)> = self
            .members
            .iter()
            .map(|x| (x.clone(), x.eval()))
            .map(|x| (x.0, objective_function(x.1)))
            .collect();
        new_members.sort_by_cached_key(|x| x.1);
        let mut new_members = new_members
            .into_iter()
            .map(|x| x.0)
            .map(|x| {
                let mut c1 = x.clone();
                c1.mutate();
                (c1, x)
            })
            .flat_map(|(a, b)| [a, b])
            .take(F);

        self.members =
            std::array::from_fn(|_| new_members.next().unwrap_or_else(ExprTree::random_expr));
    }
}

impl<const S: usize, const F: usize> Default for Universe<S, F> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Universe;

    #[test]
    pub fn approximate_max() {
        let mut universe: Box<Universe<10000, 1000>> = Box::default();
        for _ in 1..5 {
            universe.step(|x| x.as_f64() as i64);
        }
        println!("{:#?}", universe.members[0]);
        println!("{:#?}", universe.members[0].eval());
    }
}
