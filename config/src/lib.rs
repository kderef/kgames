mod conf;

pub use conf::*;

const HEX_CODE_LEN: usize = "#123456".len();

pub fn valid_hex_color(color: impl AsRef<str>) -> bool {
    let color = color.as_ref();
    if color.len() != HEX_CODE_LEN {
        return false;
    }
    false
}
