use yahoo_finance_api as yahoo;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tokio_test;
//#[cfg(not(feature = "blocking"))]
pub fn busca_zscore(
    ativo_a: &str,
    ativo_b: &str,
    inicio: Option<&str>,
    saida: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let ticker_a = format!("{}.SA", ativo_a.to_uppercase());
    let ticker_b = format!("{}.SA", ativo_b.to_uppercase());

    // Data de início
    let start = if let Some(data) = inicio {
        OffsetDateTime::parse(&format!("{data}T00:00:00Z"), &Rfc3339)?
    } else {
        OffsetDateTime::UNIX_EPOCH
    };
    let end = OffsetDateTime::now_utc();

    let conn = yahoo::YahooConnector::new()?;
    let hist1 = tokio_test::block_on(conn.get_quote_history(&ticker_a, start, end)).unwrap();
    let hist2 = tokio_test::block_on(conn.get_quote_history(&ticker_b, start, end)).unwrap();

    let quotes1 = hist1.quotes().unwrap();
    let quotes2 = hist2.quotes().unwrap();

    if quotes1.len() != quotes2.len() {
        return Err("As séries históricas têm tamanhos diferentes".into());
    }

    let mut spreads = Vec::new();
    let mut linhas = Vec::new();

    for (q1, q2) in quotes1.iter().zip(quotes2.iter()) {
        let spread = q1.close - q2.close;
        spreads.push(spread);

        let media = spreads.iter().copied().sum::<f64>() / spreads.len() as f64;
        let desvio = (spreads.iter().map(|x| (x - media).powi(2)).sum::<f64>()
            / spreads.len() as f64)
            .sqrt();
        let zscore = if desvio != 0.0 {
            (spread - media) / desvio
        } else {
            0.0
        };

        let data = OffsetDateTime::from_unix_timestamp(q1.timestamp)?;
        linhas.push(format!(
            "{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}",
            data.date(),
            q1.close,
            q2.close,
            spread,
            media,
            desvio,
            zscore
        ));
    }

    if let Some(caminho) = saida {
        let mut arquivo = File::create(caminho)?;
        writeln!(arquivo, "data,preco_a,preco_b,spread,media,desvio,zscore")?;
        for linha in linhas {
            writeln!(arquivo, "{}", linha)?;
        }
        println!("Z-score exportado para: {}", caminho);
    } else {
        for linha in linhas {
            println!("{}", linha);
        }
    }
    Ok(())
}
