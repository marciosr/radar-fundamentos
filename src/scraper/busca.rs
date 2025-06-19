use serde_json::to_string_pretty;
use std::error::Error;

use crate::modelo::{Acao, Fundo};
use crate::scraper::atributo::Scraper;
use crate::util::obter_html;

pub fn busca_acao(codigo: String, response: String) {
	let mut scraper = Acao::default();
	scraper.ativo.ticker = codigo.clone();

	let resultado = scraper.extrair_dados(&response);

	println!("{}", to_string_pretty(&resultado).unwrap());
}

pub fn busca_fundo(codigo: String, response: String) {
	let mut scraper = Fundo::default();
	scraper.ativo.ticker = codigo.clone();

	let resultado = scraper.extrair_dados(&response);

	println!("{}", to_string_pretty(&resultado).unwrap());
}

pub fn obter_numero_acoes(codigo: &str) -> Result<u64, Box<dyn Error>> {
	let scraper = Acao::default();
	let html = obter_html(codigo)?;
	let acao = scraper.extrair_dados(&html);
	acao.dados
		.num_acoes
		.ok_or_else(|| format!("Número de ações não encontrado para o ativo {}", codigo).into())
}
