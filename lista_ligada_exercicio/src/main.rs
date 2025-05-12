use std::collections::LinkedList;

fn main() {

    let mut frases = LinkedList::new();

    frases.push_back("Acredite no Elias!");
    frases.push_back("Elias, nunca desista dos seus sonhos.");
    frases.push_back("O esforço de hoje é o sucesso de amanhã, é noix.");
    frases.push_back("Você é capaz de coisas incríveis até de copiar meu código.");
    frases.push_back("A persistência leva à conquista, sai daqui copiador.");
    frases.push_back("Cada dia é uma nova chance, de copiar o código do Elias.");
    frases.push_back("Confie no seu potencial, não copie meu código.");
    frases.push_back("Grandes coisas levam tempo, menos copiar código dos outros.");
    frases.push_back("Aprenda com os erros, não copiando meu código.");
    frases.push_back("A jornada é tão importante quanto o destino e o destino é tao importante quanto a jornada..");

    let mut iter = frases.iter();
    let terceiro = iter.nth(2);
    if let Some(frase) = terceiro {
        println!("Terceira frase: {}", frase);
    } else {
        println!("Não há terceira frase.");
    }

    println!("Tamanho da lista: {}", frases.len());
    // by: elias
}