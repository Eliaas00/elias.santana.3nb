struct QuickSort {
    array: Vec<i32>, // Aqui eu estou definindo um array de inteiros
}

impl QuickSort { // Aqui estou implementando o método QuickSort
    fn sort(&mut self) { // Self passei como parâmetro e é mutável
        // Aqui eu estou definindo o método sort que irá ordenar o array
        let len = self.array.len();
        if len > 1 { // Agroa estou verificando se ele tem mais de um elemento
            self.quick_sort(0, len - 1); 
        }
    }

    fn quick_sort(&mut self, num1: usize, num2: usize) { // Creiei outra função para ordenar
        if num1 < num2 { // Condição para verificar se o primeiro número é menor que o segundo
            let p = self.part(num1, num2); // Aqui eu estou chamando a função part que vai dividir o array
            if p > 0 { // Verifico se p é maior que 0 ai não da erro de índice
                self.quick_sort(num1, p - 1);
            }
            self.quick_sort(p + 1, num2);
        }
    }

    fn part(&mut self, num1: usize, num2: usize) -> usize { // Função part que vai dividir o array
        let pivot = self.array[num2]; // Aqui eu estou pegando o último elemento como pivô camisa 9 hahah
        let mut i = num1; // Aqui eu estou definindo i como o primeiro elemento do array
        for j in num1..num2 {
            if self.array[j] < pivot {
                self.array.swap(i, j);
                i += 1;
            }
        }
        self.array.swap(i, num2); // Aqui eu estou trocando o elemento i com o camisa 9
        i
    }
}

fn main() { // Chamo a função main (ponto de entrada do programa)
    let mut sorter = QuickSort {
        array: vec![34, 7, 23, 32, 5, 62, 31, 12, 43, 3], // Defini o array conforme solicitado
    };

    println!("Professor, aqui é o array original: {:?}", sorter.array);
    sorter.sort();
    println!("Professor, aqui é o array ordenado: {:?}", sorter.array);

    println!("Forte abraço, RUST na Veiaaaaaa!"); 
}