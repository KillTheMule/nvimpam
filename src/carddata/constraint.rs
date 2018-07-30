//! This modules holds the the global static constraint [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::ges::GesType::*;
use card::keyword::Keyword::*;
use card::line::Conditional::*;
use card::line::Line::*;
use card::Card;

pub static MTOCO: Card = Card {
  lines: &[
    Provides(
      &[
        Kw,
        Integer(8),
        Integer(8),
        Binary(6),
        Integer(8),
        Integer(8),
        Integer(8),
        Float(8),
      ],
      Int(41..49, 1),
    ),
    Cells(&[Fixed("NAME"), Str(76)]),
    Optional(
      &[
        Integer(8),
        Float(8),
        Float(8),
        Float(8),
        Float(8),
        Float(8),
        Float(8),
        Float(8),
        Integer(8),
        Integer(8),
      ],
      0,
    ),
    Ges(GesNode),
  ],
  ownfold: true,
  keyword: Mtoco,
};

pub static OTMCO: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Binary(6),
      Integer(8),
      Float(8),
      Integer(8),
      Integer(8),
      Float(8),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Block(
      &[Cells(&[Blank(8), Fixed("WEIGHT"), Float(58)]), Ges(GesNode)],
      "END_OTMCO",
    ),
  ],

  ownfold: true,
  keyword: Otmco,
};

pub static RBODY0: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Blank(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Blank(8),
      Integer(8),
      Blank(8),
      Float(8),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesNode),
  ],

  ownfold: true,
  keyword: Rbody0,
};

pub static RBODY1: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Blank(8),
      Integer(8),
      Integer(8),
      Blank(16),
      Integer(8),
      Blank(8),
      Float(8),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[
      Blank(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Integer(8),
      Float(8),
      Float(8),
    ]),
    Ges(GesNode),
  ],

  ownfold: true,
  keyword: Rbody1,
};

pub static RBODY2: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Blank(8),
      Integer(8),
      Integer(8),
      Blank(16),
      Integer(8),
      Blank(8),
      Float(8),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[
      Blank(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Integer(8),
    ]),
    Ges(GesNode),
  ],

  ownfold: true,
  keyword: Rbody2,
};

pub static RBODY3: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Blank(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Blank(8),
      Integer(8),
      Float(8),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[
      Integer(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
      Float(8),
    ]),
    Ges(GesNode),
  ],

  ownfold: true,
  keyword: Rbody3,
};

#[cfg(test)]
mod tests {
  use card::keyword::Keyword::*;
  use folds::FoldList;

  const CARD_MTOCO: [&'static str; 11] = [
    "$Regular MTOCO",
    "$#         IDMTO  IDNBLANKXYZUVW   IFRA1   ITMTO   ISENS   ALPHA  DOFTYP",
    "MTOCO /        1       0  111111       0       0       0                ",
    "$#                                                                         TITLE",
    "NAME MTOCO / ->1                                                                ",
    "        END",
    "MTOCO /        1       0  111111       0       0       0                ",
    "$#                                                                         TITLE",
    "NAME MTOCO / ->1                                                                ",
    "        PART 123",
    "        END",
  ];

  #[test]
  fn fold_mtoco1() {
    let v = vec![(2, 5, Mtoco), (6, 10, Mtoco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MTOCO);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_MTOCO2: [&'static str; 8] = [
    "$MTOCO mit User Imposed Mass and Intertia",
    "$#         IDMTO  IDNBLANKXYZUVW   IFRA1   ITMTO                  DOFTYP",
    "MTOCO /        1       0  111111       0       1                        ",
    "$#                                                                         TITLE",
    "NAME MTOCO / ->1                                                                ",
    "$# IT1FL    MMTO      I1      I2      I3   BLANK   BLANK   BLANK   IFRA2    NCOG",
    "       0                                                               0       0",
    "        END",
  ];

  #[test]
  fn fold_mtoco2() {
    let v = vec![(2, 7, Mtoco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MTOCO2);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_OTMCO: [&'static str; 5] = [
    "$#         IDOTM  IDNODd  XYZUVW   IMETH  RADIUS   IELIM    ITYP   ALPHA",
    "OTMCO /        1       0  111111       0      0.                        ",
    "$#                                                                         TITLE",
    "NAME Otmco->1                                                                   ",
    "END_OTMCO",
  ];

  #[test]
  fn fold_otmco1() {
    let v = vec![(1, 4, Otmco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_OTMCO);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_OTMCO2: [&'static str; 9] = [
    "$#         IDOTM  IDNODd  XYZUVW   IMETH  RADIUS   IELIM    ITYP   ALPHA",
    "OTMCO /        1       0  111111       0      0.                        ",
    "$#                                                                         TITLE",
    "NAME Otmco->1                                                                   ",
    " ",
    " ",
    " ",
    " ",
    "END_OTMCO",
  ];

  #[test]
  fn fold_otmco2() {
    let v = vec![(1, 8, Otmco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_OTMCO2);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_RBODY0: [&'static str; 6] = [
    "$RBODY Type 0",
    "$#          IDRB   BLANK    ITRBIDNODcog    ICOG           ISENS    IFRA     HRB",
    "RBODY /        1               0       0                       0       0        ",
    "$#                                                                         TITLE",
    "NAME RBODY / ->1                                                                ",
    "        END",
  ];

  #[test]
  fn fold_rbody0() {
    let v = vec![(2, 5, Rbody0)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_RBODY0);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_RBODY1: [&'static str; 10] = [
    "$RBODY Type 1",
    "$#          IDRB   BLANK    ITRBIDNODcog                   ISENS    IFRA     HRB",
    "RBODY /        1               1       0                       0       0        ",
    "$#                                                                         TITLE",
    "NAME RBODY / ->1                                                                ",
    "$# BLANK   TFAIL   FAILD  AFAILN  AFAILS      A1      A2    INTF      D1      D2",
    "              0.      0.      0.      0.      0.      0.       0      0.      0.",
    "        PART 123",
    "        NOD 1",
    "        END",
  ];

  #[test]
  fn fold_rbody1() {
    let v = vec![(2, 9, Rbody1)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_RBODY1);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_RBODY2: [&'static str; 7] = [
    "$#          IDRB   BLANK    ITRBIDNODcog                   ISENS    IFRA     HRB",
    "RBODY /        1               2       0                       0       0        ",
    "$#                                                                         TITLE",
    "NAME RBODY / ->1                                                                ",
    "$# BLANK   TFAIL   FAILD  AFAILN  AFAILS      A1      A2    INTF",
    "              0.      0.      0.      0.      0.      0.       0",
    "        END",
  ];

  #[test]
  fn fold_rbody2() {
    let v = vec![(1, 6, Rbody2)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_RBODY2);

    assert_eq!(v, foldlist.to_vec(1));
  }

  const CARD_RBODY3: [&'static str; 8] = [
    "$RBODY Type 3",
    "$#          IDRB   BLANK    ITRBIDNODcog  IDNOD1  IDNOD2   ISENS    IFRA     HRB",
    "RBODY /        1               3       0       0       0       0       0        ",
    "$#                                                                         TITLE",
    "NAME RBODY / ->1                                                                ",
    "$# IT3FL     Mrb      I1      I2      I3   BLANK   BLANK   BLANK",
    "              0.      0.      0.      0.                        ",
    "        END",
  ];

  #[test]
  fn fold_rbody3() {
    let v = vec![(2, 7, Rbody3)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_RBODY3);

    assert_eq!(v, foldlist.to_vec(1));
  }
}
