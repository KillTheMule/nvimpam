//! This modules holds the the global static node [`Card`](crate::card::Card)
//! instances.
use crate::card::{
  cell::{Cell::*, CellHint::*, FixedStr},
  ges::GesType::*,
  keyword::Keyword::*,
  line::{Conditional::*, Line::*, LineHint},
  Card, CardHint,
};

pub static NODE: Card = Card {
  lines: &[Cells(&[
    Kw(Node),
    Integer(8),
    Float(16),
    Float(16),
    Float(16),
  ])],
  ownfold: false,
};

pub static DUMMYHINT: CardHint = CardHint { linehints: &[] };

pub static NODEHINT: CardHint = CardHint {
  linehints: &[LineHint {
    cellhints: &[Keyword(8), IDNOD(8), X(16), Y(16), Z(16)],
  }],
};

pub static CNODE: Card = Card {
  lines: &[Cells(&[
    Kw(Cnode),
    Integer(8),
    Float(16),
    Float(16),
    Float(16),
  ])],
  ownfold: false,
};

pub static MASS: Card = Card {
  lines: &[
    Cells(&[
      Kw(Mass),
      Integer(8),
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed(FixedStr::Name), Str(76)]),
    Cells(&[Float(16), Float(16), Float(16)]),
    Provides(
      &[Blank(8), Float(16), Float(16), Float(16), Blank(24), Cont],
      RelChar(80, b'&'),
    ),
    Optional(&[Blank(8), Float(16), Float(16), Float(16)], 0),
    Ges(GesNode),
  ],
  ownfold: true,
};

pub static NSMAS: Card = Card {
  lines: &[
    Cells(&[
      Kw(Nsmas),
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed(FixedStr::Name), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
};

pub static NSMAS2: Card = Card {
  lines: &[
    Cells(&[
      Kw(Nsmas2),
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed(FixedStr::Name), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
};

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

  const CARD_NSMAS: [&'static str; 7] = [
    "NSMAS /        1              0.                                                ",
    "$#                                                                         TITLE",
    "NAME NSMAS / ->1                                                                ",
    "        ELE 123",
    "        PART 2345",
    "        END",
    "#Comment",
  ];

  cardtest!(fold_nsmas, CARD_NSMAS, vec![(0, 5, Nsmas)]);

  const CARD_NSMAS2: [&'static str; 7] = [
    "$ NSMAS - Nonstructural mass",
    "$#       IDNODMS            MASS            MLEN            MARE            MVOL",
    "NSMAS /        1              0.                                                ",
    "$#                                                                         TITLE",
    "NAME NSMAS / ->1                                                                ",
    "        ELE ",
    "        END",
  ];

  cardtest!(fold_nsmas2, CARD_NSMAS2, vec![(2, 6, Nsmas)]);

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

  cardtest!(fold_mass, CARD_MASS, vec![(2, 9, Mass)]);

  const CARD_MASS_OPT: [&'static str; 12] = [
    "MASS  /        0       0                                                        ",
    "$#                                                                         TITLE",
    "NAME MASS  / ->1                                                                ",
    "$# BLANK              Mx              My              Mz",
    "                                                        ",
    "$# BLANK              Ix              Iy              Iz                   Blank",
    "                                                                                &",
    "                                                  ",
    "        PART 1234",
    "        GRP 'nogrp'",
    "        END",
    "$Comment",
  ];

  cardtest!(fold_mass_opt, CARD_MASS_OPT, vec![(0, 10, Mass)]);

  const CARD_NODES: [&'static str; 9] = [
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "#COMMENT",
    "NODE  /       28     30.29999924            50.5              0.",
    "$COMMENT",
    "NODE  /       28     30.29999924            50.5              0.",
    "NODE  /       28     30.29999924            50.5              0.",
    "SHELL /     ",
  ];

  cardtest!(fold_nodes, CARD_NODES, vec![(0, 7, Node), (8, 8, Shell)]);

  #[test]
  fn cellhint() {
    use crate::{bufdata::BufData, linenr::LineNr};
    use neovim_lib::{neovim_api::Buffer, Value};

    let buf = Buffer::new(Value::from(0_usize));
    let mut bufdata = BufData::new(&buf);
    bufdata.parse_strs(&CARD_NODES).unwrap();

    assert_eq!(
      bufdata.cellhint(LineNr::from_usize(1), 0),
      Value::from("Keyword")
    );
    assert_eq!(
      bufdata.cellhint(LineNr::from_usize(1), 10),
      Value::from("IDNOD")
    );
    assert_eq!(
      bufdata.cellhint(LineNr::from_usize(1), 8),
      Value::from("IDNOD")
    );
  }
}
