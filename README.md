# Radar Fundamentos

**Aviso Legal:** Este programa não possui qualquer afiliação, parceria nem relação com plataformas de informações financeiras, corretoras ou serviços de análise de investimentos. Os dados utilizados e tratados provêm de fontes abertas, sendo responsabilidade do usuário validar a integridade e atualização dessas informações.

## Objetivo do Projeto

O Radar Fundamentos é uma ferramenta de uso pessoal, desenvolvida com o intuito de:

* Servir como experiência prática de aprendizado da linguagem de programação Rust;
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

O executável será gerado em `target/release/radar-fundamentos`.

### Compatibilidade com a compilação cruzada

```toml
[dependencies.openssl]
version = "0.10"

[target.'cfg(target_arch = "arm")'.dependencies.openssl]
version = "0.10"
features = ["vendored"]
```

---

## Comandos Disponíveis

### Modo Batch (Consulta Rápida)

```bash
./radar-fundamentos batch <tipo> <TICKER1> <TICKER2> ...
```

**Descrição:** Processa uma lista de *tickers* e imprime o resultado da análise fundamentalista (Ação ou Fundo) no formato JSON na saída padrão (terminal).

Exemplo:

```bash
./radar-fundamentos batch acao petr4 vale3
./radar-fundamentos batch fundo ggrc11 relg11
```

---

### Modo Exportação (Gera arquivos CSV de Indicadores)

```bash
./radar-fundamentos export <tipo> <TICKER1> <TICKER2> ... [--saida <caminho.csv>]
```

**Descrição:** Coleta os indicadores fundamentalistas para os *tickers* fornecidos e exporta os dados para um arquivo CSV. Se a *flag* `--saida` for omitida, o arquivo de saída padrão será `resultado.csv`.

---

### 📊 Exportação de Indicadores Fundamentalistas (YAML)

```bash
./radar-fundamentos indicadores <tipo> [lista de ativos] [--saida arquivo.csv]
```

**Descrição:** Exporta indicadores completos para múltiplos ativos listados em um arquivo YAML externo. Ideal para relatórios periódicos de acompanhamento de carteiras.

---

### 💵 Atualização de Cotações (Alta Frequência)

```bash
./radar-fundamentos cotacoes <TICKER1> <TICKER2> ... --saida <caminho.csv>
```

**Descrição:** Coleta as últimas cotações (via Yahoo Finance) para os *tickers* fornecidos e atualiza um único arquivo CSV.

> ❗ **Importante:** A *flag* `--saida` é **obrigatória**.

---

### 📈 Cálculo de Z-score (Unificado e Incremental)

```bash
./radar-fundamentos zscore <ATIVO_A> <ATIVO_B> [--inicio YYYY-MM-DD] [--saida caminho.csv] [--plot]
```

**Descrição:** Comando consolidado que funciona com qualquer par de ativos [cite: 2025-12-21]. Ele realiza:

1. **Atualização Incremental:** Baixa apenas os novos registros de cotação para cada ativo.
2. **Cálculo Estatístico:** Calcula o Z-score acumulado do spread entre os dois ativos.
3. **Fallback de Data:** Se `--inicio` for omitido ou inválido, utiliza uma data padrão configurada.
4. **Nomeação Automática:** Se `--saida` for omitido, gera um arquivo baseado nos ativos em ordem alfabética (ex: `zscore_bbse3_pssa3.csv`) [cite: 2025-12-21].
5. **Gráfico:** Se `--plot` for usado, chama o `radar-plotter` para gerar um gráfico HTML.

**Exemplo:**

```bash
./radar-fundamentos zscore bbse3 pssa3 --inicio 2020-01-01 --plot
```

---

### Comparação Patrimonial entre Holding e Investida

```bash
./radar-fundamentos compare-holding <HOLDING> <INVESTIDA> --participacao <PORCENTAGEM>
```

**Descrição:** Compara o valor de mercado da holding com o valor de sua participação acionária na empresa investida.

---

### Tipos Aceitos

* `acao`: para empresas listadas com dados fundamentalistas.
* `fundo`: para fundos imobiliários e fiagros.

---

Para sugestões ou melhorias, fique à vontade para abrir uma *issue* ou *pull request*.
