//! This modules holds the the global static auxiliary [`Card`](::card::Card)
//! instances.
use crate::card::cell::Cell::*;
use crate::card::ges::GesType::*;
use crate::card::keyword::Keyword::*;
use crate::card::line::Line::*;
use crate::card::Card;

pub static GROUP: Card = Card {
  lines: &[
    Cells(&[Kw, Str(72)]),
    OptionalBlock("META", "END_META"),
    Ges(GesNode),
  ],
  ownfold: true,
  keyword: Group,
};

#[cfg(test)]
mod tests {

  const CARD_GROUP: [&'static str; 7] = [
    "GROUP / TitleOfTheGroup",
    "        ELE ",
    "        DELNOD ",
    "        PART 14 ",
    "        OGRP ",
    "        ELE ",
    "        END",
  ];

  #[test]
  fn fold_group() {
    use crate::card::keyword::Keyword::*;
    use crate::folds::FoldList;

    let v = vec![(0, 6, Group)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_GROUP);

    assert_eq!(v, foldlist.into_vec(1));
  }

}
