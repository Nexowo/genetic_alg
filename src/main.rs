use rand::Rng;

struct Individu {
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

    fn new(len : u32) -> Self {
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

    fn print(&self) {
        for i in 0..self.genes.len() {
            print!("{}",self.genes[i])
        }
        println!(", value = {}", self.value)
    }

    fn mutation(&mut self, ind : &mut Self) -> Self {
        let mut_point = rand::thread_rng().gen_range(0..self.genes.len());
        let mut genes = Vec::from(&mut self.genes[..mut_point]);
        genes.append(&mut ind.genes[mut_point..].to_vec());
        let mut child = Individu {
            genes : genes,
            value : 0
        };
        child.calc_val();
        child
    }
}

fn main() {
    let len = 8;
    let mut ind_1 = Individu::new(len);
    let mut ind_2 = Individu::new(len);

    println!("Generation 1 -----------------------");
    ind_1.print();
    ind_2.print();

    ind_1 = ind_1.mutation(&mut ind_2);
    ind_2 = ind_2.mutation(&mut ind_1);

    println!("Generation 2 -----------------------");
    ind_1.print();
    ind_2.print();
}
