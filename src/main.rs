use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

use radar_fundamentos::scraper::{
	atualizar_cotacoes_csv, carregar_ativos_yaml, compare::comparar_holdings, zscore, zscore_update,
};
use radar_fundamentos::scraper::{busca_acao, busca_fundo};
use radar_fundamentos::util::{exportar_csv, obter_html};

/// Radar Fundamentus: coleta e exporta múltiplos fundamentalistas
#[derive(Parser)]
#[clap(
	name = "radar-fundamentos",
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

	/// Compara valor de mercado de uma holding com sua investida
	CompareHolding {
		ativo_holding: String,
		ativo_investida: String,
		#[arg(short, long)]
		participacao: f64,
	},

	/// Atualiza cotações locais e calcula o Z-score acumulado entre dois ativos
	ZScoreUpdate {
		ativo_a: String,
		ativo_b: String,
		/// Caminho do arquivo CSV de saída
		#[clap(long)]
		saida: Option<String>,
	},
	/// Atualiza o CSV com as últimas cotações a partir de um arquivo YAML
	Cotacoes {
		/// Caminho para o arquivo YAML contendo os ativos
		#[arg(long)]
		yaml: String,

		/// Caminho de saída do arquivo CSV
		#[arg(long)]
		saida: String,
	},
	Indicadores {
		/// Tipo do ativo (fundo ou acao)
		tipo: String,
		/// Caminho para o arquivo YAML contenndo os ativos
		#[arg(long)]
		yaml: String,
		#[arg(long)]
		/// Caminho de saida do arquivo CSV
		#[clap(short, long, value_hint = ValueHint::FilePath)]
		saida: Option<PathBuf>,
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
		Commands::ZScoreUpdate {
			ativo_a,
			ativo_b,
			saida,
		} => {
			if let Err(e) =
				zscore_update::executar_zscore_update(&ativo_a, &ativo_b, saida.as_deref())
			{
				eprintln!("Erro no ZScoreUpdate: {e}");
			}
		}
		Commands::Cotacoes { yaml, saida } => match carregar_ativos_yaml(&yaml) {
			Ok(ativos) => match atualizar_cotacoes_csv(&ativos, &saida) {
				Ok(_) => println!("Arquivo de cotações atualizado com sucesso."),
				Err(e) => eprintln!("Erro ao salvar CSV: {}", e),
			},
			Err(e) => eprintln!("Erro ao carregar ativos do YAML: {}", e),
		},
		Commands::Indicadores { yaml, saida, tipo } => match carregar_ativos_yaml(&yaml) {
			Ok(ativos) => exportar_csv(&tipo, &ativos, saida),
			Err(e) => eprintln!("Erro ao carregar ativos do YAML: {}", e),
		},
	}
}
