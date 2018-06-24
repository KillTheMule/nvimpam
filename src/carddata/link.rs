//! This modules holds the the global static link [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::ges::GesType::*;
use card::keyword::Keyword::*;
use card::line::Line::*;
use card::Card;

pub static ELINK: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8), Integer(8)]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Elink,
};

pub static LLINK: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8), Integer(8)]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Llink,
};

pub static SLINK: Card = Card {
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
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Slink,
};

pub static PLINK: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Plink,
};

pub static TIED: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesNode),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Tied,
};

#[cfg(test)]
mod tests {

  const CARD_ELINK: [&'static str; 13] = [
    "$ELINK Element",
    "ELINK /                                 ",
    "        PART ",
    "        END",
    "$ELINK Element",
    "ELINK /                                 ",
    "        PART ",
    "        END",
    "$ELINK Element",
    "ELINK /                                 ",
    "        PART ",
    "        PART ",
    "        END",
  ];

  #[test]
  fn fold_elink() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 12, Elink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_ELINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_LLINK: [&'static str; 23] = [
    "$LLINK Element",
    "LLINK /                                 ",
    "        PART 1",
    "        PART 23",
    "        PART 45",
    "        END",
    "$LLINK Element",
    "LLINK /                                 ",
    "        PART ",
    "        GRP ",
    "        END",
    "LLINK /                                 ",
    "        OGRP ",
    "        GRP ",
    "        END",
    "LLINK /                                 ",
    "        PART ",
    "        ELE>NOD ",
    "        END",
    "LLINK /                                 ",
    "        DELGRP ",
    "        ELE ",
    "        END",
  ];

  #[test]
  fn fold_llink() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 22, Llink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_LLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_SLINK: [&'static str; 6] = [
    "$SLINK Element",
    "SLINK /                                 ",
    "        PART 1",
    "        PART 23",
    "        PART 45",
    "        END",
  ];

  #[test]
  fn fold_slink() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 5, Slink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_SLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PLINK: [&'static str; 9] = [
    "$PLINK Element",
    "PLINK /                                 ",
    "        PART 47",
    "        END",
    "PLINK /                                 ",
    "        PART 45",
    "        PART 45",
    "        END",
    "#PLINK",
  ];

  #[test]
  fn fold_plink() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 7, Plink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_TIED: [&'static str; 22] = [
    "$TIED Element",
    "TIED  /                         ",
    "NAME TIED",
    "        PART ",
    "        PART ",
    "        PART ",
    "        END",
    "        PART ",
    "        END",
    "TIED  /                         ",
    "NAME TIED",
    "        PART ",
    "        END",
    "        PART ",
    "        PART ",
    "        END",
    "TIED  /                         ",
    "NAME TIED",
    "        PART ",
    "        END",
    "        PART ",
    "        END",
  ];

  #[test]
  fn fold_tied() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 21, Tied)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_TIED);

    assert_eq!(v, foldlist.into_vec(1));
  }

}
