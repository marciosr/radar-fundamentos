use yahoo_finance_api as yahoo;
use std::error::Error;

/// Obtém o valor de mercado de um ativo usando a última cotação e uma estimativa de número de ações.
pub async fn comparar_holdings(
	holding: &str,
	investida: &str,
	participacao_pct: f64,
) -> Result<(), Box<dyn Error>> {
	let conn = yahoo::YahooConnector::new()?;

	// Códigos da B3 precisam de ".SA"
	let ticker_holding = format!("{}.SA", holding.to_uppercase());
	let ticker_investida = format!("{}.SA", investida.to_uppercase());

	let cotacao_holding = conn.get_latest_quotes(&ticker_holding, "1d").await?.last_quote()?.close;

	let cotacao_investida = conn.get_latest_quotes(&ticker_investida, "1d").await?.last_quote()?.close;

	// Números de ações podem ser ajustados conforme fonte oficial ou parametrizados
	let acoes_holding = estimar_numero_acoes(holding)?;
	let acoes_investida = estimar_numero_acoes(investida)?;

	let valor_mercado_holding = cotacao_holding * acoes_holding;
	let valor_mercado_investida = cotacao_investida * acoes_investida;
	let valor_participacao = valor_mercado_investida * (participacao_pct / 100.0);

	println!("{}: R$ {:.2} bilhões", holding.to_uppercase(), valor_mercado_holding / 1e9);
	println!("{}: R$ {:.2} bilhões", investida.to_uppercase(), valor_mercado_investida / 1e9);
	println!("Participação de {:.1}% = R$ {:.2} bilhões", participacao_pct, valor_participacao / 1e9);

	let diferenca = valor_participacao - valor_mercado_holding;
	println!("\n→ Diferença: R$ {:.2} bilhões", diferenca / 1e9);

	if diferenca > 0.0 {
		println!("→ A holding vale menos que sua participação na investida!");
	} else {
		println!("→ A holding está acima ou em linha com a investida.");
	}

	Ok(())
}

/// Estimativa simplificada. Idealmente, isso viria de um arquivo ou fonte oficial.
fn estimar_numero_acoes(ticker: &str) -> Result<f64, Box<dyn Error>> {
	match ticker.to_lowercase().as_str() {
		"rapt4" => Ok(329_330_533.0), // exemplo real
		"fras3" => Ok(270_016_343.0),
		_ => Err(format!("Número de ações não conhecido para {}", ticker).into()),
	}
}
