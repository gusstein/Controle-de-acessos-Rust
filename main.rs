use std::io;
use std::io::Write;

#[derive(Debug, Clone, PartialEq)]
enum Permissoes {
    Ler,
    Escrever,
    Executar
}

type Arquivo = (String, String, Vec<Permissoes>);

fn main(){
    print!("-------------------CONTROLE DE ACESSOS--------------------");
    println!();

    let mut arquivos: Vec<Arquivo> = vec![("ola.txt".to_string(), "olá senhores".to_string(), vec![Permissoes::Ler, Permissoes::Escrever]), 
                                            ("main.rs".to_string(), "fn main(println!('salve');){}".to_string(), vec![Permissoes::Ler, Permissoes::Executar]), 
                                            ("model.java".to_string(), "hello sou o javax".to_string(), vec![Permissoes::Ler, Permissoes::Escrever, Permissoes::Executar])];
    loop {

        println!("--------------------ARQUIVOS--------------------");
        println!();
        for arquivo in &arquivos {
            println!("Nome: {}", arquivo.0);
            println!("Conteúdo: {}", arquivo.1);
            println!("Permissões: {:?}", arquivo.2);
            println!();
        }

        let opcao = seleciona_acao();

        let permissao = match opcao {
            1 => Permissoes::Ler,
            2 => Permissoes::Escrever,
            3 => Permissoes::Executar,
            _ => {
                println!("Saindo...");
                break;
            }
        };

        let indice_arquivo = selecionar_arquivo(&arquivos);

        if indice_arquivo < 0{
            println!("arquivo não encontrado");
            break
        }

        let mut arquivo_selecionado = arquivos[indice_arquivo as usize].clone();

        realizar_acao(&mut arquivo_selecionado, permissao, indice_arquivo, &mut arquivos);
    }
    
}


fn seleciona_acao() -> i32{

    println!("1. Ler");
    println!("2. Escrever");
    println!("3. Executar");
    println!("Outro número. Sair");
    println!();
    println!("Digite uma opção: ");
    println!();

    let mut buffer: String = String::new();
    io::stdout().flush().expect("Erro ao atualizar stdout");
    io::stdin().read_line(&mut buffer).expect("Erro ao ler número.");
    let opcao: i32 = buffer.trim().parse().expect("Erro ao converter número");

    opcao
}

fn selecionar_arquivo(arquivos: &Vec<Arquivo>) -> i32 {
    println!("Digite o nome do arquivo: ");
    println!();

    let mut buffer_nome = String::new();
    io::stdout().flush().expect("Erro ao atualizar stdout");
    io::stdin().read_line(&mut buffer_nome).expect("Erro ao ler nome.");
    let nome_arquivo = buffer_nome.trim().to_string();

    for (indice, arquivo) in arquivos.iter().enumerate() {
        if arquivo.0 == nome_arquivo {
            return indice as i32;
        }
    }
    -1
}

fn realizar_acao(arquivo: &Arquivo, permissao: Permissoes, indice: i32, arquivos: &mut Vec<Arquivo>) {
    if possui_permissao(arquivo, &permissao) {
        match permissao {
            Permissoes::Ler => {
                println!("LER - {:.}", arquivo.1);
            }
            Permissoes::Escrever => {
                println!("ESCREVER - Didite o noco valor:");
                println!();
        
                let mut buffer_nome = String::new();
                io::stdout().flush().expect("Erro ao atualizar stdout");
                io::stdin().read_line(&mut buffer_nome).expect("Erro ao ler nome.");
                let novo_valor = buffer_nome.trim().to_string();
  
                if let Some(tupla) = arquivos.get_mut(indice as usize) {
                    tupla.1 = novo_valor;
                }
            }
            Permissoes::Executar => {
                println!("EXECUTAR - {:.}", arquivo.1);
            }
        }
    }
    else{
        println!("Não possui essa permissao");
    }

}

fn possui_permissao(arquivo: &Arquivo, permissao: &Permissoes) -> bool {
    match permissao {
        &Permissoes::Ler => {
            arquivo.2.contains(&Permissoes::Ler)
        }
        &Permissoes::Escrever => {
            arquivo.2.contains(&Permissoes::Escrever)
        }
        &Permissoes::Executar => {
            arquivo.2.contains(&Permissoes::Executar)
        }
    }
}