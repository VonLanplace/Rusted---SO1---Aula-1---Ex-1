use std::{io::stdin, time::Instant};

fn main() {
    calc_tempo(100);
    calc_tempo(1000);
    calc_tempo(10000);
}
fn input_i32(text: &str) -> i32 {
    loop {
        println!("{text}");
        let mut entrada = String::new();
        if stdin().read_line(&mut entrada).is_err() {
            println!("Erro de Leitura!");
            continue;
        }
        let saida: i32 = match entrada.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entrada Inválida");
                continue;
            }
        };
        return saida;
    }
}
fn calc_tempo(tamanho: i32) {
    print!("O tempo do vetor com {tamanho:5.} numeros foi ");
    let numeros = vec![0; tamanho as usize];
    let tempo = Instant::now();
    for _numero in numeros {}
    let duração = tempo.elapsed();
    println!("{}s.", duração.as_secs_f64());
}
