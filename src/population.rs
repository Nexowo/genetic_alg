mod individu;
use crate::individu::Individu;

pub struct Population {
    individuals : Vec<Individu>,
    best_indiv: Individu
}

impl Population {
    pub fn new(pop_size : usize, indiv_size : usize) -> Self {
        let mut individuals = Vec::new();
        for i in 0..pop_size {
            individuals.push(Individu::new(indiv_size))
        }
        
        let pop = Population{
            individuals : individuals,
            best_indiv : individuals[0].copy()
        };
        pop
    }

    fn sort(&mut self){
        let len = self.individuals.len();
        for i in 0..len {
            let n_switch = 0;
            for j in 0..len-i-1 {
                if self.individuals[j] < self.individuals[j+1] {
                    let switch = self.individuals[j];
                    self.individuals[j] = self.individuals[j+1];
                    self.individuals[j+1] = switch;
                    n_switch+=1
                }
            }
            if 0 == n_switch {
                break;
            }
        }
    }
}