#![feature(test)]

extern crate test;

use sql_builder::prelude::*;

#[bench]
fn insert_string_format(bencher: &mut test::Bencher) {
    bencher.iter(|| {
			for i in 1..100 {
        format!("INSERT INTO books (title, price) VALUES ('In Search of Lost Time', {}), ('Don Quixote', 200);", test::black_box(i));
			}
    });
}

#[bench]
fn insert_string_concat(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for i in 1..100 {
            let mut sql =
                "INSERT INTO books (title, price) VALUES ('In Search of Lost Time', ".to_string();

            sql.push_str(&test::black_box(i).to_string());

            sql.push_str("), ('Don Quixote', 200);");
        }
    });
}

#[bench]
fn insert_builder(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for i in 1..100 {
            SqlBuilder::insert_into("books")
                .field("title")
                .field("price")
                .values(&[
                    quote("In Search of Lost Time"),
                    test::black_box(i).to_string(),
                ])
                .values(&["'Don Quixote', 200"])
                .sql()
                .unwrap();
        }
    });
}
