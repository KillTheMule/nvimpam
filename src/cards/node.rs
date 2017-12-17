use cards::keywords::Keyword::*;
use cards::lines::Card;
use cards::cells::Cell::*;
use cards::lines::Line::*;
use cards::lines::Conditional::*;
use cards::lines::GesType::*;

pub static NODE: Card = Card {
  lines: &[
    Cells(&[Kw(Node), Integer(16), Float(16), Float(16), Float(16)]),
  ],
};

pub static CNODE: Card = Card {
  lines: &[
    Cells(&[Kw(Cnode), Integer(16), Float(16), Float(16), Float(16)]),
  ],
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
    Cells(&[Blank(8), Float(16), Float(16), Float(16), Cont]),
    Optional(
      &[Blank(8), Float(16), Float(16), Float(16)],
      Some(RelChar(-1, 81, '&')),
    ),
    Ges(GesNode),
  ],
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
    Ges(GesEle)
  ],
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
    Ges(GesEle)
  ],
};
