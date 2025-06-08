pub trait Scraper<T> {
	fn extrair_dados(&self, html: &str) -> T;
}
