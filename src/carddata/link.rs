//! This modules holds the the global static link [`Card`](crate::card::Card)
//! instances.
use crate::card::{
  cell::{Cell::*, FixedStr},
  ges::GesType::*,
  keyword::Keyword::*,
  line::Line::*,
  hint::Hint::*,
  Card,
};

pub static ELINK: Card = Card {
  lines: &[
    Cells(&[Kw(Elink), Integer(8, IDELE), Integer(8, IDPRT), Integer(8, IDNOD1), Integer(8, IDNOD2)]),
    Ges(GesFace),
  ],
  ownfold: false,
};

// LLINK isn't in 2018 anymore
/*
pub static LLINK: Card = Card {
  lines: &[
    Cells(&[Kw(Llink), Integer(8), Integer(8), Integer(8), Integer(8)]),
    Ges(GesFace),
  ],
  ownfold: false,
};
*/

pub static SLINK: Card = Card {
  lines: &[
    Cells(&[
      Kw(Slink),
      Integer(8, IDELE),
      Integer(8, IDPRT),
      Integer(8, IDNOD1),
      Integer(8, IDNOD2),
      Integer(8, IDNOD3),
      Integer(8, IDNOD4),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
};

pub static PLINK: Card = Card {
  lines: &[
    Cells(&[
      Kw(Plink),
      Integer(8, IDELE),
      Integer(8, IDPRT),
      Integer(8, IDNOD1),
      Integer(8, MORE),
      Integer(8, NLAYR),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
};

pub static TIED: Card = Card {
  lines: &[
    Cells(&[Kw(Tied), Integer(8, IDEL), Integer(8, IDPRT), Integer(8, IPCHK)]),
    Cells(&[Fixed(FixedStr::Name), Str(0, TITLE)]),
    Ges(GesNode),
    Ges(GesFace),
  ],
  ownfold: false,
};

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

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

  cardtest!(fold_elink, CARD_ELINK, vec![(1, 12, Elink)]);

  /*
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

  cardtest!(fold_llink, CARD_LLINK, vec![(1, 22, Llink)]);
  */

  const CARD_SLINK: [&'static str; 6] = [
    "$SLINK Element",
    "SLINK /                                 ",
    "        PART 1",
    "        PART 23",
    "        PART 45",
    "        END",
  ];

  cardtest!(fold_slink, CARD_SLINK, vec![(1, 5, Slink)]);

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

  cardtest!(fold_plink, CARD_PLINK, vec![(1, 7, Plink)]);

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

  cardtest!(fold_tied, CARD_TIED, vec![(1, 21, Tied)]);

}
