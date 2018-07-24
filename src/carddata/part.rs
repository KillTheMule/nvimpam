//! This modules holds the the global static part [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::keyword::Keyword::*;
use card::line::Conditional::*;
use card::line::Line::*;
use card::Card;

macro_rules! part {
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

// Part 3D

pub static PARTSOLID: Card = part!(
  Cells(&[Integer(5), Float(10), Float(10), Float(10)]),
  Cells(&[Integer(5), Float(10), Float(10), Float(10)])
  ;PartSolid);

pub static PARTCOS3D: Card = part!(
  Cells(&[Blank(10), Float(10), Float(10), Float(10), Float(10), Integer(10)])
  ;PartCos3d);

pub static PARTBSHEL: Card = part!(
  Cells(&[Blank(0)])
  ;PartBshel);

pub static PARTTETRA: Card = part!(
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10)]),
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10)])
  ; PartTetra);

pub static PARTSPHEL: Card = part!(
  Provides(&[Float(10), Float(10), Float(10), Float(10), Integer(5), Integer(5),
             Float(10), Float(10), Integer(5)],
           Number(46..51)),
  Repeat(&[Integer(10), Float(10)], 1)
  ; PartSphel);

// PART 2D

pub static PARTTSHEL: Card = part!(
  Cells(&[Float(10), Integer(5)])
  ;PartTshel);

pub static PARTSHELL: Card = part!(
  Cells(&[Float(10), Integer(5), Float(10), Integer(5)]),
  Cells(&[Integer(5), Float(10), Float(10), Float(10), Float(10)])
  ;PartShell);

pub static PARTMEMBR: Card = part!(
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10), Float(10),
          Float(10), Float(10), Float(10),]),
  Cells(&[Integer(5), Blank(5), Float(10), Float(10), Float(10), Float(10),
          Float(10), Float(10), Float(10),])
  ;PartMembr);

// PART 1D

pub static PARTBAR: Card = part!(
  Cells(&[Float(10)])
  ;PartBar);

pub static PARTBEAM: Card = part!(
  Cells(&[Float(10), Float(10), Float(10), Float(10), Float(10), Blank(5),
          Integer(5), Float(10)]),
  Cells(&[Float(10), Float(10), Float(10), Blank(4), Binary(6), Blank(4),
          Binary(6)]),
  Cells(&[Float(10), Float(10), Float(10), Float(10), Float(10)]),
  Cells(&[Float(10), Float(10), Float(10), Float(10), Float(10)]),
  Provides(&[Integer(5), Integer(5), Float(10), Float(10), Float(10)],
             Number(6..11)),
  Repeat(&[Float(10), Float(10), Float(10)], 1)
  ;PartBeam);

pub static PARTSPRING: Card = part!(
  Cells(&[Blank(0)])
  ;PartSpring);

pub static PARTSPRGBM: Card = part!(
  Cells(&[Blank(0)])
  ;PartSprgbm);

pub static PARTMBSPR: Card = part!(
  Cells(&[Blank(0)])
  ;PartMbspr);

pub static PARTJOINT: Card = part!(
  Cells(&[Blank(0)])
  ;PartJoint);

pub static PARTKJOIN: Card = part!(
  Cells(&[Blank(0)])
  ;PartKjoin);

pub static PARTMTOJNT: Card = part!(
  Cells(&[Blank(0)])
  ;PartMtojnt);

pub static PARTMBKJN: Card = part!(
  Cells(&[Blank(0)])
  ;PartMbkjn);

pub static PARTTIED: Card = part!(
  Provides(&[Float(10), Blank(60), Integer(10)], Int(71..81,1)),
  Optional(&[Integer(5), Blank(5), Float(10), Float(10), Float(10), Float(10)],
           1)
  ;PartTied);

pub static PARTSLINK: Card = part!(
  Provides(&[Float(10), Blank(60), Integer(10)], Int(71..81,1)),
  Optional(&[Integer(5), Blank(5), Float(10), Float(10), Float(10), Float(10)],
           1)
  ;PartSlink);

pub static PARTELINK: Card = part!(
  Provides(&[Float(10), Float(10), Blank(50), Integer(10)], Int(71..81,1)),
  Optional(&[Integer(5), Blank(5), Float(10), Float(10), Float(10), Float(10)],
           1)
  ;PartElink);

pub static PARTLLINK: Card = part!(
  Cells(&[Float(10), Float(10), Float(10), Integer(10)])
  ;PartLlink);

pub static PARTPLINK: Card = part!(
  Cells(&[Float(10), Integer(10), Float(10), Integer(10), Float(10), Float(10),
          Integer(10)])
  ;PartPlink);

pub static PARTGAP: Card = part!(
  Cells(&[Blank(10), Float(10)])
  ;PartGap);

#[cfg(test)]
mod tests {
  use card::keyword::Keyword::*;
  use folds::FoldList;

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
    let v = vec![(2, 14, PartSolid), (15, 21, PartSolid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSOLID);

    assert_eq!(v, foldlist.into_vec(1));
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
    let v = vec![(2, 14, PartSolid)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSOLID2);

    assert_eq!(v, foldlist.into_vec(1));
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
    let v = vec![(2, 12, PartCos3d)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTCOS3D);

    assert_eq!(v, foldlist.into_vec(1));
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
    let v = vec![(2, 11, PartBshel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTBSHEL);

    assert_eq!(v, foldlist.into_vec(1));
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
    let v = vec![(2, 14, PartTetra)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTTETRA);

    assert_eq!(v, foldlist.into_vec(1));
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
    let v = vec![(2, 12, PartSphel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPHEL);

    assert_eq!(v, foldlist.into_vec(1));
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
  fn fold_partshphel2() {
    let v = vec![(2, 15, PartSphel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPHEL2);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTTSHEL: [&'static str; 12] = [
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   TSHEL       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       H NINT",
    "               ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_parttshel() {
    let v = vec![(1, 11, PartTshel)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTTSHEL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTSHELL: [&'static str; 15] = [
    "$PART Type SHELL",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SHELL       2       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       H NINT    OFFSETNTDOF",
    "              5               ",
    "$#ORTBLANK      XDIR      YDIR      ZDIR",
    "    0                                   ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partshell() {
    let v = vec![(2, 14, PartShell)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSHELL);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTSHELL2: [&'static str; 11] = [
    "PART  /    10100SHELL   38103600                                                ",
    "NAME abdcd",
    "#   DTELIM|   TSCALF|",
    "                    ",
    "#    TCONT|   EPSINI| COULFRIC|",
    "      1.75                    ",
    "#        h|NINT|         |NTHD|",
    "      1.75    5               ",
    "#",
    "",
    "END_PART",
  ];

  #[test]
  fn fold_partshell2() {
    let v = vec![(0, 10, PartShell)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSHELL2);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTMEMBR: [&'static str; 17] = [
    "$PART Type MEMBR",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   MEMBR       0       2       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       H     NDOFs",
    "                    ",
    "$#RT1            VX1       VY1       VZ1   ALPHof1       TX1       TY1       TZ1",
    "                                                                                ",
    "$#RT2            VX2       VY2       VZ2   ALPHof2       TX2       TY2       TZ2",
    "                                                                                ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partmembr() {
    let v = vec![(2, 16, PartMembr)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTMEMBR);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTBAR: [&'static str; 13] = [
    "$PART Type BAR",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1     BAR       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       A",
    "          ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partbar() {
    let v = vec![(2, 12, PartBar)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTBAR);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTBEAM: [&'static str; 20] = [
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1    BEAM       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       A      Ashs        Is        It        Ir      ITPR      Asht",
    "                                                           0          ",
    "$#     Ist      COGs      COGt    RT1RR1    RT2RR2",
    "                                                  ",
    "$#  ALPHA1     BETA1    GAMMA1      KSI1      ETA1",
    "                                                  ",
    "$#  ALPHA2     BETA2    GAMMA2      KSI2      ETA2",
    "                                                  ",
    "$#SEC NIPS",
    "    0    0",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partbeam() {
    let v = vec![(1, 19, PartBeam)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTBEAM);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTBEAM2: [&'static str; 23] = [
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1    BEAM       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#       A      Ashs        Is        It        Ir      ITPR      Asht",
    "                                                           0          ",
    "$#     Ist      COGs      COGt    RT1RR1    RT2RR2",
    "                                                  ",
    "$#  ALPHA1     BETA1    GAMMA1      KSI1      ETA1",
    "                                                  ",
    "$#  ALPHA2     BETA2    GAMMA2      KSI2      ETA2",
    "                                                  ",
    "$#SEC NIPS",
    "    0    3",
    " ",
    " ",
    " ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partbeam2() {
    let v = vec![(1, 22, PartBeam)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTBEAM2);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTSPRING: [&'static str; 12] = [
    "$PART Type SPRING",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1  SPRING       1       0       0       0",
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
  fn fold_partspring() {
    let v = vec![(2, 11, PartSpring)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPRING);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTSPRGBM: [&'static str; 10] = [
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1  SPRGBM       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partsprgbm() {
    let v = vec![(1, 9, PartSprgbm)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSPRGBM);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTMBSPR: [&'static str; 9] = [
    "$PART Type MBSPR",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   MBSPR       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "",
    "END_PART",
  ];

  #[test]
  fn fold_partmbspr() {
    let v = vec![(2, 8, PartMbspr)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTMBSPR);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTJOINT: [&'static str; 12] = [
    "$PART Type JOINT",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   JOINT       1       0       0       0",
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
  fn fold_partjoint() {
    let v = vec![(2, 11, PartJoint)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTJOINT);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTKJOIN: [&'static str; 6] = [
    "PART  /        1   KJOIN       1       0       0       0",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "",
    "END_PART",
  ];

  #[test]
  fn fold_partkjoin() {
    let v = vec![(0, 5, PartKjoin)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTKJOIN);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTMTOJNT: [&'static str; 13] = [
  "$PART Type MTOJNT",
  "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
  "PART  /        1  MTOJNT       1       0       0       0",
  "$#                                                                         TITLE",
  "NAME PART_1                                                                     ",
  "$#  DTELIM    TSCALF   DTRATIO",
  "                              ",
  "$#   TCONT    EPSINI  COULFRIC",
  "                              ",
  "$#",
  "",
  "$#      ",
  "END_PART",
  ];

  #[test]
  fn fold_partmtojnt() {
    let v = vec![(2, 12, PartMtojnt)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTMTOJNT);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTMBKJN: [&'static str; 12] = [
    "$PART Type MBKJN",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   MBKJN       1       0       0       0",
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
  fn fold_partmbkjn() {
    let v = vec![(2, 11, PartMbkjn)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTMBKJN);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTTIED: [&'static str; 11] = [
    "PART  /        1    TIED       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RDIST                                                       ITSSR     INEXT",
    "                                                                                ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_parttied() {
    let v = vec![(0, 10, PartTied)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTTIED);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTTIED2: [&'static str; 12] = [
    "PART  /        1    TIED       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RDIST                                                       ITSSR     INEXT",
    "                                                                               1",
    " ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_parttied2() {
    let v = vec![(0, 11, PartTied)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTTIED2);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTSLINK: [&'static str; 13] = [
    "$PART Type SLINK",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   SLINK       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RDIST                                                       BLANK     INEXT",
    "                                                                                ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partslink() {
    let v = vec![(2, 12, PartSlink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTSLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTELINK: [&'static str; 11] = [
    "$PART Type ELINK",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   ELINK       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "                              ",
    "                                                                                ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partelink() {
    let v = vec![(2, 10, PartElink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTELINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTLLINK: [&'static str; 12] = [
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   LLINK       1       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   RSEAR     DISPW     WIDTH    NGWDTH",
    "                                        ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partllink() {
    let v = vec![(1, 11, PartLlink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTLLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTPLINK: [&'static str; 9] = [
    "$PART Type PLINK",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1   PLINK       1       0       0       0",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "                                                                                ",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partplink() {
    let v = vec![(2, 8, PartPlink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTPLINK);

    assert_eq!(v, foldlist.into_vec(1));
  }

  const CARD_PARTGAP: [&'static str; 13] = [
    "$PART Type GAP",
    "$#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT",
    "PART  /        1     GAP       0       0       0       0",
    "$#                                                                         TITLE",
    "NAME PART_1                                                                     ",
    "$#  DTELIM    TSCALF   DTRATIO",
    "                              ",
    "$#   TCONT    EPSINI  COULFRIC",
    "                              ",
    "$#   Blank         d",
    "                  0.",
    "$#      ",
    "END_PART",
  ];

  #[test]
  fn fold_partgap() {
    let v = vec![(2, 12, PartGap)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_PARTGAP);

    assert_eq!(v, foldlist.into_vec(1));
  }

}
