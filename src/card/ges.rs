//! The General Entity Selection scheme of Pamcrash.

/// An enum to denote the type of a GES. Not yet used.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GesType {
  GesNode,
  GesEle,
  GesEdge,
  GesFace,
}

impl GesType {
  /// Checks if a given line fits the basic format of a line in a GES: 8 blanks
  /// followed by one of several keywords. Checks nothing else.
  pub fn contains(self, b: &[u8]) -> bool {
    use byteorder::{BigEndian, ReadBytesExt};

    let len = b.len();

    // b"        " as a u64 in BigEndian is 2314885530818453536
    if len < 12
      || (&b[0..8]).read_u64::<BigEndian>().ok() != Some(2314885530818453536)
    {
      false
    } else {
      let num = match (&b[8..12]).read_u32::<BigEndian>() {
        Ok(n) => n,
        Err(_) => return false,
      };

      match num {
        // b"ELE " | b"GRP " | b"NOD " | b"SEG " | b"EDG " | b"MOD "
        1162626336 | 1196576800 | 1313817632 | 1162102560 | 1297040416 => true,
        // b"OGRP"
        1330074192 => len >= 13 && &b[12] == &b' ',
        // b"DELN"
        1145392206 => {
          len >= 15
            // b"OD " as u24 in BigEndian is 5194784
            && (&b[12..15]).read_u24::<BigEndian>().ok() == Some(5194784)
        }
        // b"DELE"
        1145392197 => {
          if len < 15 {
            false
          } else {
            match (&b[12..15]).read_u24::<BigEndian>().ok() {
              // b"LE " as u24 in BigEndian is 4998432
              Some(4998432) => true,
              // b"LE>" as u24 in BigEndian is 4998462
              Some(4998462) => {
                len >= 19
                  // b"NOD " as u32 in BigEndian is 1313817632
                  && (&b[15..19]).read_u32::<BigEndian>().ok()
                    == Some(1313817632)
              }
              _ => false,
            }
          }
        }
        // b"DELG"
        1145392199 => {
          if len < 15 {
            false
          } else {
            match (&b[12..15]).read_u24::<BigEndian>().ok() {
              // b"RP " as u24 in BigEndian is 5394464
              Some(5394464) => true,
              // b"RP>" as u24 in BigEndian is 5394494
              Some(5394494) => {
                len >= 19
                  // b"NOD " as u32 in BigEndian is 1313817632
                  && (&b[15..19]).read_u32::<BigEndian>().ok()
                    == Some(1313817632)
              }
              _ => false,
            }
          }
        }
        // b"END_"
        1162757215 => {
          len >= 15
            // b"MOD" as u24 in BigEndian is 5066564
            && (&b[12..15]).read_u24::<BigEndian>().ok() == Some(5066564)
        }
        // b"ELE>" | b"GRP>"
        1162626366 | 1196576830 => {
          // b"NOD " as u32 in BigEndian is 1313817632
          len >= 16
            && (&b[12..16]).read_u32::<BigEndian>().ok() == Some(1313817632)
        }
        // b"DELP"
        1145392208 => {
          // b"ART " as u32 in BigEndian is 1095914528
          // b"ART>NOD " as u64 in BigEndian is 4706917187134112800
          len >= 16
            && (&b[12..16]).read_u32::<BigEndian>().ok() == Some(1095914528)
            || len >= 20
              && (&b[12..20]).read_u64::<BigEndian>().ok()
                == Some(4706917187134112800)
        }
        // b"PART"
        1346458196 => {
          len >= 13 && &b[12] == &b' '
            || len >= 17
              && &b[16] == &b' '
              // b">NOD" as u32 in BigEndian is 1045319492
              && (&b[12..16]).read_u32::<BigEndian>().ok() == Some(1045319492)
        }
        _ => false,
      }
    }
  }

  /// Check if a given line ends a GES. That is, it consists of 8 blanks
  /// followed by "END"
  pub fn ended_by(self, b: &[u8]) -> bool {
    let len = b.len();

    len == 11 && &b[0..11] == b"        END"
  }
}

#[cfg(test)]
mod tests {
  use card::ges::GesType::GesNode;

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
    let v = vec![
      false, false, false, true, true, false, false, true, true, true,
    ];
    assert_eq!(
      v,
      LINES
        .iter()
        .map(|l| GesNode.contains(l.as_ref()))
        .collect::<Vec<bool>>()
    );
  }

  #[test]
  fn test_ends_ges() {
    let v = vec![
      false, false, false, false, false, false, true, false, false, false,
    ];
    assert_eq!(
      v,
      LINES
        .iter()
        .map(|l| GesNode.ended_by(l.as_ref()))
        .collect::<Vec<bool>>()
    );
  }

  /*
  // Keep this for later, may we'll need it again
  #[test]
  fn byteslice_to_u64() {
    use byteorder::{BigEndian, ReadBytesExt};

    /*
    let s1 = String::from("3");
    let mut v = Vec::new();
    
    for i in 0..=8-s1.len(){
      let mut s2 = String::new();
      for j in 0..i {
        s2.push(' ');
      }
      s2.push_str(&s1);
      for j in i..8-s1.len(){
        s2.push(' ');
      }
      v.push(s2);
    }
    
    let l = v.len()-1;
    let mut comment = String::new();
    for (i, x) in v.iter().enumerate() {
      match i {
        0 => comment.push_str(&format!("// \"{}\",", x)),
        _ => comment.push_str(&format!(" \"{}\",", x)),
        }
    }
    let _ = comment.pop();
    eprintln!("{}", comment);
    
    let mut s = String::new();
    
    for x in v {
      let num = x.as_bytes().read_u64::<BigEndian>().unwrap();
      s.push_str(&format!(" {} |", num));
     }
    
    let _ = s.pop();
    s.push_str("=> Some(");
    eprintln!("{}", s);
    */

    /*
    let s = b"        ";
    let mut c = &s[..];
    let num = c.read_u64::<BigEndian>().unwrap();
    eprintln!("{}", num);
    */

    /*
    let b = [b"MOD"];

    for str in b.iter() {
      let mut c = &str[..];
      let num = c.read_u24::<BigEndian>().unwrap();
      eprintln!("{}", num);
    }
    assert!(false);
    */
  }
  */

}
