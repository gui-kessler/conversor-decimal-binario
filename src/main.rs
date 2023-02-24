use std::{io, fmt::Display};

#[derive(Debug)]
struct Bin {
    value: Vec<i32>
}

impl Display for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vec_aux: Vec<String> = vec![];
        for i in self.value.iter() {
            vec_aux.push(i.to_string());
        }
        write!(f, "{}", vec_aux.join(""))
    }
}

fn main() {
    let mut numero_decimal = String::new();
    let mut numero_binario = Bin {value: vec![]};

    println!("\nConversor de numeros decimais para a base binaria\n");
    println!("Digite um numero decimal para converter:");

    io::stdin().read_line(&mut numero_decimal)
        .expect("Erro ao ler entrada");

    let numero_decimal: i32 = match numero_decimal.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    dec_to_bin(&mut numero_binario, numero_decimal);

    println!("{}", numero_binario);
}

fn dec_to_bin(numero_binario: &mut Bin, numero_decimal: i32) {
    if numero_decimal > 0 {
        let div = numero_decimal / 2;
        let resto = numero_decimal % 2;
        numero_binario.value.insert(0, resto);
        dec_to_bin(numero_binario, div);
    }
}
