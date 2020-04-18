use crate::{
    afd_parser, ffw_parser, lsr_parser, sel_parser, svr_parser, svs_parser, swo_parser, tor_parser,
};
use domain::{Event, Product};
use std::panic;

/**
 * Determines which product gets which parser.
 * NOTE: We're catching panics here - not ideal, but processing threads can't die.
 */
pub fn parse(product: &Product) -> Option<Event> {
    let result = panic::catch_unwind(|| match product.product_code.as_ref() {
        "AFD" => afd_parser::parse(&product),
        "FFW" => ffw_parser::parse(&product),
        "LSR" => lsr_parser::parse(&product),
        "SEL" => sel_parser::parse(&product),
        "SVR" => svr_parser::parse(&product),
        "SVS" => svs_parser::parse(&product),
        "SWO" => swo_parser::parse(&product),
        "TOR" => tor_parser::parse(&product),
        _ => {
            error!("unknown product code: {}", &product.product_code);
            None
        }
    });

    result.unwrap_or_else(|_| {
        error!("recovered from panic on product: {}", product.id);
        None
    })
}
