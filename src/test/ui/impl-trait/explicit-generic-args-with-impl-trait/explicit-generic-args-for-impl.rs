#![feature(explicit_generic_args_with_impl_trait)]

fn foo<T: ?Sized>(_f: impl AsRef<T>) {}

fn main() {
    foo::<str, String>("".to_string()); //~ ERROR E0107
}
