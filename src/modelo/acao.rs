use serde::{Serialize, Deserialize};
use regex::Regex;

use crate::modelo::Ativo;
use crate::scraper::atributo::Scraper;
use crate::parse_html_param;
use crate::util::parse_num;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Acao {
	pub ativo: Ativo,
	pub dados: DadosAcao,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DadosAcao {
	pub num_acoes: Option<u64>,
	pub preco_lucro: Option<f32>,
	pub lucro_por_acao: Option<f64>,
	pub marg_bruta: Option<f32>,
	pub marg_ebitda: Option<f32>,
	pub roe: Option<f32>,
	pub roic: Option<f32>,
	pub divida_liquida: Option<f64>,
	pub liquidez_diaria: Option<f64>,
}

impl Scraper<Acao> for Acao {
	fn extrair_dados(&self, html: &str) -> Acao {

		let ativo = Ativo {
			ticker: self.ativo.ticker.clone(),
			cotacao: parse_html_param!(html, "Cotação", f32),
			min_52_sem: parse_html_param!(html, "Min 52 sem", f32),
			max_52_sem: parse_html_param!(html, "Max 52 sem", f32),
			p_vp: parse_html_param!(html, "P/VP", f32),
			patrimonio_liquido: parse_html_param!(html, "Patrim. Líq", f64),
		};

		let dados = DadosAcao {
			num_acoes: parse_html_param!(html, "Nro. Ações", u64),
			preco_lucro: parse_html_param!(html, "P/L", f32),
			lucro_por_acao: parse_html_param!(html, "LPA", f64),
			marg_bruta: parse_html_param!(html, "Marg. Bruta", f32),
			marg_ebitda: parse_html_param!(html, "Marg. EBIT", f32),
			roe: parse_html_param!(html, "ROE", f32),
			roic: parse_html_param!(html, "ROIC", f32),
			divida_liquida: parse_html_param!(html, "Dív. Líquida", f64),
			liquidez_diaria: parse_html_param!(html, "Vol $ méd (2m)", f64),
		};

		Acao { ativo,dados }
	}
}