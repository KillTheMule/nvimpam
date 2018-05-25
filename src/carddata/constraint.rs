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

#[cfg(test)]
mod tests {

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
  fn fold_mtoco() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v = vec![(2, 5, Mtoco), (6, 10, Mtoco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MTOCO);

    assert_eq!(v, foldlist.into_vec());
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
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v = vec![(2, 7, Mtoco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_MTOCO2);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_OTMCO: [&'static str; 5] = [
    "$#         IDOTM  IDNODd  XYZUVW   IMETH  RADIUS   IELIM    ITYP   ALPHA",
    "OTMCO /        1       0  111111       0      0.                        ",
    "$#                                                                         TITLE",
    "NAME Otmco->1                                                                   ",
    "END_OTMCO",
  ];

  #[test]
  fn fold_otmco() {
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v = vec![(1, 4, Otmco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_OTMCO);

    assert_eq!(v, foldlist.into_vec());
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
    use card::keyword::Keyword::*;
    use folds::FoldList;

     let v = vec![(1, 8, Otmco)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_OTMCO2);

    assert_eq!(v, foldlist.into_vec());
  }

}
