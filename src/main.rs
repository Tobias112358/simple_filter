use nih_plug::prelude::*;

use simple_filter::SimpleFilter;

fn main() {
    nih_export_standalone::<SimpleFilter>();
}