use regex::{Regex, RegexBuilder};

pub struct Regexes {
    pub movement: Regex,
    pub poly_condensed: Regex,
    pub source: Regex,
    pub valid: Regex,
    pub affected: Regex,
    pub probability: Regex,
    pub wfos: Regex,
    pub md_number: Regex,
    pub watch_id: Regex,
    pub poly: Regex,
    pub warning_for: Regex,
    pub watch_for: Regex,
}

impl Regexes {
    pub fn new() -> Regexes {
        let movement_pattern = r"\ntime...mot...loc\s(?P<time>\d{4}z)\s(?P<deg>\d+)\D{3}\s(?P<kt>\d+)kt\s(?P<lat>\d{4})\s(?P<lon>\d{4,5})";
        let source_pattern = r"\n{2}\s{2}source...(?P<src>[\s|\S]*?)\.";
        let valid_pattern = r"(\d{6}t\d{4}z)-(\d{6}t\d{4}z)";
        let affected_pattern = r"Areas affected\.{3}([\S|\s]*?)\n\n";
        let probability_pattern = r"Probability of Watch Issuance...(\d{1,3}) percent";
        let wfos_pattern = r"ATTN...WFO...([\s|\S]*?)\n\n";
        let poly_pattern = r"(\d{4}\s\d{4,5})+";
        let poly_condensed_pattern = r"(\d{8})\s";
        let md_number_pattern = r"Mesoscale Discussion (\d{4})";
        let watch_id_pattern = r"Watch Number (\d{1,3})";
        let warning_for_pattern = r"Warning for...([\s|\S]+?)\n\n";
        let watch_for_pattern = r"Watch for portions of\s\n([\s|\S]+?)\n\n";

        Regexes {
            movement: RegexBuilder::new(movement_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            poly_condensed: RegexBuilder::new(poly_condensed_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            source: RegexBuilder::new(source_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            valid: RegexBuilder::new(valid_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            affected: RegexBuilder::new(affected_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            probability: RegexBuilder::new(probability_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            wfos: RegexBuilder::new(wfos_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            poly: RegexBuilder::new(poly_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            md_number: RegexBuilder::new(md_number_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            watch_id: RegexBuilder::new(watch_id_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            warning_for: RegexBuilder::new(warning_for_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
            watch_for: RegexBuilder::new(watch_for_pattern)
                .case_insensitive(true)
                .build()
                .unwrap(),
        }
    }
}
