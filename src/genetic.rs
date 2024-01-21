use rand::Rng;

const POP_SIZE: usize = 20;

pub trait ErrorCalculator{
    fn calculate(&self, genes: &Vec<f64>) -> f64;
}

struct Solution{
    genes: Vec<f64>,
    error: Option<f64>
}

pub fn optimise(genes_count: usize, error_calculator: &dyn ErrorCalculator, mutation_chance: f64, epochs: usize) -> Vec<f64> {
    assert!(mutation_chance >= 0.0 && mutation_chance <= 1.0);
    let mut rng = rand::thread_rng();
    let mut solutions: Vec<Solution> = (0..POP_SIZE)
        .map(|_| Solution{ 
            genes: (0..=genes_count)
                .map(|_2| rng.gen_range(0.0..1.0))
                .collect(),
            error:None})
        .collect();

    for i in &mut solutions {
        if i.error.is_none() {
            i.error = Some(error_calculator.calculate(&i.genes));
        }
    }

    for _ in 0..epochs {
        for i in 2..POP_SIZE {
            for j in 0..genes_count {
                if rng.gen_bool(mutation_chance) {
                    solutions[i].genes[j] = rng.gen_range(0.0..1.0);
                }
                else {
                    solutions[i].genes[j] = if rng.gen_bool(0.5) {solutions[0].genes[j]} else {solutions[1].genes[j]};
                }
            }
            solutions[i].error = None;
        }


        for i in &mut solutions {
            if i.error.is_none() {
                i.error = Some(error_calculator.calculate(&i.genes));
            }
        }

        solutions.sort_by(|a, b| a.error.unwrap().partial_cmp(&b.error.unwrap()).unwrap());
    }

    solutions[0].genes.clone()
}
