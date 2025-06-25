pub mod atributo;
pub mod busca;
pub mod compare;
pub mod ultima_cotacao_yahoo;
pub mod zscore;
pub mod zscore_update;

pub use busca::{busca_acao, busca_fundo};
pub use compare::comparar_holdings;
pub use ultima_cotacao_yahoo::{atualizar_cotacoes_csv, carregar_ativos_yaml};
pub use zscore::{
    ZscoreRegistro, busca_zscore, calcular_zscore_acumulado_com_quotes, obter_cotacoes_yahoo,
    salvar_zscore_completo,
};
pub use zscore_update::executar_zscore_update;
