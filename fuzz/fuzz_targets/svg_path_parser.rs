#![no_main]

use animato_path::{CompoundPath, SvgPathParser};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = core::str::from_utf8(data) {
        let _ = SvgPathParser::try_parse(input);
        let _ = SvgPathParser::parse(input);
        let _ = CompoundPath::try_from_svg(input);
    }
});
