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

O executável será gerado em `target/release/radar-fundamentus`

### Compatibilidade com a compilação cruzada

```toml
[dependencies.openssl]
version = "0.10"

[target.'cfg(target_arch = "arm")'.dependencies.openssl]
version = "0.10"
features = ["vendored"]
```

### Execução Interativa

Se nenhum parâmetro for passado, o programa entra no modo interativo:

```bash
./radar-fundamentus
```

### Modo Batch (Consulta Rápida)

```bash
./radar-fundamentus batch <tipo> <TICKER1> <TICKER2> ...
```

Exemplo:

```bash
./radar-fundamentus batch acao petr4 vale3
./radar-fundamentus batch fundo ggrc11 relg11
```

A saída será impressa no terminal em formato JSON.

### Modo Exportação (Gera arquivos)

```bash
./radar-fundamentus export <tipo> <TICKER1> <TICKER2> ...
```

Exemplo:

```bash
./radar-fundamentus export acao klbn11 rani3
./radar-fundamentus export fundo snlg11 cvbi11
```

Este modo gera dois arquivos:

* `resultado.json`: Resultado estruturado
* `resultado.csv`: Planilha pronta para análise

### Cálculo de Z-score Acumulado

```bash
./radar-fundamentus zscore <ATIVO_A> <ATIVO_B> --inicio <DATA-YYYY-MM-DD> [--saida caminho.csv]
```

Exemplo:

```bash
./radar-fundamentus zscore fras3 rapt4 --inicio 2023-01-01 --saida zscore.csv
```

Este comando utiliza dados históricos obtidos via Yahoo Finance e calcula o Z-score acumulado entre dois ativos, exportando opcionalmente para um arquivo CSV.

### Comparação Patrimonial entre Holding e Investida

```bash
./radar-fundamentus compare-holding <HOLDING> <INVESTIDA> --participacao <PORCENTAGEM>
```

Exemplo:

```bash
./radar-fundamentus compare-holding rapt4 fras3 --participacao 52
```

Este comando compara o valor de mercado da holding com sua participação acionária em uma empresa investida. Caso o valor da holding seja inferior à fatia que ela detém da investida, o programa indica uma possível distorção patrimonial.

### Tipos Aceitos

* `acao`: para empresas listadas com dados fundamentalistas
* `fundo`: para fundos imobiliários e fiagros
* Fiinfras, FIP-IEs e Fidics listados não são suportados

### 🔄 Atualização de Cotações e Cálculo de Z-score Acumulado

```bash
./radar-fundamentus zscore-update <ATIVO_A> <ATIVO_B> [--saida caminho.csv]
```

Esse subcomando realiza duas etapas integradas:

1. **Atualiza os dados de cotação** de cada ativo individualmente, salvando em arquivos locais (`dados/cotacoes/<ativo>.csv`)
2. **Calcula o Z-score acumulado** com base nesses dados históricos, imprimindo o último valor no terminal ou exportando para CSV

#### ✅ Exemplo de uso:

```bash
./radar-fundamentus zscore-update fras3 rapt4 --saida z.csv
```

→ Gera um arquivo `z.csv` com a seguinte estrutura:

```csv
data,preco_a,preco_b,spread,media,desvio,zscore
2023-01-02,12.34,10.11,2.23,2.23,0.00,0.00
2023-01-03,12.40,10.10,2.30,2.26,0.05,0.89
...
```

#### 💡 Observações:

* Os arquivos locais com as cotações são mantidos e atualizados incrementalmente.
* Ideal para uso recorrente com pares definidos de ativos, mantendo o histórico sem redundância.

### 📥 Atualização de Cotações com Entrada YAML

```bash
./radar-fundamentus cotacoes --yaml ativos.yaml --saida cotacoes.csv
```

Este comando percorre a lista de ativos definida no arquivo YAML e gera um arquivo CSV com as cotações atuais.

#### Exemplo de `ativos.yaml`:

```yaml
ativos:
  - PETR4
  - KLBN11
  - FRAS3
```

> ❗ **Importante:** os tickers devem ser informados **sem o sufixo ".SA"**. O programa adiciona esse sufixo automaticamente ao consultar o Yahoo Finance. Assim, use apenas os códigos da B3 como aparecem normalmente (ex: `PETR4`, `WEGE3`, `HGLG11`).

O resultado `cotacoes.csv` poderá ser importado diretamente no LibreOffice ou Excel.

---

Para sugestões ou melhorias, fique à vontade para abrir uma *issue* ou *pull request*.
