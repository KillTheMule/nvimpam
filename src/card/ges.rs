//! The General Entity Selection scheme of Pamcrash.

/// An enum to denote the type of a GES. Not yet used.
#[derive(Debug, PartialEq)]
pub enum GesType {
  GesNode,
  GesEle,
  GesEdge,
  GesFace,
}

impl GesType {
  /// Checks if a given line fits the basic format of a line in a GES: 8 blanks
  /// followed by one of several keywords. Checks nothing else.
  pub fn contains<T: AsRef<str>>(&self, line: &T) -> bool {
    let b = line.as_ref().as_bytes();

    let len = b.len();

    if len < 12 || &b[0..8] != b"        " {
      false
    } else {
      match &b[8..12] {
        b"ELE " | b"GRP " | b"NOD " | b"SEG " | b"EDG " | b"MOD " => true,
        b"OGRP" => len >= 13 && &b[12..13] == b" ",
        b"DELN" => len >= 15 && &b[12..15] == b"OD ",
        b"DELE" => {
          if len < 15 {
            false
          } else {
            match &b[12..15] {
              b"LE " => true,
              b"LE>" => len >= 19 && &b[15..19] == b"NOD",
              _ => false,
            }
          }
        }
        b"DELG" => {
          if len < 15 {
            false
          } else {
            match &b[12..15] {
              b"RP " => true,
              b"RP>" => len >= 19 && &b[15..19] == b"NOD ",
              _ => false,
            }
          }
        }
        b"END_" => len >= 15 && &b[12..15] == b"MOD",
        b"ELE>" | b"GRP>" => len >= 16 && &b[12..16] == b"NOD ",
        b"DELP" => {
          len >= 16 && &b[12..16] == b"ART "
            || len >= 20 && &b[12..20] == b"ART>NOD "
        }
        b"PART" => {
          len >= 13 && &b[12..13] == b" " || len >= 17 && &b[12..17] == b">NOD "
        }
        _ => false,
      }
    }
  }

  /// Check if a given line ends a GES. That is, it consists of 8 blanks
  /// followed by "END"
  pub fn ended_by<T: AsRef<str>>(&self, line: &T) -> bool {
    let b = line.as_ref().as_bytes();
    let len = b.len();

    len == 11 && &b[0..11] == b"        END"
  }
}

#[cfg(test)]
mod tests {
  use card::ges::GesType;

  const LINES: [&'static str; 10] = [
    "ab ll",
    "  aslb",
    "        NIX",
    "        ELE ",
    "        END_MOD",
    "ELE",
    "        END",
    "        DELGRP>NOD ",
    "        DELPART 134",
    "        OGRP 'hausbau'",
  ];

  #[test]
  fn test_is_ges() {
    let g = GesType::GesNode;
    let v = vec![
      false, false, false, true, true, false, false, true, true, true,
    ];
    assert_eq!(
      v,
      LINES.iter().map(|l| g.contains(&l)).collect::<Vec<bool>>()
    );
  }

  #[test]
  fn test_ends_ges() {
    let g = GesType::GesNode;
    let v = vec![
      false, false, false, false, false, false, true, false, false, false,
    ];
    assert_eq!(
      v,
      LINES.iter().map(|l| g.ended_by(&l)).collect::<Vec<bool>>()
    );
  }

}
