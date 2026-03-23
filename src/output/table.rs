use super::Tableable;

pub fn render<T: Tableable>(data: &T) {
    println!("{}", data.to_table());
}
