extern "C" {
    fn call_bar();
}

pub fn call_bar_rust() {
    unsafe {
        call_bar();
    }
}
