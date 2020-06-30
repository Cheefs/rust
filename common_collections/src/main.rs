// pub mod vector;
pub mod string;

fn main() {
    // vector::declare();
    // vector::fill();
    // vector::get_value();
    // vector::lost_control_error();
    // vector::with_enums();

    string::declare();
    string::concat();
    // string::error();
    string::get_chars_for_bytes();
    string::get_chars("Hello");
    string::get_chars("Привет");
    string::get_chars("नमस्ते");
}
