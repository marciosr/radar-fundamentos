use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::modelo::Ativo;
use crate::parse_html_param;
use crate::scraper::atributo::Scraper;
use crate::util::{extrair_rendimentos, parse_num};

#[derive(Debug, Serialize, Deserialize, Default)] // Diretiva Default simplifica a inicialização posterior dos dados.
pub struct Fundo {
	pub ativo: Ativo,
	pub dados: DadosFundo,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DadosFundo {
	pub num_cotas: Option<u64>,
	pub segmento: Option<String>,
	pub mandato: Option<String>,
	pub rendimento_12m: Option<f32>,
	pub liquidez_diaria: Option<f64>,
	pub rendimento_03m: Option<f32>,
}

impl Scraper<Fundo> for Fundo {
	fn extrair_dados(&self, html: &str) -> Fundo {
		let ativo = Ativo {
			ticker: self.ativo.ticker.clone(),
			cotacao: parse_html_param!(html, "Cotação", f32),
			min_52_sem: parse_html_param!(html, "Min 52 sem", f32),
			max_52_sem: parse_html_param!(html, "Max 52 sem", f32),
			p_vp: parse_html_param!(html, "P/VP", f32),
			patrimonio_liquido: parse_html_param!(html, "Patrim Líquido", f64),
		};

		let (rendimento_12m, rendimento_03m) = extrair_rendimentos(html);

		let dados = DadosFundo {
			num_cotas: parse_html_param!(html, "Nro. Cotas", u64),
			segmento: parse_html_param!(html = html, param = "Segmento", string_with_link),
			mandato: parse_html_param!(html = html, param = "Mandato", String),
			rendimento_12m,
			liquidez_diaria: parse_html_param!(html, "Vol $ méd (2m)", f64),
			rendimento_03m,
		};
		Fundo { ativo, dados }
	}
}
