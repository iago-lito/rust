// edition:2018
// ignore-compare-mode-chalk

#![feature(type_alias_impl_trait)]

pub trait Bar {
    type E: Copy;

    fn foo<T>() -> Self::E;
}

impl<S> Bar for S {
    type E = impl std::marker::Copy;
    //~^ ERROR the trait bound `impl Future: Copy` is not satisfied [E0277]
    fn foo<T>() -> Self::E {
        //~^ ERROR type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
        async {}
    }
}

fn main() {}
