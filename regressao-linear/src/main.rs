use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct RegistroPib {
    ano: f64,
    valor: f64,
}

fn calcular_media(valores: &[f64]) -> f64 {
    if valores.is_empty() {
        return 0.0;
    }
    let soma: f64 = valores.iter().sum::<f64>();
    let media: f64 = soma / valores.len() as f64;
    media
}

fn calcular_inclinacao(dados: &[RegistroPib]) -> f64 {
    if dados.len() < 2 {
        return 0.0;
    }
    let anos: Vec<f64> = dados.iter().map(|d| d.ano).collect();
    let valores: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let media_anos = calcular_media(&anos);
    let media_valores = calcular_media(&valores);

    let numerador: f64 = dados.iter()
        .map(|d| (d.ano - media_anos) * (d.valor - media_valores))
        .sum();
    let denominador: f64 = dados.iter()
        .map(|d| (d.ano - media_anos).powi(2))
        .sum();

    if denominador == 0.0 {
        return 0.0;
    }
    numerador / denominador
}

fn calcular_intercepto(dados: &[RegistroPib], inclinacao: f64) -> f64 {
    if dados.is_empty() {
        return 0.0;
    }
    let media_anos = calcular_media(&dados.iter().map(|d| d.ano).collect::<Vec<f64>>());
    let media_valores = calcular_media(&dados.iter().map(|d| d.valor).collect::<Vec<f64>>());
    media_valores - (inclinacao * media_anos)
}

fn prever_pib(intercepto: f64, inclinacao: f64, ano: f64) -> f64 {
    intercepto + (inclinacao * ano)
}

fn calcular_r_quadrado(dados: &[RegistroPib], intercepto: f64, inclinacao: f64) -> f64 {
    let valores_reais: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let valores_previstos: Vec<f64> = dados.iter().map(|d| prever_pib(intercepto, inclinacao, d.ano)).collect();
    let media_valores = calcular_media(&valores_reais);

    let ss_res: f64 = valores_reais.iter().zip(valores_previstos.iter())
        .map(|(&real, &previsto)| (real - previsto).powi(2))
        .sum();

    let ss_tot: f64 = valores_reais.iter()
        .map(|&real| (real - media_valores).powi(2))
        .sum();

    if ss_tot == 0.0 {
        if ss_res == 0.0 {
            1.0
        } else {
            0.0
        }
    } else {
        1.0 - (ss_res / ss_tot)
    }
}

fn calcular_erro_quadratico_medio(dados: &[RegistroPib], intercepto: f64, inclinacao: f64) -> f64 {
    let valores_reais: Vec<f64> = dados.iter().map(|d| d.valor).collect();
    let valores_previstos: Vec<f64> = dados.iter().map(|d| prever_pib(intercepto, inclinacao, d.ano)).collect();
    let n = valores_reais.len() as f64;

    let soma_quadrados_erro: f64 = valores_reais.iter().zip(valores_previstos.iter())
        .map(|(&real, &previsto)| (real - previsto).powi(2))
        .sum();

    soma_quadrados_erro / n
}

fn ler_dados_de_json(caminho_arquivo: &str) -> Result<Vec<RegistroPib>, Box<dyn std::error::Error>> {
    let mut arquivo = File::open(caminho_arquivo)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    let json: Value = serde_json::from_str(&conteudo)?;

    if let Some(pib_data) = json.get("PIB_per_capita_Brasil") {
        if let Some(pib_map) = pib_data.as_object() {
            let mut dados_pib: Vec<RegistroPib> = Vec::new();
            for (ano_str, valor) in pib_map {
                if let Ok(ano) = ano_str.parse::<f64>() {
                    if let Some(v) = valor.as_f64() {
                        dados_pib.push(RegistroPib { ano, valor: v });
                    }
                }
            }
            dados_pib.sort_by(|a, b| a.ano.partial_cmp(&b.ano).unwrap()); // Ordenar por ano
            Ok(dados_pib)
        } else {
            Err("Formato de 'PIB_per_capita_Brasil' inválido".into())
        }
    } else {
        Err("Chave 'PIB_per_capita_Brasil' não encontrada".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caminho_arquivo = "src/pib_brasil.json";

    let dados_pib_result = ler_dados_de_json(caminho_arquivo);

    match dados_pib_result {
        Ok(dados_pib) => {
            println!("Dados do PIB per capita do Brasil lidos com sucesso:");
            for registro in &dados_pib {
                println!("Ano: {}, PIB: {:.2}", registro.ano, registro.valor);
            }

            println!("\nAnalisando a tendência do PIB per capita:");
            let inclinacao = calcular_inclinacao(&dados_pib);
            let intercepto = calcular_intercepto(&dados_pib, inclinacao);
            let previsao_2028 = prever_pib(intercepto, inclinacao, 2028.0);
            let r_quadrado = calcular_r_quadrado(&dados_pib, intercepto, inclinacao);
            let mse = calcular_erro_quadratico_medio(&dados_pib, intercepto, inclinacao);

            println!("Inclinação (tendência anual): {:.2}", inclinacao);
            println!("Intercepto: {:.2}", intercepto);
            println!("Previsão do PIB per capita para 2028: {:.2}", previsao_2028);
            println!("Coeficiente de Determinação (R²): {:.2}", r_quadrado);
            println!("Erro Quadrático Médio (MSE): {:.2}", mse);
        }
        Err(erro) => {
            eprintln!("Erro ao ler o arquivo JSON: {}", erro);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regressao_linear_metricas() {
        // Dados com crescimento linear exato: y = 2x + 1
        let dados = vec![
            RegistroPib { ano: 1.0, valor: 3.0 },
            RegistroPib { ano: 2.0, valor: 5.0 },
            RegistroPib { ano: 3.0, valor: 7.0 },
            RegistroPib { ano: 4.0, valor: 9.0 },
            RegistroPib { ano: 5.0, valor: 11.0 },
        ];
    
        let inclinacao = calcular_inclinacao(&dados);
        let intercepto = calcular_intercepto(&dados, inclinacao);
        let r_quadrado = calcular_r_quadrado(&dados, intercepto, inclinacao);
        let mse = calcular_erro_quadratico_medio(&dados, intercepto, inclinacao);
    
        // Verificações
        assert!((inclinacao - 2.0).abs() < 1e-6, "Inclinação incorreta");
        assert!((intercepto - 1.0).abs() < 1e-6, "Intercepto incorreto");
        assert!((r_quadrado - 1.0).abs() < 1e-6, "R² deveria ser 1.0");
        assert!((mse).abs() < 1e-6, "MSE deveria ser 0");
    }

    #[test]
    fn test_regressao_e_previsao() {
        let dados = vec![
            RegistroPib { ano: 2000.0, valor: 10000.0 },
            RegistroPib { ano: 2005.0, valor: 15000.0 },
            RegistroPib { ano: 2010.0, valor: 20000.0 },
            RegistroPib { ano: 2015.0, valor: 25000.0 },
            RegistroPib { ano: 2020.0, valor: 30000.0 },
        ];
    
        let inclinacao = calcular_inclinacao(&dados);
        let intercepto = calcular_intercepto(&dados, inclinacao);
    
        let ano_previsto = 2025.0;
        let resultado = prever_pib(intercepto, inclinacao, ano_previsto);
    
        // O crescimento é de 1000 por ano, então em 2025 o PIB esperado é:
        let esperado = 35000.0;
    
        assert!(
            (resultado - esperado).abs() < 1e-6,
            "Esperado {}, mas obteve {}", esperado, resultado
        );
    }

}