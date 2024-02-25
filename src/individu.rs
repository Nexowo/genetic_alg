use rand::Rng;

pub struct Individu {
    genes : Vec<usize>,
    value : u64
}

impl Individu {
    fn calc_val(&mut self) {
        let mut value : u64 = 0;
        let j : usize = self.genes.len();
        for i in 0..j {
            value+=(self.genes[j-i-1] as u64)*2_u64.pow(i as u32);
        }
        self.value = value;
    }

    pub fn new(len : u32) -> Self {
        let mut genes = Vec::new();

        for _ in 0..len {
            genes.push(rand::thread_rng().gen_range(0..2));
        }

        let mut indiv = Individu {
            genes : genes,
            value : 0
        };

        indiv.calc_val();
        indiv
    }

    pub fn print(&self) {
        for i in 0..self.genes.len() {
            print!("{}",self.genes[i])
        }
        println!(", value = {}", self.value)
    }

    pub fn croisement(&mut self, ind : &mut Self) -> Self {
        let crois_point = rand::thread_rng().gen_range(0..self.genes.len());
        let mut genes = Vec::from(&mut self.genes[..crois_point]);
        genes.append(&mut ind.genes[crois_point..].to_vec());
        let mut child = Individu {
            genes : genes,
            value : 0
        };
        child.calc_val();
        child
    }

    pub fn mutation(&mut self){
        let mut_point = rand::thread_rng().gen_range(0..self.genes.len());
        if self.genes[mut_point] == 1 {
            self.genes[mut_point] = 0;
        }
        else {
            self.genes[mut_point] = 1;
        }
        self.calc_val();
    }
}
