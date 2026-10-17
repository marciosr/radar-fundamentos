use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, UPGRADE_INSECURE_REQUESTS};
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;

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

	let mut headers = HeaderMap::new();
	headers.insert(
		ACCEPT,
		HeaderValue::from_static(
			"text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
		),
	);
	headers.insert(
		ACCEPT_LANGUAGE,
		HeaderValue::from_static("pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7"),
	);
	headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_static("1"));

	let client = Client::builder()
		.timeout(Duration::from_secs(15))
		.user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:139.0) Gecko/20100101 Firefox/139.0")
		.default_headers(headers)
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
