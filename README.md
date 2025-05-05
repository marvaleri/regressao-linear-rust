# 📈 Análise e Previsão do PIB per Capita com Regressão Linear (Rust)

Este projeto tem como objetivo ler dados históricos do PIB per capita do Brasil a partir de um arquivo JSON e, com base nesses dados, aplicar **regressão linear** para prever valores futuros, como o PIB de um ano específico (ex: 2028). Também são calculadas métricas estatísticas como **R²** e **Erro Quadrático Médio (MSE)**.

---

## 🚀 Tecnologias utilizadas

- 🦀 `Rust`
- 📦 `Serde` (para serialização e deserialização de JSON)
- 📦 `Serde_json`

---

## 📈 Funcionalidades Principais

### `calcular_media(valores: &[f64]) -> f64`

Calcula a média aritmética de um vetor de valores `f64`.

---

### `calcular_inclinacao(dados: &[RegistroPib]) -> f64`

Aplica a fórmula de regressão linear para encontrar a **inclinação (a)** da reta

---

### `calcular_intercepto(dados: &[RegistroPib], inclinacao: f64) -> f64`

Calcula o **intercepto (b)** da reta com base na média dos anos e dos valores de PIB

---

### `prever_pib(intercepto: f64, inclinacao: f64, ano: f64) -> f64`

Utiliza a equação da reta para prever o PIB de um determinado ano

---

### `calcular_r_quadrado(...) -> f64`

Calcula o **coeficiente de determinação (R²)**, que mede a qualidade da regressão:

- **1.0** = ajuste perfeito
- **0.0** = nenhum ajuste

---

### `calcular_erro_quadratico_medio(...) -> f64`

Calcula o **Erro Quadrático Médio (MSE)** entre os valores reais e os previstos, útil para avaliar a precisão.

