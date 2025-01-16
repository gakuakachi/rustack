mod parser;
mod value;

fn main() {
    for raw_line in std::io::stdin().lines().flatten() {
        parser::parse(&raw_line);
    }
}
