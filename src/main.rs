#[link(name="test", kind="static")]

extern "C"
{
    fn test_func();
}

fn main()
{
    unsafe
    {
        test_func();
    };
}
