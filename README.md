# Radar Fundamentos

**Aviso Legal:** Este programa não possui qualquer afiliação, parceria nem relação com plataformas de informações financeiras, corretoras ou serviços de análise de investimentos. Os dados utilizados e tratados provêm de fontes abertas, sendo responsabilidade do usuário validar a integridade e atualização dessas informações.

## Objetivo do Projeto

O Radar Fundamentos é uma ferramenta de uso pessoal, desenvolvida com o intuito de:

* Servir como experiência prática de aprendizado das linguagens de programação Rust e afins;
* Explorar aplicações diretas na análise fundamentalista de ações e fundos listados na B3;
* Automatizar a coleta, estruturação e exportação de indicadores fundamentalistas para uso offline.

## Licença

Este projeto é licenciado sob a Licença MIT. Consulte o arquivo `LICENSE-MIT` para mais detalhes.

## Isenção de Garantias

Este software é fornecido "no estado em que se encontra", sem garantias de qualquer tipo, expressas ou implícitas. O uso é de inteira responsabilidade do usuário.

---

## Manual de Utilização

### Compilação

```bash
cargo build --release
```

O executável será gerado em `target/release/radar-fundamentos`

### Execução Interativa

Se nenhum parâmetro for passado, o programa entra no modo interativo:

```bash
./radar-fundamentos
```

### Modo Batch (Consulta Rápida)

```bash
./radar-fundamentos batch <tipo> <TICKER1> <TICKER2> ...
```

Exemplo:

```bash
./radar-fundamentos batch acao petr4 vale3
./radar-fundamentos batch fundo ggrc11 relg11
```

A saída será impressa no terminal em formato JSON.

### Modo Exportação (Gera arquivos)

```bash
./radar-fundamentos export <tipo> <TICKER1> <TICKER2> ...
```

Exemplo:

```bash
./radar-fundamentos export acao klbn11 rani3
./radar-fundamentos export fundo snlg11 cvbi11
```

Este modo gera dois arquivos:

* `resultado.json`: Resultado estruturado
* `resultado.csv`: Planilha pronta para análise

### Cálculo de Z-score Acumulado

```bash
./radar-fundamentos zscore <ATIVO_A> <ATIVO_B> --inicio <DATA-YYYY-MM-DD> [--saida caminho.csv]
```

Exemplo:

```bash
./radar-fundamentos z-score fras3 rapt4 --inicio 2023-01-01 --saida zscore.csv
```

Este comando utiliza dados históricos obtidos via Yahoo Finance e calcula o Z-score acumulado entre dois ativos, exportando opcionalmente para um arquivo CSV.

### Comparação Patrimonial entre Holding e Investida

```bash
./radar-fundamentos compare-holding <HOLDING> <INVESTIDA> --participacao <PORCENTAGEM>
```

Exemplo:

```bash
./radar-fundamentos compare-holding rapt4 fras3 --participacao 52
```

Este comando compara o valor de mercado da holding com sua participação acionária em uma empresa investida. Caso o valor da holding seja inferior à fatia que ela detém da investida, o programa indica uma possível distorção patrimonial.

### Tipos Aceitos

* `acao`: para empresas listadas com dados fundamentalistas
* `fundo`: para fundos imobiliários e fiagros
* Fiinfras, FIP-IEs e Fidics listados não são suportados

---

Para sugestões ou melhorias, fique à vontade para abrir uma *issue* ou *pull request*.
