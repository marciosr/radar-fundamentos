#[derive(serde::Deserialize, serde::Serialize)]
pub struct LinhaCSV {
	pub ticker: String,
	pub cotacao: Option<f32>,
	pub min_52_sem: Option<f32>,
	pub max_52_sem: Option<f32>,
	pub p_vp: Option<f32>,
	pub patrimonio_liquido: Option<f64>,
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

#[derive(serde::Deserialize, serde::Serialize)]
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
}