pub mod prelude {


#[link(name = "impls")]
extern "C" {
    pub fn add(a :usize, b: usize) -> usize;
}

}