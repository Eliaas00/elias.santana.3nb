use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    {
        let mut borrowed = data.borrow_mut(); // Empréstimo mutável em tempo de execução
        *borrowed += 1;
    }
    println!("Valor atualizado: {}", data.borrow());
}