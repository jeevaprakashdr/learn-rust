mod generic_lifetime_annotation;
mod struct_lifetime_annotation;
mod lifetime_elision;

fn main() {

    generic_lifetime_annotation::demo();
    struct_lifetime_annotation::demo();
    lifetime_elision::demo();
}
