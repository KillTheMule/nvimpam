//! This modules holds the the global static element [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::keyword::Keyword::*;
use card::line::Line::*;
use card::Card;

pub static SOLID: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8)]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
  ],
  ownfold: false,
  keyword: Solid,
};

pub static HEXA20: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8)]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[Blank(16), Integer(8), Integer(8), Integer(8), Integer(8)]),
  ],
  ownfold: false,
  keyword: Hexa20,
};

pub static PENT15: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8)]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
  ],
  ownfold: false,
  keyword: Pent15,
};

pub static PENTA6: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Penta6,
};

pub static TETR10: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8)]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[Blank(16), Integer(8), Integer(8)]),
  ],
  ownfold: false,
  keyword: Tetr10,
};

pub static BAR: Card = Card {
  lines: &[Cells(&[Kw, Integer(8), Integer(8), Integer(8), Integer(8)])],
  ownfold: false,
  keyword: Bar,
};

pub static BSHEL: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8)]),
    Cells(&[
      Blank(16),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
  ],
  ownfold: false,
  keyword: Bshel,
};

pub static TSHEL: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Blank(8),
    Float(8),
  ])],
  ownfold: false,
  keyword: Tshel,
};

pub static SHELL: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Blank(8),
    Float(8),
  ])],
  ownfold: false,
  keyword: Shell,
};

pub static SHEL6: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Shel6,
};

pub static SHEL8: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[Blank(24), Integer(8), Integer(8), Integer(8), Integer(8)]),
  ],
  ownfold: false,
  keyword: Shel8,
};

pub static MEMBR: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Blank(8),
    Float(8),
    Float(8),
  ])],
  ownfold: false,
  keyword: Membr,
};

pub static BEAM: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Blank(10),
      Binary(6),
      Blank(2),
      Binary(6),
      Integer(8),
    ]),
    Cells(&[Blank(8), Float(8), Float(8), Float(8), Float(8), Float(8)]),
    Cells(&[Blank(8), Float(8), Float(8), Float(8), Float(8), Float(8)]),
  ],
  ownfold: false,
  keyword: Beam,
};

pub static SPRGBM: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Sprgbm,
};

pub static TETR4: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Tetr4,
};

pub static SPRING: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    IntegerorBlank(8),
  ])],
  ownfold: false,
  keyword: Spring,
};

pub static JOINT: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    IntegerorBlank(8),
    Float(8),
    Float(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Joint,
};

pub static KJOIN: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Str(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Float(8),
    ]),
    Cells(&[Blank(10), Binary(6)]),
  ],
  ownfold: false,
  keyword: Kjoin,
};

pub static MTOJNT: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Str(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Cells(&[Blank(10), Binary(6)]),
  ],
  ownfold: false,
  keyword: Mtojnt,
};

pub static SPHEL: Card = Card {
  lines: &[Cells(&[Kw, Integer(8), Integer(8), Integer(8), Float(8)])],
  ownfold: false,
  keyword: Sphel,
};

pub static SPHELO: Card = Card {
  lines: &[Cells(&[Kw, Integer(8), Integer(8), Integer(8), Float(8)])],
  ownfold: false,
  keyword: Sphelo,
};

pub static GAP: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
    Integer(8),
  ])],
  ownfold: false,
  keyword: Gap,
};

pub static IMPMA: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Str(8), Str(8), Str(8), Integer(8)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[Blank(8), Str(76)]),
  ],
  ownfold: false,
  keyword: Impma,
};

#[cfg(test)]
mod tests {

  const CARD_SOLID: [&'static str; 15] = [
    "$SOLID Element",
    "SOLID /                                                                        ",
    "                                                                                ",
    "$SOLID Element",
    "SOLID /                                                                        ",
    "                                                                                ",
    "$SOLID Element",
    "SOLID /                                                                        ",
    "                                                                                ",
    "SOLID /                                                                        ",
    "                                                                                ",
    "SOLID /                                                                        ",
    "                                                                                ",
    "SOLID /                                                                        ",
    "                                                                                ",
  ];

  #[test]
  fn fold_solid() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 14, Solid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SOLID);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_HEXA20: [&'static str; 18] = [
    "$HEXA20 Element",
    "HEXA20/                 ",
    "                                                                                ",
    "                                                                                ",
    "                                                ",
    "HEXA20/                 ",
    "                                                                                ",
    "                                                                                ",
    "                                                ",
    "HEXA20/                 ",
    "                                                                                ",
    "                                                                                ",
    "                                                ",
    "$HEXA20 Element",
    "HEXA20/                 ",
    "                                                                                ",
    "                                                                                ",
    "                                                ",
  ];

  #[test]
  fn fold_hexa20() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 17, Hexa20)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_HEXA20);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PENT15: [&'static str; 14] = [
    "$PENTA15 Element",
    "PENT15/                 ",
    "                                                                                ",
    "                                                                        ",
    "PENT15/                 ",
    "                                                                                ",
    "                                                                        ",
    "PENT15/                 ",
    "                                                                                ",
    "                                                                        ",
    "$PENTA15 Element",
    "PENT15/                 ",
    "                                                                                ",
    "                                                                        ",
  ];

  #[test]
  fn fold_pent15() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 13, Pent15)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PENT15);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PENTA6: [&'static str; 9] = [
    "$PENTA6 Element ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
    "PENTA6/                                                                 ",
  ];

  #[test]
  fn fold_penta6() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Penta6)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PENTA6);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_TETR10: [&'static str; 13] = [
    "$TETR10 Element",
    "TETR10/                 ",
    "                                                                                ",
    "                                ",
    "TETR10/                 ",
    "                                                                                ",
    "                                ",
    "TETR10/                 ",
    "                                                                                ",
    "                                ",
    "TETR10/                 ",
    "                                                                                ",
    "                                ",
  ];

  #[test]
  fn fold_tetr10() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 12, Tetr10)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_TETR10);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_TETR4: [&'static str; 6] = [
    "$TETR4 Element",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
  ];

  #[test]
  fn fold_tetr4() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 5, Tetr4)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_TETR4);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_BSHEL: [&'static str; 6] = [
    "BSHEL /                 ",
    "                                                                                ",
    "BSHEL /                 ",
    "                                                                                ",
    "BSHEL /                 ",
    "                                                                                ",
  ];

  #[test]
  fn fold_bshel() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(0, 5, Bshel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_BSHEL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_TSHEL: [&'static str; 9] = [
    "$TSHEL Element",
    "TSHEL /                                                                 ",
    "TSHEL /                                                                 ",
    "$TSHEL Element",
    "TSHEL /                                                                 ",
    "TSHEL /                                                                 ",
    "TSHEL /                                                                 ",
    "$TSHEL Element",
    "TSHEL /                                                                 ",
  ];

  #[test]
  fn fold_tshel() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Tshel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_TSHEL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SHELL: [&'static str; 9] = [
    "$SHELL Element",
    "SHELL /                                                                 ",
    "SHELL /                                                                 ",
    "$SHELL Element",
    "SHELL /                                                                 ",
    "SHELL /                                                                 ",
    "SHELL /                                                                 ",
    "$SHELL Element",
    "SHELL /                                                                 ",
  ];

  #[test]
  fn fold_shell() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Shell)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SHELL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SHEL6: [&'static str; 9] = [
    "$SHEL6 Element",
    "SHEL6 /                                                                 ",
    "SHEL6 /                                                                 ",
    "$SHEL6 Element",
    "SHEL6 /                                                                 ",
    "SHEL6 /                                                                 ",
    "SHEL6 /                                                                 ",
    "$SHEL6 Element",
    "SHEL6 /                                                                 ",
  ];

  #[test]
  fn fold_shel6() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Shel6)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SHEL6);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SHEL8: [&'static str; 8] = [
    "$SHEL8 Element",
    "SHEL8 /                                                 ",
    "                                                        ",
    "SHEL8 /                                                 ",
    "                                                        ",
    "$SHEL8 Element",
    "SHEL8 /                                                 ",
    "                                                        ",
  ];

  #[test]
  fn fold_shel8() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 7, Shel8)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SHEL8);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_MEMBR: [&'static str; 9] = [
    "$MEMBR Element",
    "MEMBR /                                                                 ",
    "MEMBR /                                                                 ",
    "$MEMBR Element",
    "MEMBR /                                                                 ",
    "MEMBR /                                                                 ",
    "MEMBR /                                                                 ",
    "$MEMBR Element",
    "MEMBR /                                                                 ",
  ];

  #[test]
  fn fold_membr() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Membr)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MEMBR);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_BEAM: [&'static str; 14] = [
    "$BEAM Element",
    "BEAM  /                                                                         ",
    "                                                ",
    "                                                ",
    "$BEAM Element",
    "BEAM  /                                                                         ",
    "                                                ",
    "                                                ",
    "BEAM  /                                                                         ",
    "                                                ",
    "                                                ",
    "BEAM  /                                                                         ",
    "                                                ",
    "                                                ",
  ];

  #[test]
  fn fold_beam() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 13, Beam)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_BEAM);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SPRGBM: [&'static str; 9] = [
    "$SPRGBM Element",
    "SPRGBM/                                                                 ",
    "SPRGBM/                                                                 ",
    "$SPRGBM Element",
    "SPRGBM/                                                                 ",
    "SPRGBM/                                                                 ",
    "SPRGBM/                                                                 ",
    "$SPRGBM Element",
    "SPRGBM/                                                                 ",
  ];

  #[test]
  fn fold_sprgbm() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 8, Sprgbm)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SPRGBM);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_BAR: [&'static str; 6] = [
    "$BAR  Element",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
  ];

  #[test]
  fn fold_bar() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 5, Bar)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_BAR);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SPRING: [&'static str; 7] = [
    "$SPRING  Element",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
    "$SPRING  Element",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
  ];

  #[test]
  fn fold_spring() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 6, Spring)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SPRING);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_JOINT: [&'static str; 3] = [
    "JOINT /                                                 ",
    "JOINT /                                                 ",
    "JOINT /                                                 ",
  ];

  #[test]
  fn fold_joint() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(0, 2, Joint)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_JOINT);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_KJOIN: [&'static str; 8] = [
    "$KJOIN Element",
    "KJOIN /                                                                 ",
    "                ",
    "KJOIN /                                                                 ",
    "                ",
    "$KJOIN Element",
    "KJOIN /                                                                 ",
    "                ",
  ];

  #[test]
  fn fold_kjoin() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 7, Kjoin)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_KJOIN);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_MTOJNT: [&'static str; 7] = [
    "$MTOJNTElement",
    "MTOJNT/                                                                 ",
    "                ",
    "MTOJNT/                                                                 ",
    "                ",
    "MTOJNT/                                                                 ",
    "                ",
  ];

  #[test]
  fn fold_mtojnt() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 6, Mtojnt)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MTOJNT);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SPHEL: [&'static str; 6] = [
    "SPHEL /                                                                 ",
    "SPHEL /                                                                 ",
    "$SPHEL Element",
    "SPHEL /                                                                 ",
    "$SPHEL Element",
    "SPHEL /                                                                 ",
  ];

  #[test]
  fn fold_sphel() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(0, 5, Sphel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SPHEL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SPHELO: [&'static str; 2] = [
    "SPHELO/                                                                 ",
    "SPHELO/                                                                 ",
  ];

  #[test]
  fn fold_sphelo() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(0, 1, Sphelo)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SPHELO);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_GAP: [&'static str; 6] = [
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
    "#COMMENT",
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
  ];

  #[test]
  fn fold_gap() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(0, 5, Gap)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_GAP);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_IMPMA: [&'static str; 14] = [
    "$IMPMA Super Element Matrix Import",
    "$#       IDIMPMAQUALIFY1QUALIFY2QUALIFY3  IMATYP    ISEL",
    "IMPMA /        1                               0         ",
    "$#                                                                         TITLE",
    "NAME IMPMA / ->1                                                                ",
    "$#                                                                       FNAMEma",
    "                                                                                ",
    "$IMPMA Super Element Matrix Import",
    "$#       IDIMPMAQUALIFY1QUALIFY2QUALIFY3  IMATYP    ISEL",
    "IMPMA /        1                               0         ",
    "$#                                                                         TITLE",
    "NAME IMPMA / ->1                                                                ",
    "$#                                                                       FNAMEma",
    "                                                                                ",
  ];

  #[test]
  fn fold_impma() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 13, Impma)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_IMPMA);

    assert_eq!(v, foldlist.into_vec(1));
  }
}
