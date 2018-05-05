//! This modules holds the the global static node [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::ges::GesType::*;
use card::keyword::Keyword::*;
use card::line::Conditional::*;
use card::line::Line::*;
use card::Card;

pub static NODE: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(16),
    Float(16),
    Float(16),
    Float(16),
  ])],
  ownfold: false,
  keyword: Node,
};

pub static CNODE: Card = Card {
  lines: &[Cells(&[
    Kw,
    Integer(16),
    Float(16),
    Float(16),
    Float(16),
  ])],
  ownfold: false,
  keyword: Cnode,
};

pub static MASS: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[Float(16), Float(16), Float(16)]),
    Provides(
      &[
        Blank(8),
        Float(16),
        Float(16),
        Float(16),
        Blank(24),
        Cont,
      ],
      RelChar(80, '&'),
    ),
    Optional(&[Blank(8), Float(16), Float(16), Float(16)], 0),
    Ges(GesNode),
  ],
  ownfold: true,
  keyword: Mass,
};

pub static NSMAS: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
  keyword: Nsmas,
};

pub static NSMAS2: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Float(16),
      Float(16),
      Float(16),
      Float(16),
    ]),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
  keyword: Nsmas2,
};
