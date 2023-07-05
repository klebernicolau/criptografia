extern crate cipher_crypt;

use cipher_crypt::{Cipher, Caesar};

use std::io;
use std::io::prelude::*;

fn encrypt_message(rotacao: u8) {
    let mut msgorig = String::new();

    let cifra = Caesar::new(rotacao.into()).unwrap();

    println!("CIFRA DE CAESAR");
    println!();

    println!("Informe a mensagem a ser cifrada:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut msgorig).expect("Entrada incorreta");

    if let Some('\n') = msgorig.chars().next_back() {
        msgorig.pop();
    }
    if let Some('\r') = msgorig.chars().next_back() {
        msgorig.pop();
    }
    println!();

    let msgcifr = cifra.encrypt(&msgorig).unwrap();

    println!("Mensagem original .....: {}", msgorig);
    println!("Mensagem cifrada ......: {}", msgcifr.to_uppercase());
}

fn decrypt_message(rotacao: u8) {
    let mut msgcifr = String::new();

    let cifra = Caesar::new(rotacao.into()).unwrap();

    println!("CIFRA DE CAESAR");
    println!();

    println!("Informe a mensagem cifrada:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut msgcifr).expect("Entrada incorreta");

    if let Some('\n') = msgcifr.chars().next_back() {
        msgcifr.pop();
    }
    if let Some('\r') = msgcifr.chars().next_back() {
        msgcifr.pop();
    }
    println!();

    let msgdeci = cifra.decrypt(&msgcifr).unwrap();

    println!("Mensagem cifrada ......: {}", msgcifr);
    println!("Mensagem decifrada ....: {}", msgdeci);
}

fn main() {
    println!("CIFRA DE CAESAR");
    println!();

    println!("Informe a rotação desejada para a criptografia:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Entrada incorreta");

    let rotacao: u8 = input.trim().parse().expect("Valor inválido");

    loop {
        println!("MENU");
        println!("1. Criptografar mensagem");
        println!("2. Descriptografar mensagem");
        println!("3. Sair");
        println!();

        println!("Selecione uma opção:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Entrada incorreta");

        match input.trim() {
            "1" => {
                println!("Informe a rotação desejada para a criptografia:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Entrada incorreta");

                let rotacao_criptografia: u8 = input.trim().parse().expect("Valor inválido");
                encrypt_message(rotacao_criptografia);
                println!();
            }
            "2" => {
                println!("Informe a rotação desejada para a descriptografia:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Entrada incorreta");

                let rotacao_descriptografia: u8 = input.trim().parse().expect("Valor inválido");
                decrypt_message(rotacao_descriptografia);
                println!();
            }
            "3" => {
                break;
            }
            _ => {
                println!("Opção inválida. Tente novamente.");
                println!();
            }
        }
    }

    println!("Programa encerrado.");
}

