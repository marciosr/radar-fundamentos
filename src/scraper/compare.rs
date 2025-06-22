use std::error::Error;
use tokio_test;
use yahoo_finance_api as yahoo;

use crate::scraper::busca::obter_numero_acoes;

/// Obtém o valor de mercado de um ativo usando a última cotação e o número de papais.
pub fn comparar_holdings(
	holding: &str,
	investida: &str,
	participacao_pct: f64,
) -> Result<(), Box<dyn Error>> {
	let conn = yahoo::YahooConnector::new()?;

	// Códigos da B3 precisam de ".SA"
	let ticker_holding = format!("{}.SA", holding.to_uppercase());
	let ticker_investida = format!("{}.SA", investida.to_uppercase());

	let cotacao_holding = tokio_test::block_on(conn.get_latest_quotes(&ticker_holding, "1d"))?;

	let cotacao_investida = tokio_test::block_on(conn.get_latest_quotes(&ticker_investida, "1d"))?;

	// Números de ações podem ser ajustados conforme fonte oficial ou parametrizados
	let acoes_holding = obter_numero_acoes(holding)
		.map_err(|e| format!("Erro ao buscar o número de papéis da holding: {}", e))?
		as f64;
	let acoes_investida = obter_numero_acoes(investida)
		.map_err(|e| format!("Erro ao buscar o número de papéis da investida: {}", e))?
		as f64;

	let valor_mercado_holding = cotacao_holding.last_quote()?.close * acoes_holding;
	let valor_mercado_investida = cotacao_investida.last_quote()?.close * acoes_investida;
	let valor_participacao = valor_mercado_investida * (participacao_pct / 100.0);

	println!(
		"{}: R$ {:.2} bilhões",
		holding.to_uppercase(),
		valor_mercado_holding / 1e9
	);
	println!(
		"{}: R$ {:.2} bilhões",
		investida.to_uppercase(),
		valor_mercado_investida / 1e9
	);
	println!(
		"Participação de {:.1}% = R$ {:.2} bilhões",
		participacao_pct,
		valor_participacao / 1e9
	);

	let diferenca = valor_participacao - valor_mercado_holding;
	println!("\n→ Diferença: R$ {:.2} bilhões", diferenca / 1e9);

	if diferenca > 0.0 {
		println!("→ A holding vale menos que sua participação na investida!");
	} else {
		println!("→ A holding está acima ou em linha com a investida.");
	}

	Ok(())
}
