# Radar Fundamentos

**Aviso Legal:** Este programa não possui qualquer afiliação, parceria nem relação com plataformas de informações financeiras, corretoras ou serviços de análise de investimentos. Os dados utilizados e tratados provêm de fontes abertas, sendo responsabilidade do usuário validar a integridade e atualização dessas informações.

## Objetivo do Projeto

O Radar Fundamentos é uma ferramenta de uso pessoal, desenvolvida com o intuito de:

* Servir como experiência prática de aprendizado da linguagens de programação Rust;
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
````

O executável será gerado em `target/release/radar-fundamentos`.

### Compatibilidade com a compilação cruzada

```toml
[dependencies.openssl]
version = "0.10"

[target.'cfg(target_arch = "arm")'.dependencies.openssl]
version = "0.10"
features = ["vendored"]
```

-----

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

A saída será impressa no terminal em formato JSON.

-----

### Modo Exportação (Gera arquivos CSV de Indicadores)

```bash
./radar-fundamentos export <tipo> <TICKER1> <TICKER2> ... [--saida <caminho.csv>]
```

**Descrição:** Coleta os indicadores fundamentalistas para os *tickers* fornecidos e exporta os dados para um arquivo CSV.

Exemplo:

```bash
./radar-fundamentos export acoes klbn11 rani3 --saida acoes.csv
./radar-fundamentos export fundos snlg11 cvbi11
```

Se a *flag* `--saida` for omitida, o arquivo de saída padrão será `resultado.csv`.

-----

### 📊 Exportação de Indicadores Fundamentalistas

```bash
./radar-fundamentos indicadores <tipo> [lista de ativos] [--saida arquivo.csv]
```

**Descrição:** Este subcomando permite exportar **indicadores completos** para múltiplos ativos listados em um arquivo YAML, organizando os dados em formato CSV para análise offline.

#### Exemplo de uso:

```bash
./radar-fundamentos indicadores fundos --saida fundos.csv
```


O CSV gerado conterá diversas colunas de indicadores obtidos do site Fundamentus.

> ✅ **Dica**: ideal para gerar relatórios periódicos de acompanhamento de FIIs ou grupos de ações, de forma simples e reprodutível.

-----

### 💵 Atualização de Cotações (Alta Frequência)

```bash
./radar-fundamentos cotacoes <TICKER1> <TICKER2> ... --saida <caminho.csv>
```

**Descrição:** Coleta as últimas cotações (via Yahoo Finance) para todos os *tickers* fornecidos e **atualiza/sobrescreve** um único arquivo CSV de saída. Ideal para uso com o `radar-runner` em um ciclo de alta frequência.

> ❗ **Importante:** A *flag* `--saida` é **obrigatória**.

#### Exemplo de uso:

```bash
./radar-fundamentos cotacoes VALE3 PETR4 VGIR11 --saida cotacoes_atuais.csv
```

-----

### Cálculo de Z-score Histórico

```bash
./radar-fundamentos zscore <ATIVO_A> <ATIVO_B> --inicio <DATA-YYYY-MM-DD> [--saida caminho.csv]
```

**Descrição:** Utiliza dados históricos (via Yahoo Finance) a partir de uma data de início e calcula o Z-score acumulado do *spread* entre os dois ativos, exportando opcionalmente para um arquivo CSV.

Exemplo:

```bash
./radar-fundamentos zscore fras3 rapt4 --inicio 2023-01-01 --saida zscore.csv
```

-----

### 🔄 Atualização de Cotações e Cálculo de Z-score (Incremental)

```bash
./radar-fundamentos zscore-update <ATIVO_A> <ATIVO_B> [--saida caminho.csv]
```

**Descrição:** Realiza duas etapas:

1.  Atualiza os dados de cotação de cada ativo incrementalmente (mantendo arquivos locais).
2.  Calcula o Z-score acumulado com base nesse histórico atualizado.

#### ✅ Exemplo de uso:

```bash
./radar-fundamentos zscore-update fras3 rapt4 --saida z.csv
```

-----

### Comparação Patrimonial entre Holding e Investida

```bash
./radar-fundamentos compare-holding <HOLDING> <INVESTIDA> --participacao <PORCENTAGEM>
```

**Descrição:** Compara o valor de mercado da holding com o valor de sua participação acionária na empresa investida, indicando possíveis distorções.

Exemplo:

```bash
./radar-fundamentos compare-holding rapt4 fras3 --participacao 52
```

-----

### Tipos Aceitos

  * `acao`: para empresas listadas com dados fundamentalistas
  * `fundo`: para fundos imobiliários e fiagros
  * Fiinfras, FIP-IEs e Fidics listados não são suportados

-----

Para sugestões ou melhorias, fique à vontade para abrir uma *issue* ou *pull request*.

```

As principais alterações foram:
1.  Remoção da seção **"Execução Interativa"**.
2.  Ajuste do comando **`Cotacoes`** para aceitar `tickers` como lista e tornar `--saida` obrigatório, conforme o código.
3.  Revisão das descrições para refletir os 7 subcomandos ativos.

Se precisar de qualquer refinamento nas descrições, por favor, me avise!
```
