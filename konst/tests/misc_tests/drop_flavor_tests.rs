use konst::drop_flavor::{self, DropFlavor, MayDrop, NonDrop};

use std::mem::ManuallyDrop;

#[derive(Debug, PartialEq, Eq)]
struct Foo<D: DropFlavor, T>(D::Wrap<T>);

impl<D: DropFlavor, T> Foo<D, T> {
    const fn into_wrapper(self) -> D::Wrap<T> {
        konst::destructure! { Foo(x) = self }
        x
    }
}

#[test]
fn unwrap() {
    const fn unwrap_foo<D: DropFlavor, T>(foo: Foo<D, T>) -> T {
        drop_flavor::unwrap::<D, _>(foo.into_wrapper())
    }

    assert_eq!(unwrap_foo::<MayDrop, _>(Foo(10)), 10);
    assert_eq!(unwrap_foo::<NonDrop, _>(Foo(ManuallyDrop::new(10))), 10);
}

#[test]
fn as_inner() {
    const fn foo_as_inner<D: DropFlavor, T>(foo: &Foo<D, T>) -> &T {
        drop_flavor::as_inner::<D, _>(&foo.0)
    }

    assert_eq!(foo_as_inner::<MayDrop, _>(&Foo(10)), &10);
    assert_eq!(foo_as_inner::<NonDrop, _>(&Foo(ManuallyDrop::new(10))), &10);
}

#[test]
fn as_inner_mut() {
    const fn foo_as_inner_mut<D: DropFlavor, T>(foo: &mut Foo<D, T>) -> &mut T {
        drop_flavor::as_inner_mut::<D, _>(&mut foo.0)
    }

    assert_eq!(foo_as_inner_mut::<MayDrop, _>(&mut Foo(10)), &mut 10);
    assert_eq!(
        foo_as_inner_mut::<NonDrop, _>(&mut Foo(ManuallyDrop::new(10))),
        &mut 10
    );
}

#[test]
fn wrap() {
    const fn make_foo<D: DropFlavor, T>(val: T) -> Foo<D, T> {
        Foo(drop_flavor::wrap(val))
    }

    assert_eq!(make_foo::<MayDrop, _>(3), Foo(3));
    assert_eq!(make_foo::<NonDrop, _>(5), Foo(ManuallyDrop::new(5)));
}
