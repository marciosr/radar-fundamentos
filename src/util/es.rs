use anyhow::{Context, Result, bail};
use serde::Serialize;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

#[allow(unused_imports)]
use crate::modelo::{Acao, Ativo, DadosAcao, DadosFundo, Fundo, LinhaCSV, LinhaCSVFundo};
use crate::scraper::atributo::Scraper;
use crate::scraper::{busca_acao, busca_fundo};
use crate::util::obter_html;

pub struct CsvOptions<'a> {
	pub header: Option<&'a [&'a str]>,
	pub delimiter: Option<u8>,
	pub fail_if_empty: bool,
}

impl<'a> Default for CsvOptions<'a> {
	fn default() -> Self {
		Self {
			header: None,
			delimiter: None,
			fail_if_empty: true,
		}
	}
}

pub fn export_csv_atomico<T, I>(saida: &Path, rows: I, opts: CsvOptions) -> Result<()>
where
	T: serde::Serialize,
	I: IntoIterator<Item = T>,
{
	use std::fs;

	// Materializa para contar e permitir duas passadas se necessário
	let rows: Vec<T> = rows.into_iter().collect();
	if opts.fail_if_empty && rows.is_empty() {
		bail!("vetor de dados vazio; nada a exportar");
	}

	if let Some(dir) = saida.parent() {
		fs::create_dir_all(dir).with_context(|| format!("criando diretório {}", dir.display()))?;
	}

	let parent = saida.parent().unwrap_or_else(|| Path::new("."));
	let mut tmp = NamedTempFile::new_in(parent).context("criando arquivo temporário")?;

	{
		let mut wtr = csv::WriterBuilder::new()
			.delimiter(opts.delimiter.unwrap_or(b','))
			.from_writer(tmp.as_file_mut());

		if let Some(h) = opts.header {
			//use csv::Writer; // habilita write_record
			wtr.write_record(h).context("escrevendo cabeçalho")?;
		}

		for row in rows {
			wtr.serialize(row).context("serializando linha")?;
		}
		wtr.flush().context("flush no writer")?;
	}

	tmp.as_file().sync_all().context("fsync do temporário")?;
	tmp.persist(saida)
		.with_context(|| format!("persistindo {}", saida.display()))?;
	Ok(())
}

pub fn menu() {
	println!(
		"Busca de ativos no Fundamentus. \n Informe o tipo de ativo desejado: \n 1 - Ação\n 2 - Fundo"
	);
	io::stdout().flush().unwrap(); // força flush para printar antes da leitura

	let mut tipo = String::new();
	io::stdin().read_line(&mut tipo).unwrap();

	let tipo_i8 = match tipo.trim().parse::<i32>() {
		Ok(num) => num,
		Err(_) => {
			println!("Entrada inválida. Por favor, digite 1 ou 2.");
			return;
		}
	};

	println!("\nInforme o código do ativo: ");
	io::stdout().flush().unwrap(); // força flush para printar antes da leitura
	let mut codigo = String::new();
	io::stdin().read_line(&mut codigo).unwrap();

	match obter_html(&codigo) {
		Ok(html) => {
			// prossegue com o parsing
			match tipo_i8 {
				1 => busca_acao(codigo, html),
				2 => busca_fundo(codigo, html),
				_ => println!("Tipo inválido. Informe: \n 1 - Ação\n 2 - Fundo"),
			}
		}
		Err(e) => {
			eprintln!("Erro ao obter HTML para {}: {}", codigo, e);
		}
	};
}

#[derive(Debug, Serialize)]
pub enum Resultado {
	Acao(Acao),
	Fundo(Fundo),
}

pub fn exportar_csv(tipo: &str, codigos: &[String], saida: Option<PathBuf>) -> Result<()> {
	let mut resultados: Vec<Resultado> = vec![];

	for codigo in codigos {
		match obter_html(codigo) {
			Ok(html) => match tipo {
				"acao" => {
					let mut scraper = Acao::default();
					scraper.ativo.ticker = codigo.clone();

					let resultado = scraper.extrair_dados(&html);
					resultados.push(Resultado::Acao(resultado));
				}
				"fundo" => {
					let mut scraper = Fundo::default();
					scraper.ativo.ticker = codigo.clone();

					let resultado = scraper.extrair_dados(&html);
					resultados.push(Resultado::Fundo(resultado));
				}
				_ => {
					eprintln!("Outros tipos de ativos não implementados'{}'", tipo);
					return Ok(());
				}
			},
			Err(e) => {
				eprintln!("Erro ao obter HTML para {}: {}", codigo, e);
				continue;
			}
		}
	}

	match tipo {
		"acao" => {
			export_csv_atomico(
				&saida.unwrap(),
				resultados,
				CsvOptions {
					header: Some(&[
						"ticker",
						"cotacao",
						"min_52_sem",
						"max_52_sem",
						"p_vp",
						"patrimonio_liquido",
						"num_acoes",
						"preco_lucro",
						"lucro_por_acao",
						"marg_bruta",
						"marg_ebitda",
						"roe",
						"roic",
						"divida_liquida",
						"liquidez_diaria",
					]),
					..Default::default()
				},
			)?;
		}
		"fundo" => {
			export_csv_atomico(
				&saida.unwrap(),
				resultados,
				CsvOptions {
					header: Some(&[
						"ticker",
						"cotacao",
						"min_52_sem",
						"max_52_sem",
						"p_vp",
						"patrimonio_liquido",
						"num_cotas",
						"segmento",
						"mandato",
						"rendimento_12m",
						"liquidez_diaria",
						"rendimento_03m",
					]),
					..Default::default()
				},
			)?;
		}
		_ => {
			eprintln!("Outros tipos de ativos não implementados'{}'", tipo);
			return Ok(());
		}
	}

	Ok(())
}
