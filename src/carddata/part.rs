//! This modules holds the the global static part [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
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

pub static PARTCOS3D: Card = part3d!(
  Cells(&[Blank(10), Float(10), Float(10), Float(10), Float(10), Integer(10)])
  ;PartCos3d);

pub static PARTBSHEL: Card = part3d!(
  Cells(&[Blank(0)])
  ;PartBshel);

pub static PARTTETRA: Card = part3d!(
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10)]),
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10)])
  ; PartTetra);

pub static PARTSPHEL: Card = part3d!(
  Provides(&[Float(10), Float(10), Float(10), Float(10), Integer(5), Integer(5),
             Float(10), Float(10), Integer(5)],
           Number(46..51)),
  Repeat(&[Integer(10), Float(10)], 1)
  ; PartSphel);

#[cfg(test)]
mod tests {
  use card::keyword::Keyword;
  use card::keyword::Keyword::*;

  const CARD_PARTSOLID: [&'static str; 22] = [
    "$PART Type SOLID",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SOLID       1       0       0       0",
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
    "PART  /        1   SOLID       1       0       0       0",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "                                        ",
    "                                        ",
    "END_PART",
  ];

  #[test]
  fn fold_partsolid() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> =
      vec![(2, 14, PartSolid), (15, 21, PartSolid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSOLID);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTSOLID2: [&'static str; 17] = [
    "$PART Type SOLID",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SOLID       1       0       0       0",
    "$#                                                                         TITLE",
    "RMATname",
    "$#",
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
  ];

  #[test]
  fn fold_partsolid2() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 14, PartSolid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSOLID2);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTCOS3D: [&'static str; 13] = [
    "$PART Type COS3D",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   COS3D       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   Blank       THK     XDIR1     YDIR1     ZDIR1     IMETH",
    "                                                            ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partcos3d() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 12, PartCos3d)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTCOS3D);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTBSHEL: [&'static str; 12] = [
    "$PART Type BSHEL",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   BSHEL       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partbshel() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 11, PartBshel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTBSHEL);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTTETRA: [&'static str; 15] = [
    "$PART Type TETRA",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   TETRA       1       0       0       0",
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
  ];

  #[test]
  fn fold_parttetra() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 14, PartTetra)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTTETRA);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTSPHEL: [&'static str; 13] = [
    "$PART Type SPHEL",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SPHEL       0       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RATIO      Hmin      Hmax       ETAINORMNPAIR   ALPHAmg    BETAmg NMON",
    "                                                 0                         ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partsphel() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 12, PartSphel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPHEL);

    assert_eq!(v, foldlist.into_vec());
  }

  const CARD_PARTSPHEL2: [&'static str; 16] = [
    "$PART Type SPHEL",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SPHEL       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RATIO      Hmin      Hmax       ETAINORMNPAIR   ALPHAmg    BETAmg NMON",
    "                                                 3                         ",
    " ",
    " ",
    " ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partsphel2() {
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(2, 15, PartSphel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPHEL2);

    assert_eq!(v, foldlist.into_vec());
  }
}
