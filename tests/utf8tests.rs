use std::io::Read;

use rutex::parsing::{Input, InputResult};
struct Utf8Test {
    name: String,
    is_valid: bool,
    input: Vec<u8>,
}
struct ReadBuf {
    buf: Vec<u8>,
    pos: usize,
}
impl Read for ReadBuf {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.buf.len() - self.pos);
        buf[..len].copy_from_slice(&self.buf[self.pos..self.pos + len]);
        self.pos += len;
        Ok(len)
    }
}
impl ReadBuf {
    fn new(buf: Vec<u8>) -> Self {
        Self { buf, pos: 0 }
    }
}

/// Tests modified from https://github.com/flenniken/utf8tests
#[test]
fn test() -> Result<(), ()> {
    let tests: Vec<Utf8Test> = vec![
        Utf8Test {
            name: String::from("1.0.1"),
            is_valid: true,
            input: vec![49],
        },
        Utf8Test {
            name: String::from("1.1.0"),
            is_valid: true,
            input: vec![97, 98, 99],
        },
        Utf8Test {
            name: String::from("2.1.0"),
            is_valid: true,
            input: vec![194, 169],
        },
        Utf8Test {
            name: String::from("3.0"),
            is_valid: true,
            input: vec![226, 128, 144],
        },
        Utf8Test {
            name: String::from("4.0"),
            is_valid: true,
            input: vec![240, 157, 146, 156],
        },
        Utf8Test {
            name: String::from("5.1"),
            is_valid: true,
            input: vec![194, 128],
        },
        Utf8Test {
            name: String::from("5.2"),
            is_valid: true,
            input: vec![224, 160, 128],
        },
        Utf8Test {
            name: String::from("5.3"),
            is_valid: true,
            input: vec![240, 144, 128, 128],
        },
        Utf8Test {
            name: String::from("7.1"),
            is_valid: true,
            input: vec![194, 128],
        },
        Utf8Test {
            name: String::from("7.2"),
            is_valid: true,
            input: vec![194, 129],
        },
        Utf8Test {
            name: String::from("7.3"),
            is_valid: true,
            input: vec![194, 130],
        },
        Utf8Test {
            name: String::from("8.0"),
            is_valid: true,
            input: vec![127],
        },
        Utf8Test {
            name: String::from("8.1"),
            is_valid: true,
            input: vec![223, 191],
        },
        Utf8Test {
            name: String::from("8.2"),
            is_valid: true,
            input: vec![239, 191, 191],
        },
        Utf8Test {
            name: String::from("8.3"),
            is_valid: true,
            input: vec![244, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("10.1"),
            is_valid: true,
            input: vec![238, 128, 128],
        },
        Utf8Test {
            name: String::from("10.2"),
            is_valid: true,
            input: vec![239, 191, 189],
        },
        Utf8Test {
            name: String::from("10.3"),
            is_valid: true,
            input: vec![244, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("22.0"),
            is_valid: true,
            input: vec![47],
        },
        Utf8Test {
            name: String::from("22.1"),
            is_valid: true,
            input: vec![47],
        },
        Utf8Test {
            name: String::from("22.7"),
            is_valid: true,
            input: vec![224, 160, 128],
        },
        Utf8Test {
            name: String::from("6.0"),
            is_valid: false,
            input: vec![247, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("6.0.1"),
            is_valid: false,
            input: vec![244, 144, 128, 128],
        },
        Utf8Test {
            name: String::from("6.1"),
            is_valid: false,
            input: vec![248, 136, 128, 128, 128],
        },
        Utf8Test {
            name: String::from("6.2"),
            is_valid: false,
            input: vec![247, 191, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("6.3"),
            is_valid: false,
            input: vec![252, 132, 128, 128, 128, 128],
        },
        Utf8Test {
            name: String::from("6.4"),
            is_valid: false,
            input: vec![247, 191, 191, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("6.5"),
            is_valid: false,
            input: vec![247, 191, 191, 191, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("9.0"),
            is_valid: false,
            input: vec![247, 191, 191],
        },
        Utf8Test {
            name: String::from("11.0"),
            is_valid: false,
            input: vec![128],
        },
        Utf8Test {
            name: String::from("11.1"),
            is_valid: false,
            input: vec![191],
        },
        Utf8Test {
            name: String::from("11.2"),
            is_valid: false,
            input: vec![128, 191],
        },
        Utf8Test {
            name: String::from("11.3"),
            is_valid: false,
            input: vec![128, 191, 128],
        },
        Utf8Test {
            name: String::from("11.4"),
            is_valid: false,
            input: vec![128, 191, 128, 191],
        },
        Utf8Test {
            name: String::from("11.5"),
            is_valid: false,
            input: vec![128, 191, 128, 191, 128],
        },
        Utf8Test {
            name: String::from("11.6"),
            is_valid: false,
            input: vec![128, 191, 128, 191, 128, 191],
        },
        Utf8Test {
            name: String::from("12.0"),
            is_valid: false,
            input: vec![128, 129, 130, 131, 132, 133, 134, 135],
        },
        Utf8Test {
            name: String::from("12.1"),
            is_valid: false,
            input: vec![136, 137, 138, 139, 140, 141, 142, 143],
        },
        Utf8Test {
            name: String::from("12.2"),
            is_valid: false,
            input: vec![144, 145, 146, 147, 148, 149, 150, 151],
        },
        Utf8Test {
            name: String::from("12.3"),
            is_valid: false,
            input: vec![152, 153, 154, 155, 156, 157, 158, 159],
        },
        Utf8Test {
            name: String::from("12.4"),
            is_valid: false,
            input: vec![160, 161, 162, 163, 164, 165, 166, 167],
        },
        Utf8Test {
            name: String::from("12.5"),
            is_valid: false,
            input: vec![168, 169, 170, 171, 172, 173, 174, 175],
        },
        Utf8Test {
            name: String::from("12.6"),
            is_valid: false,
            input: vec![176, 177, 178, 179, 180, 181, 182, 183],
        },
        Utf8Test {
            name: String::from("12.7"),
            is_valid: false,
            input: vec![184, 185, 186, 187, 188, 189, 190, 191],
        },
        Utf8Test {
            name: String::from("13.0"),
            is_valid: false,
            input: vec![192, 32, 193, 32, 194, 32, 195, 32],
        },
        Utf8Test {
            name: String::from("13.1"),
            is_valid: false,
            input: vec![196, 32, 197, 32, 198, 32, 199, 32],
        },
        Utf8Test {
            name: String::from("13.2"),
            is_valid: false,
            input: vec![200, 32, 201, 32, 202, 32, 203, 32],
        },
        Utf8Test {
            name: String::from("13.3"),
            is_valid: false,
            input: vec![204, 32, 205, 32, 206, 32, 207, 32],
        },
        Utf8Test {
            name: String::from("13.4"),
            is_valid: false,
            input: vec![208, 32, 209, 32, 210, 32, 211, 32],
        },
        Utf8Test {
            name: String::from("13.5"),
            is_valid: false,
            input: vec![212, 32, 213, 32, 214, 32, 215, 32],
        },
        Utf8Test {
            name: String::from("13.6"),
            is_valid: false,
            input: vec![216, 32, 217, 32, 218, 32, 219, 32],
        },
        Utf8Test {
            name: String::from("13.7"),
            is_valid: false,
            input: vec![220, 32, 221, 32, 222, 32, 223, 32],
        },
        Utf8Test {
            name: String::from("14.0"),
            is_valid: false,
            input: vec![224, 32, 225, 32, 226, 32, 227, 32],
        },
        Utf8Test {
            name: String::from("14.1"),
            is_valid: false,
            input: vec![228, 32, 229, 32, 230, 32, 231, 32],
        },
        Utf8Test {
            name: String::from("14.2"),
            is_valid: false,
            input: vec![232, 32, 233, 32, 234, 32, 235, 32],
        },
        Utf8Test {
            name: String::from("14.3"),
            is_valid: false,
            input: vec![236, 32, 237, 32, 238, 32, 239, 32],
        },
        Utf8Test {
            name: String::from("14.4.0"),
            is_valid: false,
            input: vec![192, 175, 224, 128, 191, 240, 129, 130, 65],
        },
        Utf8Test {
            name: String::from("14.4.1"),
            is_valid: false,
            input: vec![237, 160, 128, 237, 191, 191, 237, 175, 65],
        },
        Utf8Test {
            name: String::from("14.4.2"),
            is_valid: false,
            input: vec![244, 145, 146, 147, 255, 65, 128, 191, 66],
        },
        Utf8Test {
            name: String::from("14.5.1"),
            is_valid: false,
            input: vec![225, 128, 226, 240, 145, 146, 241, 191, 65],
        },
        Utf8Test {
            name: String::from("15.0"),
            is_valid: false,
            input: vec![240, 32, 241, 32],
        },
        Utf8Test {
            name: String::from("15.1"),
            is_valid: false,
            input: vec![242, 32, 243, 32],
        },
        Utf8Test {
            name: String::from("15.2"),
            is_valid: false,
            input: vec![244, 32, 245, 32],
        },
        Utf8Test {
            name: String::from("15.3"),
            is_valid: false,
            input: vec![246, 32, 247, 32],
        },
        Utf8Test {
            name: String::from("16.0"),
            is_valid: false,
            input: vec![248, 32],
        },
        Utf8Test {
            name: String::from("16.1"),
            is_valid: false,
            input: vec![249, 32],
        },
        Utf8Test {
            name: String::from("16.2"),
            is_valid: false,
            input: vec![250, 32],
        },
        Utf8Test {
            name: String::from("16.3"),
            is_valid: false,
            input: vec![251, 32],
        },
        Utf8Test {
            name: String::from("17.0"),
            is_valid: false,
            input: vec![252, 32],
        },
        Utf8Test {
            name: String::from("17.1"),
            is_valid: false,
            input: vec![253, 32],
        },
        Utf8Test {
            name: String::from("18.0"),
            is_valid: false,
            input: vec![192],
        },
        Utf8Test {
            name: String::from("18.1"),
            is_valid: false,
            input: vec![224, 128],
        },
        Utf8Test {
            name: String::from("18.2"),
            is_valid: false,
            input: vec![240, 128, 128],
        },
        Utf8Test {
            name: String::from("18.3"),
            is_valid: false,
            input: vec![248, 128, 128, 128],
        },
        Utf8Test {
            name: String::from("18.4"),
            is_valid: false,
            input: vec![252, 128, 128, 128, 128],
        },
        Utf8Test {
            name: String::from("19.0"),
            is_valid: false,
            input: vec![223],
        },
        Utf8Test {
            name: String::from("19.1"),
            is_valid: false,
            input: vec![239, 191],
        },
        Utf8Test {
            name: String::from("19.2"),
            is_valid: false,
            input: vec![247, 191, 191],
        },
        Utf8Test {
            name: String::from("19.3"),
            is_valid: false,
            input: vec![251, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("19.4"),
            is_valid: false,
            input: vec![253, 191, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("19.5"),
            is_valid: false,
            input: vec![49, 50, 51, 239, 128],
        },
        Utf8Test {
            name: String::from("19.6"),
            is_valid: false,
            input: vec![49, 50, 51, 239, 128, 240],
        },
        Utf8Test {
            name: String::from("21.0"),
            is_valid: false,
            input: vec![128],
        },
        Utf8Test {
            name: String::from("21.1"),
            is_valid: false,
            input: vec![129],
        },
        Utf8Test {
            name: String::from("21.2"),
            is_valid: false,
            input: vec![254],
        },
        Utf8Test {
            name: String::from("21.3"),
            is_valid: false,
            input: vec![255],
        },
        Utf8Test {
            name: String::from("21.4"),
            is_valid: false,
            input: vec![55, 255],
        },
        Utf8Test {
            name: String::from("21.5"),
            is_valid: false,
            input: vec![55, 56, 254],
        },
        Utf8Test {
            name: String::from("21.6"),
            is_valid: false,
            input: vec![55, 56, 57, 254],
        },
        Utf8Test {
            name: String::from("22.2"),
            is_valid: false,
            input: vec![192, 175],
        },
        Utf8Test {
            name: String::from("22.3"),
            is_valid: false,
            input: vec![224, 128, 175],
        },
        Utf8Test {
            name: String::from("22.4"),
            is_valid: false,
            input: vec![240, 128, 128, 175],
        },
        Utf8Test {
            name: String::from("22.5"),
            is_valid: false,
            input: vec![248, 128, 128, 128, 175],
        },
        Utf8Test {
            name: String::from("22.6"),
            is_valid: false,
            input: vec![252, 128, 128, 128, 128, 175],
        },
        Utf8Test {
            name: String::from("23.0"),
            is_valid: false,
            input: vec![193, 191],
        },
        Utf8Test {
            name: String::from("23.1"),
            is_valid: false,
            input: vec![224, 159, 191],
        },
        Utf8Test {
            name: String::from("23.2"),
            is_valid: false,
            input: vec![240, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("23.3"),
            is_valid: false,
            input: vec![248, 135, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("24.0"),
            is_valid: false,
            input: vec![237, 160, 128],
        },
        Utf8Test {
            name: String::from("24.0.1"),
            is_valid: false,
            input: vec![237, 160, 128, 53],
        },
        Utf8Test {
            name: String::from("24.0.2"),
            is_valid: false,
            input: vec![49, 50, 51, 237, 160, 128, 49],
        },
        Utf8Test {
            name: String::from("24.2"),
            is_valid: false,
            input: vec![237, 173, 191],
        },
        Utf8Test {
            name: String::from("24.3"),
            is_valid: false,
            input: vec![237, 174, 128],
        },
        Utf8Test {
            name: String::from("24.4"),
            is_valid: false,
            input: vec![237, 175, 191],
        },
        Utf8Test {
            name: String::from("24.5"),
            is_valid: false,
            input: vec![237, 176, 128],
        },
        Utf8Test {
            name: String::from("24.6"),
            is_valid: false,
            input: vec![237, 190, 128],
        },
        Utf8Test {
            name: String::from("24.7"),
            is_valid: false,
            input: vec![237, 191, 191],
        },
        Utf8Test {
            name: String::from("25.0"),
            is_valid: false,
            input: vec![237, 160, 128, 237, 176, 128],
        },
        Utf8Test {
            name: String::from("25.1"),
            is_valid: false,
            input: vec![237, 160, 128, 237, 191, 191],
        },
        Utf8Test {
            name: String::from("25.2"),
            is_valid: false,
            input: vec![237, 173, 191, 237, 176, 128],
        },
        Utf8Test {
            name: String::from("25.3"),
            is_valid: false,
            input: vec![237, 173, 191, 237, 191, 191],
        },
        Utf8Test {
            name: String::from("25.4"),
            is_valid: false,
            input: vec![237, 174, 128, 237, 176, 128],
        },
        Utf8Test {
            name: String::from("25.5"),
            is_valid: false,
            input: vec![237, 174, 128, 237, 191, 191],
        },
        Utf8Test {
            name: String::from("25.6"),
            is_valid: false,
            input: vec![237, 175, 191, 237, 176, 128],
        },
        Utf8Test {
            name: String::from("25.7"),
            is_valid: false,
            input: vec![237, 175, 191, 237, 191, 191],
        },
        Utf8Test {
            name: String::from("26.0"),
            is_valid: true,
            input: vec![239, 191, 190],
        },
        Utf8Test {
            name: String::from("26.1"),
            is_valid: true,
            input: vec![239, 191, 191],
        },
        Utf8Test {
            name: String::from("26.2"),
            is_valid: true,
            input: vec![239, 183, 144],
        },
        Utf8Test {
            name: String::from("26.3"),
            is_valid: true,
            input: vec![239, 183, 145],
        },
        Utf8Test {
            name: String::from("26.4"),
            is_valid: true,
            input: vec![239, 183, 146],
        },
        Utf8Test {
            name: String::from("26.5"),
            is_valid: true,
            input: vec![239, 183, 147],
        },
        Utf8Test {
            name: String::from("26.6"),
            is_valid: true,
            input: vec![239, 183, 148],
        },
        Utf8Test {
            name: String::from("26.7"),
            is_valid: true,
            input: vec![239, 183, 149],
        },
        Utf8Test {
            name: String::from("26.8"),
            is_valid: true,
            input: vec![239, 183, 150],
        },
        Utf8Test {
            name: String::from("26.9"),
            is_valid: true,
            input: vec![239, 183, 151],
        },
        Utf8Test {
            name: String::from("26.10"),
            is_valid: true,
            input: vec![239, 183, 152],
        },
        Utf8Test {
            name: String::from("26.11"),
            is_valid: true,
            input: vec![239, 183, 153],
        },
        Utf8Test {
            name: String::from("26.12"),
            is_valid: true,
            input: vec![239, 183, 154],
        },
        Utf8Test {
            name: String::from("26.13"),
            is_valid: true,
            input: vec![239, 183, 155],
        },
        Utf8Test {
            name: String::from("26.14"),
            is_valid: true,
            input: vec![239, 183, 156],
        },
        Utf8Test {
            name: String::from("26.15"),
            is_valid: true,
            input: vec![239, 183, 157],
        },
        Utf8Test {
            name: String::from("26.16"),
            is_valid: true,
            input: vec![239, 183, 158],
        },
        Utf8Test {
            name: String::from("26.17"),
            is_valid: true,
            input: vec![239, 183, 159],
        },
        Utf8Test {
            name: String::from("27.0"),
            is_valid: true,
            input: vec![240, 159, 191, 190],
        },
        Utf8Test {
            name: String::from("27.1"),
            is_valid: true,
            input: vec![240, 175, 191, 190],
        },
        Utf8Test {
            name: String::from("27.2"),
            is_valid: true,
            input: vec![240, 191, 191, 190],
        },
        Utf8Test {
            name: String::from("27.3"),
            is_valid: true,
            input: vec![241, 143, 191, 190],
        },
        Utf8Test {
            name: String::from("27.4"),
            is_valid: true,
            input: vec![241, 159, 191, 190],
        },
        Utf8Test {
            name: String::from("27.5"),
            is_valid: true,
            input: vec![241, 175, 191, 190],
        },
        Utf8Test {
            name: String::from("27.6"),
            is_valid: true,
            input: vec![241, 191, 191, 190],
        },
        Utf8Test {
            name: String::from("27.7"),
            is_valid: true,
            input: vec![242, 143, 191, 190],
        },
        Utf8Test {
            name: String::from("27.8"),
            is_valid: true,
            input: vec![242, 159, 191, 190],
        },
        Utf8Test {
            name: String::from("27.9"),
            is_valid: true,
            input: vec![242, 175, 191, 190],
        },
        Utf8Test {
            name: String::from("27.10"),
            is_valid: true,
            input: vec![242, 191, 191, 190],
        },
        Utf8Test {
            name: String::from("27.11"),
            is_valid: true,
            input: vec![243, 143, 191, 190],
        },
        Utf8Test {
            name: String::from("27.12"),
            is_valid: true,
            input: vec![243, 159, 191, 190],
        },
        Utf8Test {
            name: String::from("27.13"),
            is_valid: true,
            input: vec![243, 175, 191, 190],
        },
        Utf8Test {
            name: String::from("27.14"),
            is_valid: true,
            input: vec![243, 191, 191, 190],
        },
        Utf8Test {
            name: String::from("27.15"),
            is_valid: true,
            input: vec![244, 143, 191, 190],
        },
        Utf8Test {
            name: String::from("28.0"),
            is_valid: true,
            input: vec![240, 159, 191, 191],
        },
        Utf8Test {
            name: String::from("28.1"),
            is_valid: true,
            input: vec![240, 175, 191, 191],
        },
        Utf8Test {
            name: String::from("28.2"),
            is_valid: true,
            input: vec![240, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("28.3"),
            is_valid: true,
            input: vec![241, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("28.4"),
            is_valid: true,
            input: vec![241, 159, 191, 191],
        },
        Utf8Test {
            name: String::from("28.5"),
            is_valid: true,
            input: vec![241, 175, 191, 191],
        },
        Utf8Test {
            name: String::from("28.6"),
            is_valid: true,
            input: vec![241, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("28.7"),
            is_valid: true,
            input: vec![242, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("28.8"),
            is_valid: true,
            input: vec![242, 159, 191, 191],
        },
        Utf8Test {
            name: String::from("28.9"),
            is_valid: true,
            input: vec![242, 175, 191, 191],
        },
        Utf8Test {
            name: String::from("28.10"),
            is_valid: true,
            input: vec![242, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("28.11"),
            is_valid: true,
            input: vec![243, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("28.12"),
            is_valid: true,
            input: vec![243, 159, 191, 191],
        },
        Utf8Test {
            name: String::from("28.13"),
            is_valid: true,
            input: vec![243, 175, 191, 191],
        },
        Utf8Test {
            name: String::from("28.14"),
            is_valid: true,
            input: vec![243, 191, 191, 191],
        },
        Utf8Test {
            name: String::from("28.15"),
            is_valid: true,
            input: vec![244, 143, 191, 191],
        },
        Utf8Test {
            name: String::from("29.0"),
            is_valid: false,
            input: vec![128],
        },
        Utf8Test {
            name: String::from("29.1"),
            is_valid: false,
            input: vec![32, 128],
        },
        Utf8Test {
            name: String::from("29.2"),
            is_valid: false,
            input: vec![32, 33, 33, 35, 254, 32],
        },
        Utf8Test {
            name: String::from("29.3"),
            is_valid: false,
            input: vec![32, 33, 33, 35, 36, 254],
        },
        Utf8Test {
            name: String::from("29.4"),
            is_valid: false,
            input: vec![128, 32],
        },
        Utf8Test {
            name: String::from("29.5"),
            is_valid: false,
            input: vec![32, 128, 32],
        },
        Utf8Test {
            name: String::from("29.6"),
            is_valid: false,
            input: vec![129, 32],
        },
        Utf8Test {
            name: String::from("29.7"),
            is_valid: false,
            input: vec![193, 32],
        },
        Utf8Test {
            name: String::from("29.8"),
            is_valid: false,
            input: vec![245, 32],
        },
        Utf8Test {
            name: String::from("29.9"),
            is_valid: false,
            input: vec![255, 32],
        },
        Utf8Test {
            name: String::from("30.1"),
            is_valid: false,
            input: vec![194, 127],
        },
        Utf8Test {
            name: String::from("30.2"),
            is_valid: false,
            input: vec![194, 192],
        },
        Utf8Test {
            name: String::from("30.3"),
            is_valid: false,
            input: vec![194, 255],
        },
        Utf8Test {
            name: String::from("30.5"),
            is_valid: false,
            input: vec![223, 127],
        },
        Utf8Test {
            name: String::from("30.6"),
            is_valid: false,
            input: vec![223, 192],
        },
        Utf8Test {
            name: String::from("30.7"),
            is_valid: false,
            input: vec![223, 255],
        },
        Utf8Test {
            name: String::from("31.1"),
            is_valid: false,
            input: vec![224, 128, 127],
        },
        Utf8Test {
            name: String::from("31.2"),
            is_valid: false,
            input: vec![224, 128, 192],
        },
        Utf8Test {
            name: String::from("31.3"),
            is_valid: false,
            input: vec![224, 128, 255],
        },
        Utf8Test {
            name: String::from("32.1"),
            is_valid: false,
            input: vec![237, 128, 127],
        },
        Utf8Test {
            name: String::from("32.2"),
            is_valid: false,
            input: vec![237, 128, 192],
        },
        Utf8Test {
            name: String::from("32.3"),
            is_valid: false,
            input: vec![237, 128, 255],
        },
        Utf8Test {
            name: String::from("33.1"),
            is_valid: false,
            input: vec![240, 144, 128, 127],
        },
        Utf8Test {
            name: String::from("33.2"),
            is_valid: false,
            input: vec![240, 144, 128, 192],
        },
        Utf8Test {
            name: String::from("33.3"),
            is_valid: false,
            input: vec![240, 144, 128, 255],
        },
        Utf8Test {
            name: String::from("34.1"),
            is_valid: false,
            input: vec![241, 128, 128, 127],
        },
        Utf8Test {
            name: String::from("34.2"),
            is_valid: false,
            input: vec![241, 128, 128, 192],
        },
        Utf8Test {
            name: String::from("34.3"),
            is_valid: false,
            input: vec![241, 128, 128, 255],
        },
        Utf8Test {
            name: String::from("35.1"),
            is_valid: false,
            input: vec![244, 128, 128, 127],
        },
        Utf8Test {
            name: String::from("35.2"),
            is_valid: false,
            input: vec![244, 128, 128, 192],
        },
        Utf8Test {
            name: String::from("35.3"),
            is_valid: false,
            input: vec![244, 128, 128, 255],
        },
        Utf8Test {
            name: String::from("9.1"),
            is_valid: false,
            input: vec![194, 65, 66],
        },
        Utf8Test {
            name: String::from("36.1"),
            is_valid: true,
            input: vec![
                114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 99, 104, 97, 114, 97, 99,
                116, 101, 114, 61, 239, 191, 189, 61, 239, 191, 189, 46,
            ],
        },
        Utf8Test {
            name: String::from("36.2"),
            is_valid: false,
            input: vec![239, 191, 189, 61, 255, 46],
        },
        Utf8Test {
            name: String::from("36.3"),
            is_valid: false,
            input: vec![239, 191, 189, 239, 191, 189, 61, 224, 128, 46],
        },
        Utf8Test {
            name: String::from("36.4"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 240, 128, 128, 46,
            ],
        },
        Utf8Test {
            name: String::from("36.5"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 240, 128, 128, 128,
                46,
            ],
        },
        Utf8Test {
            name: String::from("36.6"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 224, 128, 224, 128,
                46,
            ],
        },
        Utf8Test {
            name: String::from("36.7"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 247, 191, 191, 191,
                46,
            ],
        },
        Utf8Test {
            name: String::from("36.8"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 237, 160, 128, 46,
            ],
        },
        Utf8Test {
            name: String::from("36.10"),
            is_valid: false,
            input: vec![
                239, 191, 189, 239, 191, 189, 239, 191, 189, 61, 224, 128, 175, 46,
            ],
        },
        Utf8Test {
            name: String::from("36.9"),
            is_valid: true,
            input: vec![239, 191, 191, 61, 239, 191, 191, 46],
        },
        Utf8Test {
            name: String::from("36.9.1"),
            is_valid: true,
            input: vec![239, 191, 190, 61, 239, 191, 190, 46],
        },
        Utf8Test {
            name: String::from("5.0"),
            is_valid: true,
            input: vec![0],
        },
        Utf8Test {
            name: String::from("30.0"),
            is_valid: false,
            input: vec![194, 0],
        },
        Utf8Test {
            name: String::from("30.4"),
            is_valid: false,
            input: vec![223, 0],
        },
        Utf8Test {
            name: String::from("31.0"),
            is_valid: false,
            input: vec![224, 128, 0],
        },
        Utf8Test {
            name: String::from("32.0"),
            is_valid: false,
            input: vec![237, 128, 0],
        },
        Utf8Test {
            name: String::from("33.0"),
            is_valid: false,
            input: vec![240, 144, 128, 0],
        },
        Utf8Test {
            name: String::from("34.0"),
            is_valid: false,
            input: vec![241, 128, 128, 0],
        },
        Utf8Test {
            name: String::from("35.0"),
            is_valid: false,
            input: vec![244, 128, 128, 0],
        },
        Utf8Test {
            name: String::from("37.0"),
            is_valid: false,
            input: vec![192, 128],
        },
        Utf8Test {
            name: String::from("37.1"),
            is_valid: false,
            input: vec![224, 128, 128],
        },
        Utf8Test {
            name: String::from("37.2"),
            is_valid: false,
            input: vec![240, 128, 128, 128],
        },
        Utf8Test {
            name: String::from("37.2.1"),
            is_valid: true,
            input: vec![32, 0, 53],
        },
        Utf8Test {
            name: String::from("37.3"),
            is_valid: false,
            input: vec![32, 0, 32, 255],
        },
        Utf8Test {
            name: String::from("37.4"),
            is_valid: true,
            input: vec![32, 0],
        },
    ];
    let mut success = true;
    for test in tests.iter() {
        let mut inp =
            Input::new_from_source(&test.name, Box::new(ReadBuf::new(test.input.clone())));
        let result = loop {
            match inp.read_char() {
                InputResult::Char(_) => continue,
                InputResult::Eof => break true,
                InputResult::Error(_) => break false,
            }
        };

        if result != test.is_valid {
            println!("{:?}", String::from_utf8(test.input.clone()));
            eprintln!("Failed test: {}", test.name);
            success = false;
        } else {
            println!("Passed test: {}", test.name);
        }
    }
    if success {
        Ok(())
    } else {
        Err(())
    }
}
