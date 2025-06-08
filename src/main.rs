use std::env;
use radar_fundamentos::util::{ menu, exportar_csv, obter_html };
use radar_fundamentos::scraper::{ busca_acao, busca_fundo };

fn main() {
	let args: Vec<String> = env::args().collect();

/* 	if args.len() == 1 {
		menu(); // modo interativo
		return;
	}

	if args[1] == "export" {
		if args.len() < 4 {
			eprintln!("Uso: script export <tipo> <TICKER1> <TICKER2> ...");
			return;
		}
		let tipo = &args[2];
		let codigos = &args[3..];
		exportar_csv(tipo, codigos);
		return;
	}

	if args[1] == "batch" {
		if args.len() < 4 {
			eprintln!("Uso: script batch <tipo> <TICKER1> <TICKER2> ...");
			return;
		}

		let tipo = &args[2];
		let codigos = &args[3..];

		for codigo in codigos {

			match obter_html(codigo) {
				Ok(html) => {
					// prossegue com o parsing
					match tipo.as_str() {
						"acao" => busca_acao(codigo.to_string(), html),
						"fundo" => busca_fundo(codigo.to_string(), html),
						_ => eprintln!("Tipo inválido: {}", tipo),
					}
				}
				Err(e) => {
					eprintln!("Erro ao obter HTML para {}: {}", codigo, e);
					continue; // ignora e passa para o próximo
				}
			};
		}

	return;
	} */
    match args.get(1).map(String::as_str) {
        None => menu(),
        Some("export") => {
            if args.len() < 4 {
                eprintln!("Uso: script export <tipo> <TICKER1> <TICKER2> ...");
                return;
            }
            let tipo = &args[2];
            let codigos = &args[3..];
            exportar_csv(tipo, codigos);
            return;
        }
        Some("batch") => {
            if args.len() < 4 {
                eprintln!("Uso: script batch <tipo> <TICKER1> <TICKER2> ...");
                return;
            }

            let tipo = &args[2];
            let codigos = &args[3..];

            for codigo in codigos {

                match obter_html(codigo) {
                    Ok(html) => {
                        // prossegue com o parsing
                        match tipo.as_str() {
                            "acao" => busca_acao(codigo.to_string(), html),
                            "fundo" => busca_fundo(codigo.to_string(), html),
                            _ => eprintln!("Tipo inválido: {}", tipo),
                        }
                    }
                    Err(e) => {
                        eprintln!("Erro ao obter HTML para {}: {}", codigo, e);
                        continue; // ignora e passa para o próximo
                    }
                };
            }

        return;
        }
        Some(other) => eprintln!("Comando desconhecido: {}", other),
    }
}
