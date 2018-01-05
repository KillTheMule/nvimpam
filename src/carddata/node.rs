use card::keyword::Keyword::*;
use card::Card;
use card::cell::Cell::*;
use card::line::Line::*;
use card::line::Conditional::*;
use card::ges::GesType::*;

pub static NODE: Card = Card {
  lines: &[
    Cells(&[Kw(Node), Integer(16), Float(16), Float(16), Float(16)]),
  ],
  ownfold: false,
};

pub static CNODE: Card = Card {
  lines: &[
    Cells(&[Kw(Cnode), Integer(16), Float(16), Float(16), Float(16)]),
  ],
  ownfold: false,
};

pub static MASS: Card = Card {
  lines: &[
    Cells(
      &[
        Kw(Mass),
        Integer(8),
        Integer(8),
        Float(16),
        Float(16),
        Float(16),
      ],
    ),
    Cells(&[Fixed("NAME"), Str(76)]),
    Cells(&[Float(16), Float(16), Float(16)]),
    Provides(
      &[Blank(8), Float(16), Float(16), Float(16), Blank(24), Cont],
      RelChar(80, '&'),
    ),
    Optional(&[Blank(8), Float(16), Float(16), Float(16)], 0),
    Ges(GesNode),
  ],
  ownfold: true,
};

pub static NSMAS: Card = Card {
  lines: &[
    Cells(
      &[
        Kw(Nsmas),
        Integer(8),
        Float(16),
        Float(16),
        Float(16),
        Float(16),
      ],
    ),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
};

pub static NSMAS2: Card = Card {
  lines: &[
    Cells(
      &[
        Kw(Nsmas2),
        Integer(8),
        Float(16),
        Float(16),
        Float(16),
        Float(16),
      ],
    ),
    Cells(&[Fixed("NAME"), Str(76)]),
    Ges(GesEle),
  ],
  ownfold: true,
};
