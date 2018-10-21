//! This modules holds the the global static link [`Card`](::card::Card)
//! instances.
use card::{
  cell::Cell::*, ges::GesType::*, keyword::Keyword::*, line::Line::*, Card,
};

pub static ELINK: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8), Integer(8)]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Elink,
};

pub static LLINK: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8), Integer(8)]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Llink,
};

pub static SLINK: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Slink,
};

pub static PLINK: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Plink,
};

pub static TIED: Card = Card {
  lines: &[
    Cells(&[Kw, Integer(8), Integer(8), Integer(8)]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesNode),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Tied,
};

#[cfg(test)]
mod tests {
  use card::keyword::Keyword::*;

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
