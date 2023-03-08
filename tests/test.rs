#[test]
fn a() {
    use teitei::Converter;

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

    let foo_partial = FooPartial {
        a: Some(1),
        b: Some(1.2),
    };
    let foo = Foo::from_partial(foo_partial);
    dbg!(foo);

}
