use scraper::{Html, Selector, node::Text};

const BC_DOLLAR_UR: &'static str =
    "https://ptax.bcb.gov.br/ptax_internet/consultarUltimaCotacaoDolar.do";

#[derive(Clone, Copy)]
struct Empty;

fn main() {
    let dollar_page = get_dollar_quotation_page();
    let dollar_value_text = get_dollar_value(&dollar_page);
    let dollar_value: f64 = dollar_value_text.to_string().replace(',', ".").parse().unwrap();
    println!("dollar price:  R$ {:.1$}", dollar_value, 2);
}

fn get_dollar_value(html: &str) -> Text {
    let document = Html::parse_document(&html);
    let selector = Selector::parse(".fundoPadraoBClaro2 td[align=right]").unwrap();
    let element = document.select(&selector).next().unwrap().children().next().unwrap().value().as_text().unwrap();
    element.clone()
}


fn get_dollar_quotation_page() -> String {
    match reqwest::blocking::get(BC_DOLLAR_UR) {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    }
}