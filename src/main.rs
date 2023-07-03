use std::{env::args, process};

use scraper::{node::Text, Html, Selector};

const BC_DOLLAR_UR: &str = "https://ptax.bcb.gov.br/ptax_internet/consultarUltimaCotacaoDolar.do";

fn main() {
    let mut args = args();
    let executable_name = args.next().unwrap();
    let value = if let Some(argument_value) = args.next() {
        if let Ok(value) = argument_value.parse::<f64>() {
            value
        } else {
            print_usage(&executable_name);
            process::exit(1);
        }
    } else {
        1f64
    };
    let dollar_value: f64 = match get_dollar_quotation_page() {
        Ok(page) => {
            let dollar_value_text = get_dollar_value(&page);
            dollar_value_text
                .to_string()
                .replace(',', '.')
                .parse()
                .unwrap()
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };
    println!("R$ {:.1$}", dollar_value * value, 2);
}

fn print_usage(executable_name: &str) {
    eprintln!("modo de uso:
    {executable_name} <valor numerico> - calcula o valor do dolar para reais");
}

fn get_dollar_value(html: &str) -> Text {
    let document = Html::parse_document(html);
    let selector = Selector::parse(".fundoPadraoBClaro2 td[align=right]").unwrap();
    let element = document
        .select(&selector)
        .next()
        .unwrap()
        .children()
        .next()
        .unwrap()
        .value()
        .as_text()
        .unwrap();
    element.clone()
}

fn get_dollar_quotation_page() -> Result<String, String> {
    match reqwest::blocking::get(BC_DOLLAR_UR) {
        Ok(resp) => {
            if let Ok(value) = resp.text() {
                Ok(value)
            } else {
                Err("erro, não foi possivel recuperar a cotação do dolar".into())
            }
        }
        Err(err) => Err(format!("Error: {}", err)),
    }
}
