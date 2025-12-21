use clap::{Parser, Subcommand, ValueHint};
// use dirs;
use radar_fundamentos::scraper::{
	atualizar_cotacoes_csv, compare::comparar_holdings, zscore_update,
};
use radar_fundamentos::scraper::{busca_acao, busca_fundo};
use radar_fundamentos::util::{exportar_csv, obter_html};
use std::{path::PathBuf, process::Command};

/// Radar Fundamentus: coleta e exporta múltiplos fundamentalistas
#[derive(Parser)]
#[clap(
	name = "radar-fundamentos",
	version = "0.1.0",
	author = "Márcio <marciosr10 at gmail.com>"
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

	/// Compara valor de mercado de uma holding com sua investida
	CompareHolding {
		ativo_holding: String,
		ativo_investida: String,
		#[arg(short, long)]
		participacao: f64,
	},

	/// Atualiza cotações locais e calcula o Z-score acumulado entre dois ativos
	Zscore {
		ativo_a: String,
		ativo_b: String,
		/// Data de início no formato YYYY-MM-DD
		#[clap(long)]
		inicio: Option<String>,
		/// Caminho do arquivo CSV de saída
		#[clap(long)]
		saida: Option<String>,
		#[clap(long)]
		plot: bool,
	},

	/// Atualiza o CSV com as últimas cotações a partir de uma lista de ativos
	Cotacoes {
		/// Lista de tickers cujas cotações devem ser atualizadas
		tickers: Vec<String>,

		/// Caminho de saída do arquivo CSV
		#[arg(long)]
		saida: String,
	},

	Indicadores {
		/// Tipo do ativo (fundos ou acoes)
		tipo: String,
		/// Lista de tickers cujas cotações devem ser atualizadas
		tickers: Vec<String>,

		#[arg(long)]
		/// Caminho de saida do arquivo CSV
		#[clap(short, long, value_hint = ValueHint::FilePath)]
		saida: Option<PathBuf>,
	},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Batch { tipo, tickers } => {
			for codigo in &tickers {
				match obter_html(codigo) {
					Ok(html) => match tipo.as_str() {
						"acoes" => busca_acao(codigo.to_string(), html),
						"fundos" => busca_fundo(codigo.to_string(), html),
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
			exportar_csv(&tipo, &tickers, saida)?;
		}

		Commands::Zscore {
			ativo_a,
			ativo_b,
			inicio,
			saida,
			plot,
		} => {
			// let mut ativos = vec![ativo_a.to_lowercase(), ativo_b.to_ascii_lowercase()];
			// ativos.sort();
			// let nome_csv = format!("zscore_{}_{}.csv", ativos[0], ativos[1]);
			// let nome_html = format!("zscore_{}_{}.csv", ativos[0], ativos[1]);

			// Dentro da função que processa o comando Zscore

			// 1. Pegamos os nomes e colocamos em ordem alfabética para ser agnóstico e consistente [cite: 2025-12-21]
			let mut ativos = vec![ativo_a.to_lowercase(), ativo_b.to_lowercase()];
			ativos.sort();

			// 2. Criamos o nome padrão baseado nos ativos (ex: zscore_bbse3_pssa3)
			let nome_padrao = format!("zscore_{}_{}", ativos[0], ativos[1]);

			// 3. Resolvemos o caminho do CSV usando o match no Option 'saida'
			let csv_final = match saida {
				Some(caminho) => caminho, // Se o usuário digitou --saida, usamos o que ele quer
				None => format!("{}.csv", nome_padrao), // Se não digitou, usamos o padrão automático
			};

			// 4. O HTML do gráfico segue a mesma lógica do nome padrão
			//let html_final = format!("{}.html", nome_padrao);

			match zscore_update::executar_zscore_update(
				&ativo_a,
				&ativo_b,
				inicio,
				Some(csv_final.as_str()),
			) {
				Ok(()) => {
					if plot {
						let mut cmd = Command::new("radar-plotter");
						cmd.arg("zscore");
						cmd.arg("--arquivo");
						cmd.arg(csv_final);

						println!("[plotter] [zscore] Comando completo: {:?}", cmd);
						match cmd.status() {
							Ok(status) => {
								println!("[plotter] [zscore] Finalizado com: {}", status)
							}
							Err(e) => eprintln!("[runner] [zscore] Erro ao executar: {}", e),
						}
					}
				}
				Err(e) => {
					eprintln!("Erro na geração do zscore: {e}");
				}
			}
		}
		Commands::CompareHolding {
			ativo_holding,
			ativo_investida,
			participacao,
		} => {
			let _ = comparar_holdings(&ativo_holding, &ativo_investida, participacao);
		}
		Commands::Cotacoes { tickers, saida } => match atualizar_cotacoes_csv(&tickers, &saida) {
			Ok(_) => println!("Arquivo de cotações atualizado com sucesso."),
			Err(e) => eprintln!("Erro ao salvar CSV: {}", e),
		},
		Commands::Indicadores {
			tickers,
			saida,
			tipo,
		} => exportar_csv(&tipo, &tickers, saida)?,
	}
	Ok(())
}
