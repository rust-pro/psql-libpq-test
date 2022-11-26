use std::os::raw::c_int;

#[link(name = "libpq", kind = "dylib")]
extern "C" {
    fn PQisthreadsafe() -> c_int;
}

fn main() {
    println!("{}", unsafe { PQisthreadsafe() });
}
