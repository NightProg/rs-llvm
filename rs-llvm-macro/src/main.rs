use rs_llvm_macro::llvm_version;


#[llvm_version(4..16)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn main() {
}