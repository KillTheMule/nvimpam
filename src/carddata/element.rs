//! This modules holds the the global static element [`Card`](crate::card::Card)
//! instances.
use crate::card::{
  cell::{Cell::*, FixedStr},
  hint::Hint::*,
  keyword::Keyword::*,
  line::Line::*,
  Card,
};

pub static SOLID: Card = Card {
  lines: &[
    Cells(&[Kw(Solid), Integer(8, IDEL), Integer(8, IDPRT)]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(8, IDNOD8),
    ]),
  ],
  ownfold: false,
};

pub static HEXA20: Card = Card {
  lines: &[
    Cells(&[Kw(Hexa20), Integer(8, IDEL), Integer(8, IDPRT)]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(8, IDNOD8),
    ]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD9),
      Integer(8, IDNOD10),
      Integer(8, IDNOD11),
      Integer(8, IDNOD12),
      Integer(8, IDNOD13),
      Integer(8, IDNOD14),
      Integer(8, IDNOD15),
      Integer(8, IDNOD16),
    ]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD17),
      Integer(8, IDNOD18),
      Integer(8, IDNOD19),
      Integer(8, IDNOD20),
    ]),
  ],
  ownfold: false,
};

pub static PENT15: Card = Card {
  lines: &[
    Cells(&[Kw(Pent15), Integer(8, IDEL), Integer(8, IDPRT)]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(8, IDNOD8),
    ]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD9),
      Integer(8, IDNOD10),
      Integer(8, IDNOD11),
      Integer(8, IDNOD12),
      Integer(8, IDNOD13),
      Integer(8, IDNOD14),
      Integer(8, IDNOD15),
    ]),
  ],
  ownfold: false,
};

pub static PENTA6: Card = Card {
  lines: &[Cells(&[
    Kw(Penta6),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
    Integer(8, IDNOD5),
    Integer(8, IDNOD6),
  ])],
  ownfold: false,
};

pub static TETR10: Card = Card {
  lines: &[
    Cells(&[Kw(Tetr10), Integer(8, IDEL), Integer(8, IDPRT)]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(8, IDNOD8),
    ]),
    Cells(&[Blank(16), Integer(8, IDNOD9), Integer(8, IDNOD10)]),
  ],
  ownfold: false,
};

pub static BAR: Card = Card {
  lines: &[Cells(&[
    Kw(Bar),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
  ])],
  ownfold: false,
};

pub static BSHEL: Card = Card {
  lines: &[
    Cells(&[Kw(Bshel), Integer(8, IDEL), Integer(8, IDPRT)]),
    Cells(&[
      Blank(16),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(8, IDNOD8),
    ]),
  ],
  ownfold: false,
};

pub static TSHEL: Card = Card {
  lines: &[Cells(&[
    Kw(Tshel),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
    Blank(8),
    Float(8, H),
  ])],
  ownfold: false,
};

pub static SHELL: Card = Card {
  lines: &[Cells(&[
    Kw(Shell),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
    Blank(8),
    Float(8, H),
  ])],
  ownfold: false,
};

pub static SHEL6: Card = Card {
  lines: &[Cells(&[
    Kw(Shel6),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
    Integer(8, IDNOD5),
    Integer(8, IDNOD6),
  ])],
  ownfold: false,
};

pub static SHEL8: Card = Card {
  lines: &[
    Cells(&[
      Kw(Shel8),
      Integer(8, IDEL),
      Integer(8, IDPRT),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
    ]),
    Cells(&[
      Blank(24),
      Integer(8, IDNOD5),
      Integer(8, IDNOD6),
      Integer(8, IDNOD7),
      Integer(9, IDNOD8),
    ]),
  ],
  ownfold: false,
};

pub static MEMBR: Card = Card {
  lines: &[Cells(&[
    Kw(Membr),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
    Blank(8),
    Float(8, BETA1),
    Float(8, BETA2),
  ])],
  ownfold: false,
};

pub static BEAM: Card = Card {
  lines: &[
    Cells(&[
      Kw(Beam),
      Integer(8, IDEL),
      Integer(8, IDPRT),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Blank(10),
      Binary(6, DOFCD1),
      Blank(2),
      Binary(6, DOFCD2),
      Integer(8, ITPR),
    ]),
    Cells(&[
      Blank(8),
      Float(8, ALPHA1),
      Float(8, BETA1),
      Float(8, GAMMA1),
      Float(8, KSI1),
      Float(8, ETA1),
    ]),
    Cells(&[
      Blank(8),
      Float(8, ALPHA2),
      Float(8, BETA2),
      Float(8, GAMMA2),
      Float(8, KSI2),
      Float(8, ETA2),
    ]),
  ],
  ownfold: false,
};

pub static SPRGBM: Card = Card {
  lines: &[Cells(&[
    Kw(Sprgbm),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
  ])],
  ownfold: false,
};

pub static TETR4: Card = Card {
  lines: &[Cells(&[
    Kw(Tetr4),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    Integer(8, IDNOD4),
  ])],
  ownfold: false,
};

// TODO(KillTheMule): Last 2 cells could be different, see docs
pub static SPRING: Card = Card {
  lines: &[Cells(&[
    Kw(Spring),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    IntegerorBlank(8, IDNOD4),
  ])],
  ownfold: false,
};

// TODO(KillTheMule): IDNOD3/4 could be something different
pub static JOINT: Card = Card {
  lines: &[Cells(&[
    Kw(Joint),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IDNOD3),
    IntegerorBlank(8, IDNOD4),
    Float(8, PHI0),
    Float(8, THET0),
    Float(8, PSI0),
  ])],
  ownfold: false,
};

pub static KJOIN: Card = Card {
  lines: &[
    Cells(&[
      Kw(Kjoin),
      Integer(8, IDEL),
      Integer(8, IDPRT),
      Str(8, JTYP),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IFRA1),
      Integer(8, IFRA2),
      Float(8, REPLEN),
    ]),
    Cells(&[Blank(10), Binary(6, DOFCOD)]),
  ],
  ownfold: false,
};

pub static MTOJNT: Card = Card {
  lines: &[
    Cells(&[
      Kw(Mtojnt),
      Integer(8, IDMTOJ),
      Integer(8, IDPRT),
      Str(8, JTYP),
      Integer(8, IDNODi),
      Integer(8, IDNODd),
      Integer(8, IFRA1),
      Integer(8, IFRA2),
    ]),
    Cells(&[Blank(10), Binary(6, DOFCOD)]),
  ],
  ownfold: false,
};

pub static SPHEL: Card = Card {
  lines: &[Cells(&[
    Kw(Sphel),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD),
    Float(8, VOL),
  ])],
  ownfold: false,
};

pub static SPHELO: Card = Card {
  lines: &[Cells(&[
    Kw(Sphelo),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD),
    Float(8, VOL),
  ])],
  ownfold: false,
};

pub static GAP: Card = Card {
  lines: &[Cells(&[
    Kw(Gap),
    Integer(8, IDEL),
    Integer(8, IDPRT),
    Integer(8, IDNOD1),
    Integer(8, IDNOD2),
    Integer(8, IFRA),
  ])],
  ownfold: false,
};

pub static IMPMA: Card = Card {
  lines: &[
    Cells(&[
      Kw(Impma),
      Integer(8, IDIMPMA),
      Str(8, QUALIFIER1),
      Str(8, QUALIFIER2),
      Str(8, QUALIFIER3),
      Integer(8, IMATYP),
      Integer(8, ISEL),
    ]),
    Cells(&[Fixed(FixedStr::Name), Str(0, TITLE)]),
    Cells(&[Blank(8), Str(8, FNAMEma)]),
  ],
  ownfold: false,
};

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

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

  cardtest!(fold_solid, CARD_SOLID, vec![(1, 14, Solid)]);

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

  cardtest!(fold_hexa20, CARD_HEXA20, vec![(1, 17, Hexa20)]);

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

  cardtest!(fold_pent15, CARD_PENT15, vec![(1, 13, Pent15)]);

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

  cardtest!(fold_penta6, CARD_PENTA6, vec![(1, 8, Penta6)]);

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

  cardtest!(fold_tetr10, CARD_TETR10, vec![(1, 12, Tetr10)]);

  const CARD_TETR4: [&'static str; 6] = [
    "$TETR4 Element",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
    "TETR4 /                                                 ",
  ];

  cardtest!(fold_tetr4, CARD_TETR4, vec![(1, 5, Tetr4)]);

  const CARD_BSHEL: [&'static str; 6] = [
    "BSHEL /                 ",
    "                                                                                ",
    "BSHEL /                 ",
    "                                                                                ",
    "BSHEL /                 ",
    "                                                                                ",
  ];

  cardtest!(fold_bshel, CARD_BSHEL, vec![(0, 5, Bshel)]);

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

  cardtest!(fold_tshel, CARD_TSHEL, vec![(1, 8, Tshel)]);

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

  cardtest!(fold_shell, CARD_SHELL, vec![(1, 8, Shell)]);

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

  cardtest!(fold_shel6, CARD_SHEL6, vec![(1, 8, Shel6)]);

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

  cardtest!(fold_shel8, CARD_SHEL8, vec![(1, 7, Shel8)]);

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

  cardtest!(fold_membr, CARD_MEMBR, vec![(1, 8, Membr)]);

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

  cardtest!(fold_beam, CARD_BEAM, vec![(1, 13, Beam)]);

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

  cardtest!(fold_sprgbm, CARD_SPRGBM, vec![(1, 8, Sprgbm)]);

  const CARD_BAR: [&'static str; 6] = [
    "$BAR  Element",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
    "BAR   /                                                 ",
  ];

  cardtest!(fold_bar, CARD_BAR, vec![(1, 5, Bar)]);

  const CARD_SPRING: [&'static str; 7] = [
    "$SPRING  Element",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
    "$SPRING  Element",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
    "SPRING/                                                 ",
  ];

  cardtest!(fold_spring, CARD_SPRING, vec![(1, 6, Spring)]);

  const CARD_JOINT: [&'static str; 3] = [
    "JOINT /                                                 ",
    "JOINT /                                                 ",
    "JOINT /                                                 ",
  ];

  cardtest!(fold_joint, CARD_JOINT, vec![(0, 2, Joint)]);

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

  cardtest!(fold_kjoin, CARD_KJOIN, vec![(1, 7, Kjoin)]);

  const CARD_MTOJNT: [&'static str; 7] = [
    "$MTOJNTElement",
    "MTOJNT/                                                                 ",
    "                ",
    "MTOJNT/                                                                 ",
    "                ",
    "MTOJNT/                                                                 ",
    "                ",
  ];

  cardtest!(fold_mtojnt, CARD_MTOJNT, vec![(1, 6, Mtojnt)]);

  const CARD_SPHEL: [&'static str; 6] = [
    "SPHEL /                                                                 ",
    "SPHEL /                                                                 ",
    "$SPHEL Element",
    "SPHEL /                                                                 ",
    "$SPHEL Element",
    "SPHEL /                                                                 ",
  ];

  cardtest!(fold_sphel, CARD_SPHEL, vec![(0, 5, Sphel)]);

  const CARD_SPHELO: [&'static str; 2] = [
    "SPHELO/                                                                 ",
    "SPHELO/                                                                 ",
  ];

  cardtest!(fold_sphelo, CARD_SPHELO, vec![(0, 1, Sphelo)]);

  const CARD_GAP: [&'static str; 6] = [
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
    "#COMMENT",
    "GAP   /                                                                 ",
    "GAP   /                                                                 ",
  ];

  cardtest!(fold_gap, CARD_GAP, vec![(0, 5, Gap)]);

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

  cardtest!(fold_impma, CARD_IMPMA, vec![(2, 13, Impma)]);

}
