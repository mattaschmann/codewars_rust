// Preloaded:
//
// struct MorseDecoder {
//     morse_code: HashMap<String, String>,
// }
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

impl MorseDecoder {

  fn decode_morse(&self, encoded: &str) -> String {
    encoded.trim()
           .split("   ")
           .map(|morse_word| {
             morse_word.split(' ')
                       .filter_map(|c| self.morse_code.get(c))
                       .cloned()
                       .collect()
           })
           .collect::<Vec<String>>()
           .join(" ")
  }
}
