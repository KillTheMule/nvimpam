//! This modules holds the the global static auxiliary [`Card`](::card::Card)
//! instances.
use crate::card::{
  cell::Cell::*, ges::GesType::*, keyword::Keyword::*, line::Line::*, Card,
};

pub static GROUP: Card = Card {
  lines: &[
    Cells(&[Kw(Group), Str(72)]),
    OptionalBlock(b"META", b"END_META"),
    Ges(GesNode),
  ],
  ownfold: true,
};

pub static COMMENT: Card = Card {
  lines: &[Cells(&[Fixed("#")])],
  ownfold: false,
};

#[cfg(test)]
mod tests {
  use crate::card::keyword::Keyword::*;

  const CARD_GROUP: [&'static str; 7] = [
    "GROUP / TitleOfTheGroup",
    "        ELE ",
    "        DELNOD ",
    "        PART 14 ",
    "        OGRP ",
    "        ELE ",
    "        END",
  ];

  cardtest!(fold_group, CARD_GROUP, vec![(0, 6, Group)]);

  const CARD_GROUP2: [&'static str; 11] = [
    "GROUP / TitleOfTheGroup",
    "        ELE ",
    "        DELNOD ",
    "        PART 14 ",
    "        OGRP ",
    "        ELE ",
    "        END",
    "GROUP / TitleOfTheGroup2",
    "        ELE ",
    "        ELE ",
    "        ELE ",
  ];

  cardtest!(
    fold_group2,
    CARD_GROUP2,
    vec![(0, 6, Group), (7, 10, Group)],
    vec![(0, 10, Group)]
  );

}
