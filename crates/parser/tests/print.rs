use parser::{parse, print_schema};

const EXAMPLE: &str = include_str!("../../../examples/awesome.schema");

#[test]
fn print_schema_round_trips_example() {
    let parsed = parse(EXAMPLE).expect("parse");
    let printed = print_schema(&parsed);
    let again = parse(&printed).expect("re-parse");
    assert_eq!(parsed.models.len(), again.models.len());
    assert_eq!(parsed.edges.len(), again.edges.len());
    assert_eq!(parsed.datasource.provider, again.datasource.provider);
}
