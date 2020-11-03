#![feature(test)]

extern crate test;

use sql_builder::prelude::*;

#[bench]
fn simple_select_string_format(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for i in 1..100 {
            format!("SELECT price FROM books WHERE id = {}", test::black_box(i));
        }
    });
}

#[bench]
fn simple_select_string_concat(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for i in 1..100 {
            let mut sql = "SELECT price FROM books WHERE id = ".to_owned();
            sql.push_str(&test::black_box(i).to_string());
            sql.push(';');
        }
    });
}

#[bench]
fn simple_select_builder(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for i in 1..100 {
            SqlBuilder::select_from("books")
                .field("price")
                .and_where_eq("id", test::black_box(i))
                .sql()
                .unwrap();
        }
    });
}
