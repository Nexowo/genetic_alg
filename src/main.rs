mod individu;
use crate::individu::Individu;
fn main() {
    let len = 8;
    let mut ind_1 = Individu::new(len);
    let mut ind_2 = Individu::new(len);

    println!("Generation -----------------------");
    ind_1.print();
    ind_2.print();

    let ind_3 = ind_1.croisement(&mut ind_2);
    ind_2 = ind_2.croisement(&mut ind_1);

    println!("Croisement -----------------------");
    ind_3.print();
    ind_2.print();

    ind_1.mutation();
    ind_2.mutation();

    println!("Mutation ------------------------");
    ind_1.print();
    ind_2.print();
}
