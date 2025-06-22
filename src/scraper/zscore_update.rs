use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use time::OffsetDateTime;

use chrono::{Duration, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::scraper::{
	ZscoreRegistro, calcular_zscore_acumulado_com_quotes, obter_cotacoes_yahoo,
	salvar_zscore_completo,
};
use yahoo_finance_api::Quote;

const DIR_COTACOES: &str = "dados/cotacoes/";
const DATA_MINIMA_PADRAO: &str = "2014-01-01";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RegistroCotacao {
	data: NaiveDate,
	preco_fechamento: f64,
}

pub fn executar_zscore_update(
	atv_a: &str,
	atv_b: &str,
	caminho_saida: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
	fs::create_dir_all(DIR_COTACOES)?;

	let quotes_a = atualizar_e_carregar_quotes(atv_a)?;
	let quotes_b = atualizar_e_carregar_quotes(atv_b)?;

	let resultado: Vec<ZscoreRegistro> =
		calcular_zscore_acumulado_com_quotes(&quotes_a, &quotes_b)?;

	if let Some(arquivo) = caminho_saida {
		salvar_zscore_completo(&resultado, arquivo)?;
		println!("[ZscoreUpdate] Resultado salvo em {arquivo}");
	} else {
		if let Some(ultimo) = resultado.last() {
			println!(
				"[ZscoreUpdate] Último valor: {} | Z = {:.2} (spread {:.2})",
				ultimo.data, ultimo.zscore, ultimo.spread
			);
		}
	}

	Ok(())
}

fn atualizar_e_carregar_quotes(ticker: &str) -> Result<Vec<Quote>, Box<dyn std::error::Error>> {
	let caminho = format!("{DIR_COTACOES}{ticker}.csv");

	let mut historico: Vec<RegistroCotacao> = if Path::new(&caminho).exists() {
		ler_csv(&caminho)?
	} else {
		Vec::new()
	};

	let ultima_data = historico.last().map(|r| r.data);
	let nova_data_inicio = ultima_data.map(|d| d + Duration::days(1));

	let data_str: Option<String> = Some(
		nova_data_inicio
			.map(|d| d.to_string())
			.unwrap_or_else(|| DATA_MINIMA_PADRAO.to_string()),
	);

	let novos_dados = obter_cotacoes_yahoo(ticker, data_str.as_deref())?;

	if novos_dados.is_empty() {
		return Err(format!(
			"Sem dados encontrados para {ticker} desde {}",
			data_str.unwrap_or_default()
		)
		.into());
	}

	let total_novos = novos_dados.len();

	if total_novos > 0 {
		for quote in &novos_dados {
			let data = NaiveDate::from_ymd_opt(
				OffsetDateTime::from_unix_timestamp(quote.timestamp)?.year(),
				OffsetDateTime::from_unix_timestamp(quote.timestamp)?.month() as u32,
				OffsetDateTime::from_unix_timestamp(quote.timestamp)?.day() as u32,
			)
			.unwrap();
			historico.push(RegistroCotacao {
				data,
				preco_fechamento: quote.close,
			});
		}
		historico.sort_by_key(|r| r.data);
		salvar_cotacoes(&historico, &caminho)?;
		println!("[ZscoreUpdate] Cotação de {ticker} atualizada ({total_novos} novos registros)");
	} else {
		println!("[ZscoreUpdate] Cotação de {ticker} já está atualizada");
	}

	// Reconstrói os quotes (já que Quote contém timestamp)
	let quotes_reconstruidos: Vec<Quote> = historico
		.iter()
		.map(|r| Quote {
			timestamp: r.data.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(),
			open: 0.0,
			high: 0.0,
			low: 0.0,
			close: r.preco_fechamento,
			volume: 0,
			adjclose: r.preco_fechamento,
		})
		.collect();

	Ok(quotes_reconstruidos)
}

fn ler_csv(caminho: &str) -> Result<Vec<RegistroCotacao>, Box<dyn std::error::Error>> {
	let reader = BufReader::new(File::open(caminho)?);
	let mut rdr = csv::Reader::from_reader(reader);
	let mut dados = Vec::new();

	for result in rdr.deserialize() {
		let r: RegistroCotacao = result?;
		dados.push(r);
	}

	Ok(dados)
}

fn salvar_cotacoes(
	dados: &[RegistroCotacao],
	caminho: &str,
) -> Result<(), Box<dyn std::error::Error>> {
	let writer = BufWriter::new(File::create(caminho)?);
	let mut wtr = csv::Writer::from_writer(writer);

	for d in dados {
		wtr.serialize(d)?;
	}

	wtr.flush()?;
	Ok(())
}
