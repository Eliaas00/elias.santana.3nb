use std::collections::HashMap;

fn main() {
    // Criação do HashMap
    let mut estoque = HashMap::new();

    // 1. Inserindo valores
    estoque.insert("Banana", 100);
    estoque.insert("pepino", 50);
    estoque.insert("maçã", 2);
    estoque.insert("caqui", 20);

    // 2. Acessando de forma segura os valores

    if let Some(qtde) = estoque.get("maçã") {
        println!("Temos {:?} maçãs em estoque!", qtde)
    }

    // 3. Atualizando estoque

    estoque.entry("pepino").and_modify(|qtde| *qtde += 100);
    if let Some(qtde) = estoque.get("pepino") {
        println!("Temos {:?} pepino em estoque!", qtde)
    }

    // 4. Removendo

    estoque.remove("caqui");
    println!("{:?}", estoque);

    // 5. Filtrar todas as frutas acima de 100 unidades

    estoque.retain(|frutas, &mut qtde| qtde > 100);
    println!("${:?}", estoque);

    // 6. Limpeza total!

    estoque.clear();
    println!("{:?}", estoque);
}