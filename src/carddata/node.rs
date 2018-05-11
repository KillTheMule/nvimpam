//! This modules holds the the global static node [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::ges::GesType::*;
use card::keyword::Keyword::*;
use card::line::Conditional::*;
use card::line::Line::*;
use card::Card;

pub static NODE: Card = Card {
  lines: &[Cells(&[Kw, Integer(16), Float(16), Float(16), Float(16)])],
  ownfold: false,
  keyword: Node,
};

pub static CNODE: Card = Card {
  lines: &[Cells(&[Kw, Integer(16), Float(16), Float(16), Float(16)])],
  ownfold: false,
  keyword: Cnode,
};

pub static MASS: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Float(16), Float(16), Float(16)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[Float(16), Float(16), Float(16)]),
    Provides(
      &[Blank(8), Float(16), Float(16), Float(16), Blank(24), Cont],
      RelChar(80, '&'),
    ),
    Optional(&[Blank(8), Float(16), Float(16), Float(16)], 0),
    Ges(GesNode),
  ],
  ownfold: true,
  keyword: Mass,
};

pub static NSMAS: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Float(16), Float(16), Float(16), Float(16)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
  keyword: Nsmas,
};

pub static NSMAS2: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Float(16), Float(16), Float(16), Float(16)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
  keyword: Nsmas2,
};

#[cfg(test)]
mod tests {

  const CARD_NSMAS: [&'static str; 7] = [
    "$ NSMAS - Nonstructural mass",
    "$#       IDNODMS            MASS            MLEN            MARE            MVOL",
    "NSMAS /        1              0.                                                ",
    "$#                                                                         TITLE",
    "NAME NSMAS / ->1                                                                ",
    "        ELE ",
    "        END",
  ];

  #[test]
  fn fold_nsmas() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let mut it = CARD_NSMAS.iter().enumerate();
    let _ = it.next();
    let _ = it.next();
    let _ = it.next();

    let v = vec![(2, 6, Nsmas)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_NSMAS);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_MASS: [&'static str; 10] = [
    "$ MASS Card",
    "$#         IDNOD    IFRA   Blank            DISr            DISs            DISt",
    "MASS  /        0       0                                                        ",
    "$#                                                                         TITLE",
    "NAME MASS  / ->1                                                                ",
    "$# BLANK              Mx              My              Mz",
    "                                                        ",
    "$# BLANK              Ix              Iy              Iz                   Blank",
    "                                                                                ",
    "        END",
  ];

  #[test]
  fn fold_mass() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 9, Mass)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MASS);

    assert_eq!(v, foldlist.into_vec());
  }

}
