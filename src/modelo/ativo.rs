use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, Default)] // Diretiva Default simplifica a inicialização posterior dos dados.
pub struct Ativo {
	pub ticker: String,
	pub cotacao: Option<f32>,
	pub min_52_sem: Option<f32>,
	pub max_52_sem: Option<f32>,
	pub p_vp: Option<f32>,
	pub patrimonio_liquido: Option<f64>,
}
