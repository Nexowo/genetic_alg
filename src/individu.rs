use rand::Rng;

///# Individual Struct
/// Has two attributes:
/// * `genes` : a vector of `usize` each element is either 0 or 1
/// * `value` : the quality of an individual which is the conversion in base 10 of the binary sequence of `genes`
pub struct Individu {
    genes : Vec<usize>,
    value : u64
}

impl Individu {
    /// Calculates the value of the individual and actualize it.
    fn calc_val(&mut self) {
        let mut value : u64 = 0;
        let j : usize = self.genes.len();
        for i in 0..j {
            value+=(self.genes[j-i-1] as u64)*2_u64.pow(i as u32);
        }
        self.value = value;
    }

    /// Creates a new individual  
    /// :param len: the length of the individual  
    /// :type len: u32  
    /// :return: a new generated individual  
    /// :rtype: Individu
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

    /// Formated print for an Individual
    pub fn print(&self) {
        for i in 0..self.genes.len() {
            print!("{}",self.genes[i])
        }
        println!(", value = {}", self.value)
    }

    /// Creates a new individual using the current individual and the other in parameter  
    /// :param ind: the other individual to use  
    /// :type ind: Individu  
    /// :return: the new individual created  
    /// :rtype: Individu
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

    /// Mutation operator for individuals  
    /// Takes a random bit of an individual and flips it to the value `0` if it was `1` and `1` elsewhere
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

    /// Returns a copy of the current individual  
    /// :return: a copy of the individual
    /// :rtype: Individu
    pub fn copy(&self) -> Self {
        let copied_ind = Individu {
            genes : self.genes.clone(),
            value : self.value
        };
        copied_ind
    }
}
