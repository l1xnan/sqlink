use sqlparser::dialect::DuckDbDialect;
use sqlparser::parser::{Parser, ParserOptions};
fn main() {
    let dialect = DuckDbDialect {}; // or AnsiDialect

    let parser = Parser::new(&dialect).with_options(ParserOptions {
        trailing_commas: true,
        unescape: true,
    });
    let sql = r#"
    SELECT a, b, 123, myfunc(b),
    FROM table_1
    WHERE a > b AND b < 100
    GROUP BY ALL
    ORDER BY a DESC, {{ b }}
    "#;

    let ast = parser
        .try_with_sql(sql)
        .unwrap()
        .parse_statements()
        .unwrap();

    println!("AST: {:?}", ast);

    let serialized = serde_json::to_string_pretty(&ast).unwrap();
    println!("Serialized as JSON:\n{serialized}");
}
