fn main() {
    let mut _mutable_integer = 7_i32;

    {
        // use immut shadow the mut  variable
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 6;
    }

    _mutable_integer = 3;
}
