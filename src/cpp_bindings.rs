#[link(name="test", kind="static")]
#[link(name="sub", kind="static")]


extern "C"
{
    /// tests i32 binding
    pub fn test_func() -> i32;
    
    /// tests char binding
    pub fn test_char() -> libc::c_uchar;
}
