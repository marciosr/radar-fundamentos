use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_num(s: &str) -> f64 {
	s.replace('.', "")
		.replace(',', ".")
		.replace('%', "") // Remove o símbolo de porcentagem
		.trim()
		.parse()
		.unwrap_or(0.0)
}

pub fn obter_html(codigo: &str) -> Result<String, Box<dyn Error>> {
	let url = format!(
		"https://www.fundamentus.com.br/detalhes.php?papel={}",
		codigo.to_uppercase()
	);

	let client = Client::builder()
		.user_agent(
			"Mozilla/5.0 (X11; Linux x86_64; rv:139.0) Gecko/20100101 Firefox/139.0; RadarFundamentos/0.1",
		)
		//.user_agent("Mozilla/5.0 (compatible; RadarFundamentos/0.1; +https://github.com/marciosr/radar-fundamentos)")
		.build()?;

	let resposta = client.get(&url).send()?.error_for_status()?;

	Ok(resposta.text()?)
}

pub fn extrair_rendimentos(html: &str) -> (Option<f32>, Option<f32>) {
	let document = Html::parse_document(html);
	let tr_selector = Selector::parse("tr").unwrap();
	let label_selector = Selector::parse("td.label.w2 span.txt").unwrap();
	let data_selector = Selector::parse("td.data.w2 span.txt").unwrap();

	for tr in document.select(&tr_selector) {
		let labels: Vec<_> = tr
			.select(&label_selector)
			.map(|e| e.text().collect::<String>().trim().to_string())
			.collect();

		let is_rend_distribuido = labels.iter().filter(|t| t == &"Rend. Distribuído").count();

		if is_rend_distribuido == 2 {
			let valores: Vec<_> = tr
				.select(&data_selector)
				.map(|e| {
					let txt = e
						.text()
						.collect::<String>()
						.replace(".", "")
						.replace(",", ".");
					txt.trim().parse::<f32>().ok()
				})
				.flatten()
				.collect();

			// Retorna os dois primeiros valores
			let rendimento_12m = valores.get(0).copied();
			let rendimento_3m = valores.get(1).copied();
			return (rendimento_12m, rendimento_3m);
		}
	}

	(None, None)
}
