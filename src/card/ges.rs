#[derive(Debug, PartialEq)]
pub enum GesType {
  GesNode,
  GesEle,
  GesEdge,
  GesFace,
}

impl GesType {
  pub fn contains<T: AsRef<str>>(&self, line: &T) -> bool {
    let b = line.as_ref().as_bytes();

    let len = b.len();
    
    if len < 12 {
      false
    } else if &b[0..8] != b"        " {
      false 
    } else {
      match &b[8..12] {
        b"ELE " | b"GRP " | b"NOD " | b"SEG " | b"EDG " | b"MOD " => true,
        b"OGRP" => {
          if len < 13 {
            false
          } else if &b[12..13] == b" " {
            true
          } else { false }
        }
        b"DELN" => {
          if len < 15 {
            false
          } else if &b[12..15] == b"NOD " {
            true
          } else { false }
        }
        b"DELE" => {
          if len < 15 {
            false
          } else {
            match &b[12..15] {
              b"LE " => true,
              b"LE>" => {
                if len < 19 {
                  false
                } else if &b[15..19] == b"NOD" {
                  true
                } else {false }
              }
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
              b"RP>" => {
                if len < 19 {
                  false
                } else if &b[15..19] == b"NOD " {
                  true
                } else { false }
              }
              _ => false,
            }
          }
        }
        b"END_" => {
          if len < 15 {
            false
          } else if &b[12..15] == b"MOD" {
            true
          } else { false }
        }
        b"ELE>" => {
          if len < 16 {
            false
          } else if &b[12..16] == b"NOD " {
            true
          } else { false }
        }
        b"GRP>" => {
          if len < 16 {
            false
          } else if &b[12..16] == b"NOD " {
            true
          } else { false }
        }
        b"DELP" => {
          if len < 16 {
            false
          } else if &b[12..16] == b"ART " {
            true
          } else if len < 20 {
            false
          } else if &b[12..20] == b"ART>NOD " {
            true
          } else { false }
        }
        b"PART" => {
          if len < 13 {
            false
          } else if &b[12..13] == b" " {
            true
          } else if len < 17 {
            false
          } else if &b[12..17] == b">NOD " {
            true
          } else { false }
        }
        _ => false,
      }
    }
  }

  pub fn ended_by<T: AsRef<str>>(&self, line: &T) -> bool {
    let b = line.as_ref().as_bytes();
    let len = b.len();

    if len == 11 && &b[0..11] == b"        END" {
      true
    } else { false }
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
      false, false, false, true, true, false, false, true, true, true
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
      false, false, false, false, false, false, true, false, false, false
    ];
    assert_eq!(
      v,
      LINES.iter().map(|l| g.ended_by(&l)).collect::<Vec<bool>>()
    );
  }

}
