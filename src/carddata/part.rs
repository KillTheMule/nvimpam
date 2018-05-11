//! This modules holds the the global static part [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::ges::GesType::*;
use card::keyword::Keyword::*;
use card::line::Conditional::*;
use card::line::Line::*;
use card::Card;

macro_rules! part3d {
  ($($e: expr),+; $k: expr) => { 
    Card {
      lines: &[
        Provides(&[
          Kw,
          Integer(8),
          Str(8),
          Integer(8),
          Integer(8),
          Integer(8),
          Integer(8)
          ], Int(25..33, 0)),
        Optional(&[Str(4), Str(76)], 0),
        Cells(&[Fixed("NAME"), Str(76)]),
        Cells(&[Float(10), Float(10)]),
        Cells(&[Float(10), Float(10), Float(10)]),
        $( $e ),+ ,
        Cells(&[Fixed("END_PART")]), 
      ],
      ownfold: true,
      keyword: $k
    };
  };
}

pub static PARTSOLID: Card = part3d!(
  Cells(&[Integer(5), Float(10), Float(10), Float(10)]),
  Cells(&[Integer(5), Float(10), Float(10), Float(10)])
  ;PartSolid);

#[cfg(test)]
mod tests {

  const CARD_PARTSOLID: [&'static str; 22] = [
    "$PART Type SOLID",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SOLID       0       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#RT1          XDIR1     YDIR1     ZDIR1",
    "                                        ",
    "$#RT2          XDIR2     YDIR2     ZDIR2",
    "                                        ",
    "$#      ",
    "END_PART",
    "PART  /        1   SOLID       0       0       0       0",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "                                        ",
    "                                        ",
    "END_PART",
  ];

  #[test]
  fn fold_partsolid() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> =
      vec![(2, 14, PartSolid), (15, 21, PartSolid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSOLID);

    assert_eq!(v, foldlist.into_vec());
  }
}
