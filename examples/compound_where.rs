use sql_builder::{quote, where_builder::Where, SqlBuilder};

fn main() {
    let mut builder = SqlBuilder::select_from("table");
    builder.and_where_eq("field1", quote(""));

    builder.or_where(
        Where::new("field2")
            .eq(quote(""))
            .and(Where::new("field3").eq(quote("")))
            .in_brackets(),
    );

    let sql = builder.sql().unwrap();
    assert_eq!(
        &sql,
        r#"SELECT * FROM table WHERE field1 = '' OR ((field2 = '') AND (field3 = ''));"#
    );
    println!("{}", sql);
}
