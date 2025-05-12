use std::collections::VecDeque;
use std::io;

fn adicionar(fila: &mut VecDeque<i32>) {
    println!("Digite um número inteiro para adicionar:");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor).expect("Erro na leitura");
    if let Ok(num) = valor.trim().parse::<i32>() {
        fila.push_back(num);
        println!("{} adicionado à fila.", num);
    } else {
        println!("Valor inválido.");
    }
}

fn remover(fila: &mut VecDeque<i32>) {
    if let Some(removido) = fila.pop_front() {
        println!("{} removido da fila.", removido);
    } else {
        println!("Fila vazia, nada a remover.");
    }
}

fn ver_frente(fila: &VecDeque<i32>) {
    if let Some(frente) = fila.front() {
        println!("Elemento na frente da fila: {}", frente);
    } else {
        println!("Fila vazia.");
    }
}

fn esta_vazia(fila: &VecDeque<i32>) {
    if fila.is_empty() {
        println!("A fila está vazia.");
    } else {
        println!("A fila NÃO está vazia.");
    }
}

fn tamanho(fila: &VecDeque<i32>) {
    println!("Tamanho atual da fila: {}", fila.len());
}

fn main() {
    let mut fila: VecDeque<i32> = VecDeque::new();

    loop {
        println!("\n##---- Menu ----##");
        println!("1. Adicionar elemento");
        println!("2. Remover elemento");
        println!("3. Ver elemento da frente");
        println!("4. Verificar se a fila está vazia");
        println!("5. Ver tamanho da fila");
        println!("0. Sair");

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).expect("Erro na leitura");
        let escolha = escolha.trim().parse::<u32>().unwrap_or(99);

        match escolha {
            1 => adicionar(&mut fila),
            2 => remover(&mut fila),
            3 => ver_frente(&fila),
            4 => esta_vazia(&fila),
            5 => tamanho(&fila),
            0 => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}