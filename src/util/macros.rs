// Na definição das macros não é necessário importar os módulos que são utilizados,
// apenas no escopo onde a macro é expandida deve-se fazer a chamada dos módulos.

#[macro_export]
macro_rules! parse_html_param {
	// Caso especial: String com link interno (ex: Segmento)
	(html = $html:expr, param = $param:expr, string_with_link) => {{
		let pattern = format!(
			r#"<span class="txt">\s*{}\s*</span>\s*</td>\s*<td class="data.*?">\s*<span class="txt">\s*<a[^>]*>(.*?)</a>"#,
			regex::escape($param)
		);
		Regex::new(&pattern)
			.unwrap()
			.captures($html)
			.map(|cap| cap[1].trim().to_string())
	}};

	// Caso geral: String simples (sem <a>)
	(html = $html:expr, param = $param:expr, String) => {{
		let pattern = format!(
			r#"<span class="txt">\s*{}\s*</span>\s*</td>\s*<td class="data.*?">\s*<span class="txt">\s*(.*?)\s*</span>"#,
			regex::escape($param)
		);
		Regex::new(&pattern)
			.unwrap()
			.captures($html)
			.map(|cap| cap[1].trim().to_string())
	}};

	// Caso padrão para tipos numéricos
	($html:expr, $param:expr, $tipo:ty) => {{
		let pattern = format!(
			r#"<span class="txt">\s*{}\s*</span>\s*</td>\s*<td class="data.*?">.*?>\s*([\d.,%]+)\s*<"#,
			regex::escape($param)
		);
		Regex::new(&pattern)
			.unwrap()
			.captures($html)
			.and_then(|cap| Some(parse_num(&cap[1]) as $tipo))
	}};
}

