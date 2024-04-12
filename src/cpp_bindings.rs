#[link(name="test", kind="static")]
#[link(name="sub", kind="static")]
extern "C"
{
    pub fn test_func() -> i32;
    pub fn test_char() -> libc::c_uchar;
}
