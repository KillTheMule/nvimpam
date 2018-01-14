#[derive(Debug, PartialEq)]
pub enum GesType {
  GesNode,
  GesEle,
  GesEdge,
  GesFace,
}

impl GesType {
  pub fn contains<T: AsRef<str>>(&self,line: &T) -> bool {
    let b = line.as_ref();

    match b.get(0..8) {
      None => return false,
      Some("        ") => {}
      Some(_) => return false,
    }

    match b.get(8..12) {
      None => return false,
      Some("ELE ") => return true,
      Some("GRP ") => return true,
      Some("NOD ") => return true,
      Some("SEG ") => return true,
      Some("EDG ") => return true,
      Some("MOD ") => return true,
      Some(_) => {}
    }

    match b.get(8..13) {
      None => return false,
      Some("PART ") => return true,
      Some("OGRP ") => return true,
      Some(_) => {}
    }

    match b.get(8..15) {
      None => return false,
      Some("DELNOD ") => return true,
      Some("DELELE ") => return true,
      Some("DELGRP ") => return true,
      Some("END_MOD") => return true,
      Some(_) => {}
    }

    match b.get(8..16) {
      None => return false,
      Some("ELE>NOD ") => return true,
      Some("GRP>NOD ") => return true,
      Some("DELPART ") => return true,
      Some(_) => {}
    }

    match b.get(8..17) {
      None => return false,
      Some("PART>NOD ") => return true,
      Some(_) => {}
    }

    match b.get(8..19) {
      None => return false,
      Some("DELELE>NOD ") => return true,
      Some("DELGRP>NOD ") => return true,
      Some(_) => {}
    }

    match b.get(8..20) {
      None => return false,
      Some("DELPART>NOD ") => return true,
      Some(_) => return false,
    }
  }

  pub fn ended_by<T: AsRef<str>>(&self, line: &T) -> bool {
    let b = line.as_ref();
    match b.get(0..11) {
      Some("        END") => {}
      _ => return false,
    }
    match b.get(11..12) {
      None => return true,
      _ => return false,
    }
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
      false,
      false,
      false,
      true,
      true,
      false,
      false,
      true,
      true,
      true,
    ];
    assert_eq!(v, LINES.iter().map(|l| g.contains(&l)).collect::<Vec<bool>>());
  }

  #[test]
  fn test_ends_ges() {
    let g = GesType::GesNode;
    let v = vec![
      false,
      false,
      false,
      false,
      false,
      false,
      true,
      false,
      false,
      false,
    ];
    assert_eq!(v, LINES.iter().map(|l| g.ended_by(&l)).collect::<Vec<bool>>());
  }

}
