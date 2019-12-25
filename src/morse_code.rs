//* Kata: https://www.codewars.com/kata/decode-the-morse-code
//* Kata: https://www.codewars.com/kata/decode-the-morse-code-advanced

use std::collections::HashMap;

lazy_static! {
    static ref MORSE_CODE: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert(".-", "A");
        m.insert("-...", "B");
        m.insert("-.-.", "C");
        m.insert("-..", "D");
        m.insert(".", "E");
        m.insert("..-.", "F");
        m.insert("--.", "G");
        m.insert("....", "H");
        m.insert("..", "I");
        m.insert(".---", "J");
        m.insert("-.-", "K");
        m.insert(".-..", "L");
        m.insert("--", "M");
        m.insert("-.", "N");
        m.insert("---", "O");
        m.insert(".--.", "P");
        m.insert("--.-", "Q");
        m.insert(".-.", "R");
        m.insert("...", "S");
        m.insert("-", "T");
        m.insert("..-", "U");
        m.insert("...-", "V");
        m.insert(".--", "W");
        m.insert("-..-", "X");
        m.insert("-.--", "Y");
        m.insert("--..", "Z");
        m.insert("-----", "0");
        m.insert(".----", "1");
        m.insert("..---", "2");
        m.insert("...--", "3");
        m.insert("....-", "4");
        m.insert(".....", "5");
        m.insert("-....", "6");
        m.insert("--...", "7");
        m.insert("---..", "8");
        m.insert("----.", "9");
        m.iter()
            .map(|(&s, &x)| (s.to_string(), x.to_string()))
            .collect()
    };
}

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}
impl MorseDecoder {
    fn new() -> MorseDecoder {
        Self {
            morse_code: MORSE_CODE.clone(),
        }
    }
    fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|word| {
                word.split(' ')
                    .filter_map(|ch| self.morse_code.get(ch))
                    .cloned()
                    .collect()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    fn decode_bits(&self, encoded: &str) -> String {
        let encoded = encoded.trim_matches('0');
        let ones_only = encoded.split('0');
        let zeros_only = encoded.split('1');
        let trans_rate = ones_only
            .chain(zeros_only)
            .filter_map(|x| if !x.is_empty() { Some(x.len()) } else { None })
            .min()
            .unwrap_or(encoded.len());
        encoded
            .replace(&("0000000".repeat(trans_rate)), "   ")
            .replace(&("000".repeat(trans_rate)), " ")
            .replace(&("111".repeat(trans_rate)), "-")
            .replace(&("1".repeat(trans_rate)), ".")
            .replace(&("0".repeat(trans_rate)), "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }
    #[test]
    fn advanced() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
        assert_eq!(
            decoder.decode_morse(&decoder.decode_bits("1")),
            "E".to_string()
        );
        assert_eq!(
            decoder.decode_morse(&decoder.decode_bits("001110111000")),
            "M".to_string()
        );
        assert_eq!(
            decoder.decode_morse(&decoder.decode_bits("10001")),
            "EE".to_string()
        );
    }
    #[test]
    fn for_real() {
        assert!(true)
    }
}
