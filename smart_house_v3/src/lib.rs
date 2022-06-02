pub mod device;
pub mod edifice;
pub mod report;

pub fn print_report<T: report::Report>(report_item: T) {
    println!("{}", report_item.get_report());
}