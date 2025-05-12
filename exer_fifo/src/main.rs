use std::collections::VecDeque;

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    fila.push_back(10);
    fila.push_back(20);
    fila.push_back(30);

    println!("Fila após inserções: {:?}", fila);

    if let Some(valor) = fila.pop_front() {
        println!("removido: {}", valor);
    }

    println!("Fila após uma remoção: {:?}", fila);

    if let Some(proximo) = fila.front() {
        println!("Próximo elemento da fila: {}", proximo);
    }
}