use scraper::{Html, Selector, node::Text};


const BC_DOLLAR_UR: &'static str =
    "https://ptax.bcb.gov.br/ptax_internet/consultarUltimaCotacaoDolar.do";

fn main() {
    match get_dollar_quotation_page() {
        Ok(page) => {
            let dollar_value_text = get_dollar_value(&page);
            let dollar_value: f64 = dollar_value_text.to_string().replace(',', ".").parse().unwrap();
            println!("dollar price:  R$ {:.1$}", dollar_value, 2);

        }
        Err(error) => eprintln!("{error}")
    }
}

fn get_dollar_value(html: &str) -> Text {
    let document = Html::parse_document(&html);
    let selector = Selector::parse(".fundoPadraoBClaro2 td[align=right]").unwrap();
    let element = document.select(&selector).next().unwrap().children().next().unwrap().value().as_text().unwrap();
    element.clone()
}


fn get_dollar_quotation_page() -> Result<String, String> {
    match reqwest::blocking::get(BC_DOLLAR_UR) {
        Ok(resp) => match resp.text() {
            Ok(value) => Ok(value),
            Err(_) => Err("erro, não foi possivel recuperar a cotação do dolar".into())
        },
        Err(err) => Err(format!("Error: {}", err))
    }
}