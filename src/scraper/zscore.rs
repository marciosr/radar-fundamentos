use std::error::Error;
use std::fs::File;
use std::io::Write;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use yahoo_finance_api::{self as yahoo, Quote};

// Função pública: busca_zscore (mantida como interface legada)
pub fn busca_zscore(
	ativo_a: &str,
	ativo_b: &str,
	inicio: Option<&str>,
	saida: Option<&str>,
) -> Result<(), Box<dyn Error>> {
	let quotes1 = obter_cotacoes_yahoo(&ativo_a, inicio)?;
	let quotes2 = obter_cotacoes_yahoo(&ativo_b, inicio)?;

	let resultado = calcular_zscore_acumulado_com_quotes(&quotes1, &quotes2)?;

	if let Some(caminho) = saida {
		salvar_zscore_completo(&resultado, caminho)?;
		println!("Z-score exportado para: {}", caminho);
	} else {
		for linha in &resultado {
			println!(
				"{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
				linha.data,
				linha.preco_a,
				linha.preco_b,
				linha.spread,
				linha.media,
				linha.desvio,
				linha.zscore
			);
		}
	}

	Ok(())
}

// Estrutura de dados par ao registro de z-score
#[derive(Debug)]
pub struct ZscoreRegistro {
	pub data: String,
	pub preco_a: f64,
	pub preco_b: f64,
	pub spread: f64,
	pub media: f64,
	pub desvio: f64,
	pub zscore: f64,
}

pub fn calcular_zscore_acumulado_com_quotes(
	quotes1: &[Quote],
	quotes2: &[Quote],
) -> Result<Vec<ZscoreRegistro>, Box<dyn Error>> {
	if quotes1.len() != quotes2.len() {
		return Err("As séries históricas têm tamanhos diferentes".into());
	}

	let mut resultado = Vec::new();
	let mut spreads = Vec::new();

	for (q1, q2) in quotes1.iter().zip(quotes2.iter()) {
		let spread = q1.close - q2.close;
		spreads.push(spread);

		let media = spreads.iter().copied().sum::<f64>() / spreads.len() as f64;
		let desvio = (spreads.iter().map(|x| (x - media).powi(2)).sum::<f64>()
			/ spreads.len() as f64)
			.sqrt();
		let zscore = if desvio != 0.0 {
			(spread - media) / desvio
		} else {
			0.0
		};

		let data = OffsetDateTime::from_unix_timestamp(q1.timestamp)?
			.date()
			.to_string();

		resultado.push(ZscoreRegistro {
			data,
			preco_a: q1.close,
			preco_b: q2.close,
			spread,
			media,
			desvio,
			zscore,
		});
	}

	Ok(resultado)
}

// Salvar resultado completo
pub fn salvar_zscore_completo(
	dados: &[ZscoreRegistro],
	caminho: &str,
) -> Result<(), Box<dyn Error>> {
	let mut arquivo = File::create(caminho)?;
	writeln!(arquivo, "data,preco_a,preco_b,spread,media,desvio,zscore")?;
	for r in dados {
		writeln!(
			arquivo,
			"{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
			r.data, r.preco_a, r.preco_b, r.spread, r.media, r.desvio, r.zscore
		)?;
	}
	Ok(())
}

// Função para obter cotações do Yahoo (reutilizada pelo zscore-update)
pub fn obter_cotacoes_yahoo(
	codigo: &str,
	data_inicio: Option<&str>,
) -> Result<Vec<Quote>, Box<dyn Error>> {
	let codigo = format!("{}.SA", codigo.to_uppercase());

	let inicio = if let Some(data) = data_inicio {
		OffsetDateTime::parse(&format!("{data}T00:00:00Z"), &Rfc3339)?
	} else {
		OffsetDateTime::UNIX_EPOCH
	};

	let data_fim = OffsetDateTime::now_utc();
	let conn = yahoo::YahooConnector::new()?;
	let historico = tokio_test::block_on(conn.get_quote_history(&codigo, inicio, data_fim))?;

	Ok(historico.quotes()?)
}
