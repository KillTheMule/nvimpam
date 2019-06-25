//! This modules holds the the global static part [`Card`](crate::card::Card)
//! instances.
use crate::card::{
  cell::{Cell::*, FixedStr},
  hint::Hint::*,
  keyword::Keyword::*,
  line::{Conditional::*, Line::*},
  Card,
};

macro_rules! part {
  ($($e: expr),+; $k: expr) => {
    Card {
      lines: &[
        Provides(&[
          Kw($k),
          Integer(8, IDPRT),
          Str(8, ATYPE),
          Integer(8, IDMAT),
          Integer(8, IDVAMAT),
          Integer(8, IDTHMAT),
          Integer(8, IDPMAT),
          Integer(8, IDNUMPAR),
          ], Int(25..33, 0)),
        Optional(&[Fixed(FixedStr::Rmat), Str(0, REFNAM)], 0),
        Cells(&[Fixed(FixedStr::Name), Str(0, TITLE)]),
        OptionalBlock(b"META", b"END_META", META),
        Cells(&[Float(10, DTELIM), Float(10, TSCALF), Float(10, DTRATIO)]),
        Cells(&[Float(10, TCONT), Float(10, EPSINI), Float(10, COULFRIC)]),
        $( $e ),+ ,
        Cells(&[Fixed(FixedStr::EndPart)]),
      ],
      ownfold: true,
    };
  };
}

macro_rules! part1d {
  ($($e: expr),+; $k: expr) => {
    Card {
      lines: &[
        Provides(&[
          Kw($k),
          Integer(8, IDPRT),
          Str(8, ATYPE),
          Integer(8, IDMAT),
          Integer(8, IDVAMAT),
          Integer(8, IDTHMAT),
          Integer(8, IDPMAT)
          ], Int(25..33, 0)),
        Optional(&[Fixed(FixedStr::Rmat), Str(0, REFNAM)], 0),
        Cells(&[Fixed(FixedStr::Name), Str(0, TITLE)]),
        OptionalBlock(b"META", b"END_META", META),
        Cells(&[Float(10, DTELIM), Float(10, TSCALF), Float(10, DTRATIO)]),
        Cells(&[Float(10, TCONT), Float(10, EPSINI), Float(10, COULFRIC)]),
        $( $e ),+ ,
        Cells(&[Fixed(FixedStr::EndPart)]),
      ],
      ownfold: true,
    };
  };
}

// Part 3D

pub static PARTSOLID: Card = part!(
  Cells(&[Integer(5, IORT1), Blank(5), Float(10, XDIR1), Float(10, YDIR1), Float(10, ZDIR1)]),
  Cells(&[Integer(5, IORT2), Blank(5), Float(10, XDIR2), Float(10, YDIR2), Float(10, ZDIR2)])
  ;PartSolid);

pub static PARTCOS3D: Card = part!(
  Cells(&[Blank(10), Float(10, THK), Float(10, XDIR1), Float(10, YDIR1), Float(10, ZDIR1), Integer(10, IMETH)])
  ;PartCos3d);

pub static PARTBSHEL: Card = part!(
  Cells(&[Blank(0)])
  ;PartBshel);

pub static PARTTETRA: Card = part!(
  Cells(&[Integer(5, IORT1), Blank(5), Float(10, XDIR1), Float(10, YDIR1), Float(10, ZDIR1)]),
  Cells(&[Integer(5, IORT2), Blank(5), Float(10, XDIR2), Float(10, YDIR2), Float(10, ZDIR2)])
  ; PartTetra);

pub static PARTSPHEL: Card = part!(
  Provides(&[Float(10, RATIO), Float(10, Hmin), Float(10, Hmax), Float(10, ETA), Integer(5, INORM), Integer(5, NPAIR),
             Float(10, ALPHAmg), Float(10, BETAmg), Integer(5, NMON)],
           Number(46..51)),
  Repeat(&[Integer(10, IDPRT), Float(10, Hfac)], 1)
  ; PartSphel);

// PART 2D

pub static PARTTSHEL: Card = part!(
  Cells(&[Float(10, H), Integer(5, NINT)])
  ;PartTshel);

pub static PARTSHELL: Card = part!(
  Cells(&[Float(10, H), Integer(5, NINT), Float(10, OFFSET), Integer(5, NTHDOF)]),
  Cells(&[Integer(5, IORT), Blank(5), Float(10, XDIR), Float(10, YDIR), Float(10, ZDIR), Float(10, Aoff)])
  ;PartShell);

pub static PARTMEMBR: Card = part!(
  Cells(&[Float(10, H), Blank(15), Integer(5, NTHDOF)]),
  Cells(&[Integer(5, IORT1), Blank(5), Float(10, VX1), Float(10, VY1), Float(10,  VZ1), Float(10, ALPHof1), Float(10, TX1),
          Float(10, TY1), Float(10, TZ1)]),
  Cells(&[Integer(5, IORT2), Blank(5), Float(10, VX2), Float(10, VY2), Float(10,  VZ2), Float(10, ALPHof2), Float(10, TX2),
          Float(10, TY2), Float(10, TZ2)])
  ;PartMembr);

// PART 1D

pub static PARTBAR: Card = part1d!(
  Cells(&[Float(10, A)])
  ;PartBar);

// TODO(KillTheMule): Need to check IDMAT for the first line's hints
pub static PARTBEAM: Card = part1d!(
  Cells(&[Float(10, A), Float(10, Ashs), Float(10, Is), Float(10, It), Float(10, Ir), Blank(5),
          Integer(5, ITPR), Float(10, Asht)]),
  Cells(&[Float(10, Ist), Float(10, COGs), Float(10, COGt), Blank(4), Binary(6, DOFCD1), Blank(4),
          Binary(6, DOFCD2)]),
  Cells(&[Float(10, ALPHA1), Float(10, BETA1), Float(10, GAMMA1), Float(10, KSI1), Float(10, ETA1)]),
  Cells(&[Float(10, ALPHA2), Float(10, BETA2), Float(10, GAMMA2), Float(10, KSI2), Float(10, ETA2)]),
  Provides(&[Integer(5, IDSEC), Integer(5, NIPS), Float(10, a), Float(10, b), Float(10, c)],
             Number(6..11)),
  Repeat(&[Float(10, LCOORDsi), Float(10, LCOORDti), Float(10, WTFACi)], 1)
  ;PartBeam);

pub static PARTSPRING: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartSpring);

pub static PARTSPRGBM: Card = part1d!(
  Cells(&[Integer(10, ARM), Float(10, Sr), Float(10, Ss), Float(10, St)])
  ;PartSprgbm);

pub static PARTMBSPR: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartMbspr);

pub static PARTJOINT: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartJoint);

pub static PARTKJOIN: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartKjoin);

pub static PARTMTOJNT: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartMtojnt);

pub static PARTMBKJN: Card = part1d!(
  Cells(&[Blank(0)])
  ;PartMbkjn);

// TODO(KillTheMule): Line 2 hints depend on IDMAT
pub static PARTTIED: Card = part1d!(
  Provides(&[Float(10, RDIST), Blank(50), Integer(10, ITSSR), Integer(10, INEXT)], Int(71..81, 1)),
  Optional(&[Integer(5, IORT), Blank(5), Float(10, XDIR), Float(10, YDIR), Float(10, ZDIR), Float(10, ALPHof)],
           1)
  ;PartTied);

// TODO(KillTheMule): Line 2 hints depend on IDMAT
pub static PARTSLINK: Card = part1d!(
  Provides(&[Float(10, RDIST), Blank(60), Integer(10, INEXT)], Int(71..81, 1)),
  Optional(&[Integer(5, IORT), Blank(5), Float(10, XDIR), Float(10, YDIR), Float(10, ZDIR), Float(10, ALPHof)],
           1)
  ;PartSlink);

// TODO(KillTheMule): Line 2 hints depend on IDMAT
pub static PARTELINK: Card = part1d!(
  Provides(&[Float(10, RDIST), Float(10, Aedge), Blank(50), Integer(10, INEXT)], Int(71..81, 1)),
  Optional(&[Integer(5, IORT), Blank(5), Float(10, XDIR), Float(10, YDIR), Float(10, ZDIR), Float(10, ALPHof)],
           1)
  ;PartElink);

pub static PARTPLINK: Card = part1d!(
  Cells(&[Float(10, RSEAR), Integer(10, NLAYR), Float(10, SPWLG), Integer(10, NGESP), Float(10, SPOTRA), Float(10, THETA),
          Integer(10, IRADBEN), Float(10, PHYRAD)])
  ;PartPlink);

pub static PARTGAP: Card = part1d!(
  Cells(&[Blank(10), Float(10, GAP)])
  ;PartGap);

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

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

  cardtest!(
    fold_partsolid,
    CARD_PARTSOLID,
    vec![(2, 14, PartSolid), (15, 21, PartSolid)]
  );

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

  cardtest!(fold_partsolid2, CARD_PARTSOLID2, vec![(2, 14, PartSolid)]);

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

  cardtest!(fold_partcos3d, CARD_PARTCOS3D, vec![(2, 12, PartCos3d)]);

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

  cardtest!(fold_partbshel, CARD_PARTBSHEL, vec![(2, 11, PartBshel)]);

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

  cardtest!(fold_parttetra, CARD_PARTTETRA, vec![(2, 14, PartTetra)]);

  const CARD_PARTSPHEL: [&'static str; 13] = [
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
    "                                                 0                         ",
    "$#      ",
    "END_PART",
  ];

  cardtest!(fold_partsphel, CARD_PARTSPHEL, vec![(2, 12, PartSphel)]);

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

  cardtest!(fold_sphel2, CARD_PARTSPHEL2, vec![(2, 15, PartSphel)]);

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

  cardtest!(fold_parttshel, CARD_PARTTSHEL, vec![(1, 11, PartTshel)]);

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

  cardtest!(fold_partshell, CARD_PARTSHELL, vec![(2, 14, PartShell)]);

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

  cardtest!(fold_partshell2, CARD_PARTSHELL2, vec![(0, 10, PartShell)]);

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

  cardtest!(fold_partmembr, CARD_PARTMEMBR, vec![(2, 16, PartMembr)]);

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

  cardtest!(fold_partbar, CARD_PARTBAR, vec![(2, 12, PartBar)]);

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

  cardtest!(fold_partbeam, CARD_PARTBEAM, vec![(1, 19, PartBeam)]);

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

  cardtest!(fold_partbeam2, CARD_PARTBEAM2, vec![(1, 22, PartBeam)]);

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

  cardtest!(fold_partspring, CARD_PARTSPRING, vec![(2, 11, PartSpring)]);

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

  cardtest!(fold_partsprgbm, CARD_PARTSPRGBM, vec![(1, 9, PartSprgbm)]);

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

  cardtest!(fold_partmbspr, CARD_PARTMBSPR, vec![(2, 8, PartMbspr)]);

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

  cardtest!(fold_partjoint, CARD_PARTJOINT, vec![(2, 11, PartJoint)]);

  const CARD_PARTKJOIN: [&'static str; 6] = [
    "PART  /        1   KJOIN       1       0       0       0",
    "NAME PART_1                                                                     ",
    "                              ",
    "                              ",
    "",
    "END_PART",
  ];

  cardtest!(fold_partkjoin, CARD_PARTKJOIN, vec![(0, 5, PartKjoin)]);

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

  cardtest!(fold_partmtojnt, CARD_PARTMTOJNT, vec![(2, 12, PartMtojnt)]);

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

  cardtest!(fold_partmbkjn, CARD_PARTMBKJN, vec![(2, 11, PartMbkjn)]);

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

  cardtest!(fold_parttied, CARD_PARTTIED, vec![(0, 10, PartTied)]);

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

  cardtest!(fold_parttied2, CARD_PARTTIED2, vec![(0, 11, PartTied)]);

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

  cardtest!(fold_partslink, CARD_PARTSLINK, vec![(2, 12, PartSlink)]);

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

  cardtest!(fold_partelink, CARD_PARTELINK, vec![(2, 10, PartElink)]);

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

  cardtest!(fold_partplink, CARD_PARTPLINK, vec![(2, 8, PartPlink)]);

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

  cardtest!(fold_partgap, CARD_PARTGAP, vec![(2, 12, PartGap)]);

}
