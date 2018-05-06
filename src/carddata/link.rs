//! This modules holds the the global static link [`Card`](::card::Card)
//! instances.
use card::cell::Cell::*;
use card::keyword::Keyword::*;
use card::line::Line::*;
use card::Card;
use card::ges::GesType::*;

pub static ELINK: Card = Card {
  lines: &[
    Cells(&[
      Kw,
      Integer(8),
      Integer(8),
      Integer(8),
      Integer(8),
    ]),
    Ges(GesFace),
  ],
  ownfold: false,
  keyword: Elink,
};

#[cfg(test)]
mod tests {

  const CARD_ELINK: [&'static str; 13]= [
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

  #[test]
  fn fold_elink() {
    use card::keyword::Keyword;
    use card::keyword::Keyword::*;
    use folds::FoldList;

    let v: Vec<(u64, u64, Keyword)> = vec![(1, 12, Elink)];
    let mut foldlist = FoldList::new();
    let _ = foldlist.add_folds(&CARD_ELINK);

    assert_eq!(v, foldlist.into_vec());
  }

}
