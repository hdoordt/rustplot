extern crate rustplot;

use rustplot::chart_builder;
use rustplot::chart_builder::Chart;
use rustplot::data_parser;

#[test]
fn histogram_tests() {
    let data_1 = data_parser::get_num_col(1, 0, 1000, "./resources/histogram_tests.csv");
    let histogram_1 =
        chart_builder::Histogram::new(String::from("Test Histogram 1"), data_1.clone());
    histogram_1.draw();
}
