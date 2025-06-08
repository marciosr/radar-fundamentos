use serde_json::to_string_pretty;

use crate::modelo::{ Acao, Fundo };
use crate::scraper::atributo::Scraper;

pub fn busca_acao (codigo: String, response: String) {
	let mut scraper = Acao::default();
	scraper.ativo.ticker = codigo.clone();

	let resultado = scraper.extrair_dados(&response);
	
	println!("{}", to_string_pretty(&resultado).unwrap());
}

pub fn busca_fundo (codigo: String, response: String) {
	let mut scraper = Fundo::default();
	scraper.ativo.ticker = codigo.clone();
	
	let resultado = scraper.extrair_dados(&response);
	
	println!("{}", to_string_pretty(&resultado).unwrap());
}