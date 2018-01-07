#[derive(Debug)]
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
  use card::keyword::Keyword;

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
    let g = GesType::Gesnode;
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
    let g = GesType::Gesnode;
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

  const GES1: [&'static str; 5] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "        DELGRP>NOD 'nix'",
    "        END",
    "NODE  / ",
  ];

  #[test]
  fn ges_can_be_skipped() {
    let g = ges::GesType::GesNode;
    assert_eq!(
      (Some(3), Some(Keyword::Node), Some(4)),
      g.skip_ges(&mut GES1.iter().enumerate())
    );
  }

  const GES2: [&'static str; 9] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "        END",
    "        DELGRP>NOD 'nix'",
    "        MOD 10234",
    "        NOD 1 23 093402 82",
    "        END_MOD",
    "        DELELE 12",
    "        END",
  ];

  #[test]
  fn ges_can_be_skipped_repeatedly() {
    let g = ges::GesType::GesNode;
    let mut it = GES2.iter().enumerate();
    assert_eq!((Some(2), None, Some(3)), g.skip_ges(&mut it));
    assert_eq!((Some(8), None, None), g.skip_ges(&mut it));
  }

  const GES3: [&'static str; 9] = [
    "        PART 1234",
    "        OGRP 'hausbau'",
    "NODE  /         END",
    "        DELGRP>NOD 'nix'",
    "        MOD 10234",
    "        NOD 1 23 093402 82",
    "        END_MOD",
    "Whatever",
    "        END",
  ];

  #[test]
  fn ges_ends_without_end() {
    let g = ges::GesType::GesNode;
    let mut it = GES3.iter().enumerate();
    assert_eq!((Some(1), Some(Keyword::Node), Some(2)), g.skip_ges(&mut it));
    assert_eq!((Some(6), None, Some(7)), g.skip_ges(&mut it));
  }

  const GES4: [&'static str; 2] = ["wupdiwup", "NODE  / "];

  #[test]
  fn ges_can_skip_nothing() {
    let g = ges::GesType::GesNode;
    let mut it = GES4.iter().enumerate();
    assert_eq!((None, None, Some(0)), g.skip_ges(&mut it));
    assert_eq!((Some(0), Some(Keyword::Node), Some(1)), g.skip_ges(&mut it));
  }
}
