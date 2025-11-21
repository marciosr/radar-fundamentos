use crate::modelo::Acao;
use crate::modelo::Fundo;
use serde::Serialize;

/// Estrutura plana para exportação de dados de Ação para CSV.
#[derive(Debug, Serialize)]
pub struct LinhaCSV {
	pub ticker: String,
	pub cotacao: Option<f32>,
	pub min_52_sem: Option<f32>,
	pub max_52_sem: Option<f32>,
	pub p_vp: Option<f32>,
	pub patrimonio_liquido: Option<f64>,
	pub num_acoes: Option<u64>,
	pub preco_lucro: Option<f32>,
	pub lucro_por_acao: Option<f32>,
	pub marg_bruta: Option<f32>,
	pub marg_ebitda: Option<f32>,
	pub roe: Option<f32>,
	pub roic: Option<f32>,
	pub divida_liquida: Option<f64>,
	pub liquidez_diaria: Option<f64>,
}

impl LinhaCSV {
	/// Retorna o cabeçalho CSV para Ações.
	pub fn headers() -> Vec<&'static str> {
		vec![
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
		]
	}
}

impl Acao {
	/// Converte a estrutura Acao para a estrutura plana LinhaCSV
	pub fn to_csv_line(&self) -> LinhaCSV {
		LinhaCSV {
			ticker: self.ativo.ticker.clone(),
			cotacao: self.ativo.cotacao,
			min_52_sem: self.ativo.min_52_sem,
			max_52_sem: self.ativo.max_52_sem,
			p_vp: self.ativo.p_vp,
			patrimonio_liquido: self.ativo.patrimonio_liquido,
			// Usa num_acoes para Ações.
			num_acoes: self.dados.num_acoes,
			preco_lucro: self.dados.preco_lucro,
			lucro_por_acao: self.dados.lucro_por_acao,
			marg_bruta: self.dados.margem_bruta,
			marg_ebitda: self.dados.margem_liquida,
			roe: self.dados.roe,
			roic: self.dados.roic,
			divida_liquida: self.dados.divida_liquida_patrim,
			liquidez_diaria: self.dados.liquidez_diaria,
		}
	}
}

/// Estrutura plana para exportação de dados de Fundo Imobiliário para CSV.
#[derive(Debug, Serialize)]
pub struct LinhaCSVFundo {
	pub ticker: String,
	pub cotacao: Option<f32>,
	pub min_52_sem: Option<f32>,
	pub max_52_sem: Option<f32>,
	pub p_vp: Option<f32>,
	pub patrimonio_liquido: Option<f64>,
	pub num_cotas: Option<u64>,
	pub segmento: Option<String>,
	pub mandato: Option<String>,
	pub rendimento_12m: Option<f32>,
	pub liquidez_diaria: Option<f64>,
	pub rendimento_03m: Option<f32>,
}

impl LinhaCSVFundo {
	/// Retorna o cabeçalho CSV para Fundos Imobiliários (FIIs).
	pub fn headers() -> Vec<&'static str> {
		vec![
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
		]
	}
}

impl Fundo {
	/// Converte a estrutura Fundo para a estrutura plana LinhaCSVFundo
	pub fn to_csv_line(&self) -> LinhaCSVFundo {
		LinhaCSVFundo {
			ticker: self.ativo.ticker.clone(),
			cotacao: self.ativo.cotacao,
			min_52_sem: self.ativo.min_52_sem,
			max_52_sem: self.ativo.max_52_sem,
			p_vp: self.ativo.p_vp,
			patrimonio_liquido: self.ativo.patrimonio_liquido,
			num_cotas: self.dados.num_cotas,
			segmento: self.dados.segmento.clone(),
			mandato: self.dados.mandato.clone(),
			rendimento_12m: self.dados.rendimento_12m,
			liquidez_diaria: self.dados.liquidez_diaria,
			rendimento_03m: self.dados.rendimento_03m,
		}
	}
}
