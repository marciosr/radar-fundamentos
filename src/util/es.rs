use std::io::{self, Write};
use std::path::PathBuf;
use serde::Serialize;

#[allow(unused_imports)]
use crate::modelo::{Acao, Fundo, Ativo, DadosAcao, DadosFundo, LinhaCSV, LinhaCSVFundo};
use crate::scraper::atributo::Scraper;
use crate::scraper::{busca_acao, busca_fundo};
use crate::util::obter_html;

pub fn menu() {
	println!("Busca de ativos no Fundamentus. \n Informe o tipo de ativo desejado: \n 1 - Ação\n 2 - Fundo");
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

pub fn exportar_csv(tipo: &str, codigos: &[String], saida: Option<PathBuf>) {
	let mut resultados: Vec<Resultado> = vec![];
	
	let saida: PathBuf = saida.unwrap_or(PathBuf::from(format!("saida-{}.csv", tipo)));
	
	for codigo in codigos {
		match obter_html(codigo) {
			Ok(html) => {
				match tipo {
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
						return;
					}
				}
			}
			Err(e) => {
				eprintln!("Erro ao obter HTML para {}: {}", codigo, e);
				continue;
			}
		}
	}

	// Estabelece os nomes corretos dos arquivos.
/* 	let file_name = match resultados[0] {
		Resultado::Acao(_) => "resultado_acao",
		Resultado::Fundo(_) => "resultado_fundo",
	};
 */
	// Escreve o arquivo .json.
	if let Err(e) = std::fs::write(&saida, serde_json::to_string_pretty(&resultados).unwrap(), )
	{
		eprintln!("Erro ao salvar JSON: {}", e);
		return;
	}

	// Escrever CSV
	let mut wtr = csv::Writer::from_path(&saida).unwrap();
	for item in resultados {
		match item {
			Resultado::Acao(acao) => {
				let linha = LinhaCSV {
					ticker: acao.ativo.ticker,
					cotacao: acao.ativo.cotacao,
					min_52_sem: acao.ativo.min_52_sem,
					max_52_sem: acao.ativo.max_52_sem,
					p_vp: acao.ativo.p_vp,
					patrimonio_liquido: acao.ativo.patrimonio_liquido,
					num_acoes: acao.dados.num_acoes,
					preco_lucro: acao.dados.preco_lucro,
					lucro_por_acao: acao.dados.lucro_por_acao,
					marg_bruta: acao.dados.marg_bruta,
					marg_ebitda: acao.dados.marg_ebitda,
					roe: acao.dados.roe,
					roic: acao.dados.roic,
					divida_liquida: acao.dados.divida_liquida,
					liquidez_diaria: acao.dados.liquidez_diaria,
				};
				wtr.serialize(linha).unwrap();
			}
			Resultado::Fundo(fundo) => {
				let linha = LinhaCSVFundo {
					ticker: fundo.ativo.ticker,
					cotacao: fundo.ativo.cotacao,
					min_52_sem: fundo.ativo.min_52_sem,
					max_52_sem: fundo.ativo.max_52_sem,
					p_vp: fundo.ativo.p_vp,
					patrimonio_liquido: fundo.ativo.patrimonio_liquido,
					num_cotas: fundo.dados.num_cotas,
					segmento: fundo.dados.segmento,
					mandato: fundo.dados.mandato,
					rendimento_12m: fundo.dados.rendimento_12m,
					liquidez_diaria: fundo.dados.liquidez_diaria,
				};
				wtr.serialize(linha).unwrap();
			}

		}
	}
		
	wtr.flush().unwrap();
	println!("Exportação finalizada: {:?}", &saida);
}