#![feature(test)]

extern crate test;

use sql_builder::prelude::*;

#[bench]
fn join_select_string(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        "SELECT books.title, shops.total FROM books JOIN shops ON books.id = shops.book;"
            .to_string();
    });
}

#[bench]
fn join_select_builder(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        SqlBuilder::select_from("books")
            .inner()
            .join("shops")
            .on("books.id = shops.book")
            .field("books.title")
            .field("shops.total")
            .sql()
            .unwrap();
    });
}
