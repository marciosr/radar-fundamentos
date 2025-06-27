use chrono::Local;
use csv::Writer;
use serde::Deserialize;
use serde_yaml;
use std::fs::File;
use std::io::Read;
use yahoo_finance_api as yahoo;

/// Estrutura esperada para o arquivo YAML de ativos
#[derive(Debug, Deserialize)]
struct AtivosYaml {
    ativos: Vec<String>,
}

/// Carrega uma lista de ativos a partir de um arquivo YAML
/// O YAML deve ter o seguinte formato:
///
/// ```yaml
/// ativos:
///   - PETR4.SA
///   - KLBN11.SA
///   - FRAS3.SA
/// ```
///
pub fn carregar_ativos_yaml(caminho: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut file = File::open(caminho)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let ativos_yaml: AtivosYaml = serde_yaml::from_str(&contents)?;
    Ok(ativos_yaml.ativos)
}

/// Percorre uma lista de ativos e coleta a última cotação de cada um,
/// utilizando o intervalo "1d" da API do Yahoo Finance para maior estabilidade.
///
/// As cotações são gravadas em um arquivo CSV com as colunas:
/// `ativo`, `data`, `cotacao`.
///
/// Este arquivo pode ser importado diretamente no LibreOffice,
/// substituindo planilhas que dependiam do Google Finance.
///
/// Exemplo de saída CSV:
/// ```csv
/// ativo,data,cotacao
/// PETR4.SA,2025-06-22,39.85
/// KLBN11.SA,2025-06-22,4.12
/// ```
///
/// Recomendado executar periodicamente via `radar-runner` ou agendador de sua escolha.
///
pub fn atualizar_cotacoes_csv(lista: &[String], caminho: &str) -> std::io::Result<()> {
    let hoje = Local::now().format("%Y-%m-%d").to_string();

    let mut linhas = vec![];

    for ativo in lista {
        if let Some(preco) = ultima_cotacao(&format!("{}.SA", ativo.to_uppercase())) {
            linhas.push(vec![ativo.to_string(), hoje.clone(), preco.to_string()]);
        }
    }

    if !linhas.is_empty() {
        let mut wtr = Writer::from_path(caminho)?;
        wtr.write_record(&["ativo", "data", "cotacao"])?;
        for linha in linhas {
            wtr.write_record(&linha)?;
        }
        wtr.flush()?;
    }

    Ok(())
}

/// Obtém a última cotação disponível para um ativo utilizando o intervalo "1d".
/// Retorna `None` em caso de erro durante a requisição ou parsing.
fn ultima_cotacao(ativo: &str) -> Option<f64> {
    let provider = yahoo::YahooConnector::new().ok()?;
    let response = tokio_test::block_on(provider.get_latest_quotes(ativo, "1d")).ok()?;
    let quote = response.last_quote().ok()?;
    Some(quote.close)
}

// Exemplo de uso:
//
// fn main() {
//	 let ativos = carregar_ativos_yaml("ativos.yaml").unwrap();
//	 let caminho = "cotacoes.csv";
//	 match atualizar_cotacoes_csv(&ativos, caminho) {
//		 Ok(_) => println!("Arquivo atualizado com sucesso!"),
//		 Err(e) => eprintln!("Erro ao atualizar cotacoes: {}", e),
//	 }
// }
