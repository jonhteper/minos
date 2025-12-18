const MINOS_FILE_REFERENCE: &str = include_str!("../../assets/v0_17.minos");

#[test]
fn v0_17_works() {
    crate::MinosParser::easy_parse_str(MINOS_FILE_REFERENCE)
        .expect("Error al decodificar archivo MINOS");
}
