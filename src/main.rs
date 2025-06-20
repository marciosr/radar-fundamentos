use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;
//use tokio::runtime::Runtime;

use radar_fundamentos::scraper::{busca_acao, busca_fundo};
use radar_fundamentos::scraper::{compare::comparar_holdings, zscore};
use radar_fundamentos::util::{exportar_csv, obter_html};

/// Radar Fundamentus: coleta e exporta múltiplos fundamentalistas
#[derive(Parser)]
#[clap(
	name = "radar-fundamentus",
	version = "0.1.0",
	author = "Seu Nome <seu-email>"
)]
struct Cli {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Processa vários tickers e escreve JSON na saída padrão
	Batch {
		/// Tipo de ativo: "acao" ou "fundo"
		tipo: String,
		/// Lista de tickers a processar
		tickers: Vec<String>,
	},

	/// Exporta dados para CSV chamando exportar_csv(tipo, codigos)
	Export {
		/// Tipo de ativo: "acao" ou "fundo"
		tipo: String,
		/// Lista de tickers a exportar
		tickers: Vec<String>,
		/// Caminho de saída para o CSV (opcional)
		#[clap(short, long, value_hint = ValueHint::FilePath)]
		saida: Option<PathBuf>,
	},

	/// Calcula o Z-score acumulado entre dois ativos com dados do Yahoo Finance
	ZScore {
		ativo_a: String,
		ativo_b: String,
		/// Data de início no formato YYYY-MM-DD
		#[clap(long)]
		inicio: Option<String>,
		/// Caminho do arquivo CSV de saída
		#[clap(long)]
		saida: Option<String>,
	},
	CompareHolding {
		ativo_holding: String,
		ativo_investida: String,
		#[arg(short, long)]
		participacao: f64,
	},
}

fn main() {
	let cli = Cli::parse();

	match cli.command {
		Commands::Batch { tipo, tickers } => {
			for codigo in &tickers {
				match obter_html(codigo) {
					Ok(html) => match tipo.as_str() {
						"acao" => busca_acao(codigo.to_string(), html),
						"fundo" => busca_fundo(codigo.to_string(), html),
						_ => eprintln!("Tipo inválido: {}", tipo),
					},
					Err(e) => eprintln!("Erro ao obter HTML para {}: {}", codigo, e),
				}
			}
		}

		Commands::Export {
			tipo,
			tickers,
			saida,
		} => {
			exportar_csv(&tipo, &tickers, saida);
		}

		Commands::ZScore {
			ativo_a,
			ativo_b,
			inicio,
			saida,
		} => {
			if let Err(e) =
				zscore::busca_zscore(&ativo_a, &ativo_b, inicio.as_deref(), saida.as_deref())
			{
				eprintln!("Erro ao calcular Z-score: {e}");
			}
		}

		Commands::CompareHolding {
			ativo_holding,
			ativo_investida,
			participacao,
		} => {
			let _ = comparar_holdings(&ativo_holding, &ativo_investida, participacao);
		}
	}
}
