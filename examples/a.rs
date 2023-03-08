use teitei::Converter;

pub trait X {
    type Item;
}

impl X for Foo {
    type Item = Foo;
}

#[derive(Debug)]
pub struct FooPartial {
    pub a: Option<u32>,
    pub b: Option<f32>,
}

#[derive(Converter, Debug)]
#[teitei(FooPartial)]
struct Foo {
    a: u32,
    b: f32,
}

fn main() {
    let foo_partial = FooPartial {
        a: Some(1),
        b: Some(1.2),
    };
    let foo = Foo::from_partial(foo_partial);
    dbg!(foo);
}
