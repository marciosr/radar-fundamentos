use reqwest::blocking::Client;
use std::error::Error;

pub fn parse_num(s: &str) -> f64 {
	s.replace('.', "")
	 .replace(',', ".")
	 .replace('%', "") // Remove o símbolo de porcentagem
	 .trim()
	 .parse()
	 .unwrap_or(0.0)
}

pub fn obter_html(codigo: &str) -> Result<String, Box<dyn std::error::Error>> {
	let url = format!("https://www.fundamentus.com.br/detalhes.php?papel={}", codigo.to_uppercase());

	let client = reqwest::blocking::Client::builder()
		.user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:139.0) Gecko/20100101 Firefox/139.0; RadarFundamentos/0.1")
		//.user_agent("Mozilla/5.0 (compatible; RadarFundamentos/0.1; +https://github.com/marciosr/radar-fundamentos)")
		.build()?;

	let resposta = client.get(&url)
		.send()?
		.error_for_status()?;

	Ok(resposta.text()?)
}


