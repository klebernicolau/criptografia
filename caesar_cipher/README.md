# Caesar Cipher

O Caesar Cipher é um programa simples em Rust para criptografar e descriptografar mensagens usando a cifra de César.

A cifra de César é um tipo de criptografia por substituição em que cada letra no texto original é deslocada um certo número de posições para a direita ou esquerda no alfabeto. Neste programa, você pode escolher a rotação desejada para a cifra de César, permitindo que você personalize o processo de criptografia e descriptografia.

## Pré-requisitos

Certifique-se de ter o Rust instalado em sua máquina. Se você ainda não tiver o Rust, pode instalá-lo a partir do site oficial: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Como usar

1. Clone este repositório em sua máquina local:

git clone https://github.com/klebernicolau/criptografia.git

2. Navegue até o diretório do projeto:

cd caesar_cipher

3. Compile o código usando o Cargo:

cargo build --release

4. Execute o programa:

cargo run --release

5. Siga as instruções do menu para criptografar ou descriptografar mensagens usando a cifra de César.

- Escolha a opção "1. Criptografar mensagem" para inserir uma mensagem e a rotação desejada para a cifra de César. O programa criptografará a mensagem e exibirá o resultado.
- Escolha a opção "2. Descriptografar mensagem" para inserir uma mensagem cifrada e a rotação desejada para a cifra de César. O programa descriptografará a mensagem e exibirá o resultado.
- Escolha a opção "3. Sair" para encerrar o programa.

## Personalização da Rotação

Durante a execução do programa, você pode escolher a rotação desejada para a cifra de César. A rotação determina quantas posições as letras serão deslocadas no alfabeto durante o processo de criptografia ou descriptografia. Você pode experimentar diferentes valores de rotação para obter resultados diferentes.

## Contribuição

Contribuições são bem-vindas! Se você tiver sugestões de melhorias ou encontrar problemas no código, sinta-se à vontade para enviar pull requests ou relatar problemas no repositório.

## Licença

Este projeto é licenciado sob a licença [MIT](LICENSE).

## Documentação para Usuários Avançados

Para usuários avançados que desejam personalizar ainda mais o programa, é possível fazer modificações adicionais no código-fonte. O programa foi escrito em Rust e utiliza a biblioteca cipher_crypt para a implementação da cifra de César. Você pode explorar a documentação da biblioteca e fazer modificações no código para adicionar funcionalidades extras ou adaptar o programa às suas necessidades específicas.
