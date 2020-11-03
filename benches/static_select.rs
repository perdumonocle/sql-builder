#![feature(test)]

extern crate test;

use sql_builder::prelude::*;

#[bench]
fn static_select_string(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        "SELECT * FROM tests;".to_string();
    });
}

#[bench]
fn static_select_builder(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        SqlBuilder::select_from("tests").sql().unwrap();
    });
}
