#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
//! This module provides the [`Keyword`](::card::keyword::Keyword) enum to
//! classify lines according to what card type they belong to. The terms
//! "Keyword" and "Card" are lingo from the FEM solver Pamcrash, but generally
//! used among FEM solvers.
//!
//! Also provides the [`Keywords`](::card::keyword::Keywords) struct to hold the
//! keywords of a [`Lines`](::lines::Lines) struct. Supposed to be kept in sync
//! via [`Keywords::update`](::card::keyword::Keywords::update).
use crate::lines::Lines;
use std::ops::Deref;

/// An enum to denote the several types of cards a line might belong to.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Keyword {
  Comment,
  // Node
  Node,
  Cnode,
  Mass,
  Nsmas,
  Nsmas2,
  // Element
  Solid,
  Hexa20,
  Pent15,
  Penta6,
  Tetr10,
  Tetr4,
  Bshel,
  Tshel,
  Shell,
  Shel6,
  Shel8,
  Membr,
  Beam,
  Sprgbm,
  Bar,
  Spring,
  Joint,
  Kjoin,
  Mtojnt,
  Sphel,
  Sphelo,
  Gap,
  Impma,
  // Link
  Elink,
  Llink,
  Slink,
  Plink,
  Tied,
  // Part 3D
  PartSolid,
  PartBshel,
  PartTetra,
  PartSphel,
  PartCos3d,
  // Part 2D
  PartTshel,
  PartShell,
  PartMembr,
  // Part 1D
  PartBar,
  PartBeam,
  PartSpring,
  PartSprgbm,
  PartMbspr,
  PartJoint,
  PartKjoin,
  PartMbkjn,
  PartMtojnt,
  PartTied,
  PartSlink,
  PartElink,
  PartLlink,
  PartPlink,
  PartGap,
  // Constraint
  Mtoco,
  Otmco,
  Rbody0,
  Rbody1,
  Rbody2,
  Rbody3,
  // Auxiliaries
  Group,
}

impl Keyword {
  /// Return the length of the keyword in the pamcrash input file
  /// Should be 8 for all right now...
  #[inline]
  pub fn len(self) -> u8 {
    8
  }

  #[inline]
  pub fn is_empty(self) -> bool {
    false
  }

  /// Parse a string to determine if it starts with the keyword of a card.
  #[inline]
  pub fn parse(s: &[u8]) -> Option<Keyword> {
    use self::Keyword::*;
    use byteorder::{BigEndian, ReadBytesExt};

    let len = s.len();

    if len == 0 {
      None
    } else if s[0] == b'#' || s[0] == b'$' {
      Some(Comment)
    } else if len < 8 {
      None
    } else {
      let mut start = &s[0..8];
      let num = match start.read_u64::<BigEndian>() {
        Ok(n) => n,
        Err(_) => return None,
      };

      match num {
        // Node
        // NODE
        5642803921800933152 => Some(Node),
        // b"CNODE / "
        4849901003360710432 => Some(Cnode),
        // b"MASS  / "
        5566822230893014816 => Some(Mass),
        // b"NSMAS / "
        5643939700988194592 => Some(Nsmas),
        // b"NSMAS2/ "
        5643939700989374240 => Some(Nsmas2),
        // b"SOLID / "
        6003100705867444000 => Some(Solid),
        // b"HEXA20/ "
        5207665581161983776 => Some(Hexa20),
        // b"PENT15/ "
        5784115419937058592 => Some(Pent15),
        // b"PENTA6/ "
        5784115420205559584 => Some(Penta6),
        // b"TETR10/ "
        6072352384568274720 => Some(Tetr10),
        // b"TETR4 / "
        6072352384617557792 => Some(Tetr4),
        // b"BSHEL / "
        4779243092037349152 => Some(Bshel),
        // b"TSHEL / "
        6076279784720052000 => Some(Tshel),
        // b"SHELL / "
        6001122697468194592 => Some(Shell),
        // b"SHEL6 / "
        6001122697099095840 => Some(Shel6),
        // b"SHEL8 / "
        6001122697132650272 => Some(Shel8),
        // b"MEMBR / "
        5567941461554507552 => Some(Membr),
        // b"BEAM  / "
        4775294779403546400 => Some(Beam),
        // b"SPRGBM/ "
        6003388769293381408 => Some(Sprgbm),
        // b"BAR   / "
        4774187377920847648 => Some(Bar),
        // b"SPRING/ "
        6003388778084249376 => Some(Spring),
        // b"JOINT / "
        5354579082734481184 => Some(Joint),
        // b"KJOIN / "
        5425235877383122720 => Some(Kjoin),
        // b"MTOJNT/ "
        5572165819524460320 => Some(Mtojnt),
        // b"SPHEL / "
        6003377765751992096 => Some(Sphel),
        // b"SPHELO/ "
        6003377765755072288 => Some(Sphelo),
        // b"GAP   / "
        5134473149087231776 => Some(Gap),
        // b"IMPMA / "
        5281966230710791968 => Some(Impma),
        // b"ELINK / "
        4993446687463714592 => Some(Elink),
        // b"LLINK / "
        5497849845729210144 => Some(Llink),
        // b"SLINK / "
        6002253003994705696 => Some(Slink),
        // b"PLINK / "
        5786080221880921888 => Some(Plink),
        // b"TIED  / "
        6073461731384897312 => Some(Tied),
        // b"PART  / "
        5782993917790138144 => {
          if len < 24 {
            None
          } else {
            let mut p = &s[16..24];

            let num2 = match p.read_u64::<BigEndian>() {
              Ok(n) => n,
              Err(_) => return None,
            };
            match num2 {
              // "   SOLID", "  SOLID ", " SOLID  ", "SOLID   "
              2314885750653208900 | 2314941808515826720
              | 2329292621345988640 | 6003100705867440160 => Some(PartSolid),
              // "BSHEL   ", " BSHEL  ", "  BSHEL ", "   BSHEL"
              4779243092037345312 | 2324511927541964832
              | 2314923133930654752 | 2314885677705610572 => Some(PartBshel),
              // "TETRA   ", " TETRA  ", "  TETRA ", "   TETRA"
              6072352384835657760 | 2329563135716958240
              | 2314942865212588320 | 2314885754780930625 => Some(PartTetra),
              // "SPHEL   ", " SPHEL  ", "  SPHEL ", "   SPHEL"
              6003377765751988256 | 2329293703611162656
              | 2314941812743425056 | 2314885750669722956 => Some(PartSphel),
              // "COS3D   ", " COS3D  ", "  COS3D ", "   COS3D"
              4850186803352707104 | 2324789051414290464
              | 2314924216445781024 | 2314885681934185284 => Some(PartCos3d),
              // "TSHEL   ", " TSHEL  ", "  TSHEL ", "   TSHEL"
              6076279784720048160 | 2329578477122756640
              | 2314942925139954720 | 2314885755015021900 => Some(PartTshel),
              // "SHELL   ", " SHELL  ", "  SHELL ", "   SHELL"
              6001122697468190752 | 2329284894750679072
              | 2314941778333813792 | 2314885750535310412 => Some(PartShell),
              // "MEMBR   ", " MEMBR  ", "  MEMBR ", "   MEMBR"
              5567941461554503712 | 2327592780547891232
              | 2314935168512709152 | 2314885724715696722 => Some(PartMembr),
              // "BAR     ", " BAR    ", "  BAR   ", "   BAR  ", "    BAR ",
              // "     BAR"
              4774187377920843808 | 2324492178658697248
              | 2314923056786579488 | 2314885677404266528
              | 2314885531391054368 | 2314885530820690258 => Some(PartBar),
              // "BEAM    ", " BEAM   ", "  BEAM  ", "   BEAM ", "    BEAM"
              4775294779403542560 | 2324496504445739040
              | 2314923073684185120 | 2314885677470272800
              | 2314885531391312205 => Some(PartBeam),
              // "SPRING  ", " SPRING ", "  SPRING"
              6003388778084245536 | 2329293746628085536
              | 2314941812911459911 => Some(PartSpring),
              // "SPRGBM  ", " SPRGBM ", "  SPRGBM"
              6003388769293377568 | 2329293746593746208
              | 2314941812911325773 => Some(PartSprgbm),
              // "MBSPR   ", " MBSPR  ", "  MBSPR ", "   MBSPR"
              5567103693823680544 | 2327589508017692704
              | 2314935155729388064 | 2314885724665761874 => Some(PartMbspr),
              // "JOINT   ", " JOINT  ", "  JOINT ", "   JOINT"
              5354579082734477344 | 2326759333755625504
              | 2314931912861176864 | 2314885711998307924 => Some(PartJoint),
              // "KJOIN   ", " KJOIN  ", "  KJOIN ", "   KJOIN"
              5425235877383118880 | 2327035336859721760
              | 2314932990998302240 | 2314885716209781070 => Some(PartKjoin),
              // "MTOJNT  ", " MTOJNT ", "  MTOJNT"
              5572165819524456480 | 2327609281946211360
              | 2314935232971296340 => Some(PartMtojnt),
              // "MBKJN   ", " MBKJN  ", "  MBKJN ", "   MBKJN"
              5567094871893745696 | 2327589473557028896
              | 2314935155594776096 | 2314885724665236046 => Some(PartMbkjn),
              // "TIED    ", " TIED   ", "  TIED  ", "   TIED ", "    TIED"
              6073461731384893472 | 2329567469101916192
              | 2314942882139873312 | 2314885754847052832
              | 2314885531693565252 => Some(PartTied),
              // "SLINK   ", " SLINK  ", "  SLINK ", "   SLINK"
              6002253003994701856 | 2329289310010548256
              | 2314941795580922656 | 2314885750602681931 => Some(PartSlink),
              // "ELINK   ", " ELINK  ", "  ELINK ", "   ELINK"
              4993446687463710752 | 2325348660336599072
              | 2314926402418133792 | 2314885690473139787 => Some(PartElink),
              // "LLINK   ", " LLINK  ", "  LLINK ", "   LLINK"
              5497849845729206304 | 2327318985173573664
              | 2314934098999528224 | 2314885720537910859 => Some(PartLlink),
              // "PLINK   ", " PLINK  ", "  PLINK ", "   PLINK"
              5786080221880918048 | 2328444885080416288
              | 2314938497046039328 | 2314885737717780043 => Some(PartPlink),
              // "GAP     ", " GAP    ", "  GAP   ", "   GAP  ", "    GAP ",
              // "     GAP"
              5134473149087227936 | 2325899544952315936
              | 2314928554311163936 | 2314885698878971936
              | 2314885531474939936 | 2314885530821017936 => Some(PartGap),
              _ => None,
            }
          }
        }
        // Constraint
        // b"MTOCO / "
        5572165789473058592 => Some(Mtoco),
        // b"OTMCO / "
        5716278778525658912 => Some(Otmco),
        // b"RBODY / "
        5927387214544645920 => {
          if len < 32 {
            None
          } else {
            let mut p = &s[24..32];
            let num3 = match p.read_u64::<BigEndian>() {
              Ok(n) => n,
              Err(_) => return None,
            };

            match num3 {
              // "0       ", " 0      ", "  0     ", "   0    ", "    0   ",
              // "     0  ", "      0 ", "       0"
              3467807035425300512 | 2319389130445824032
              | 2314903123004497952 | 2314885599537930272
              | 2314885531086888992 | 2314885530819502112
              | 2314885530818457632 | 2314885530818453552 => Some(Rbody0),
              // "1       ", " 1      ", "  1     ", "   1    ", "    1   ",
              // "     1  ", "      1 ", "       1"
              3539864629463228448 | 2319670605422534688
              | 2314904222516125728 | 2314885603832897568
              | 2314885531103666208 | 2314885530819567648
              | 2314885530818457888 | 2314885530818453553 => Some(Rbody1),
              // "2       ", " 2      ", "  2     ", "   2    ", "    2   ",
              // "     2  ", "      2 ", "       2"
              3611922223501156384 | 2319952080399245344
              | 2314905322027753504 | 2314885608127864864
              | 2314885531120443424 | 2314885530819633184
              | 2314885530818458144 | 2314885530818453554 => Some(Rbody2),
              // "3       ", " 3      ", "  3     ", "   3    ", "    3   ",
              // "     3  ", "      3 ", "       3"
              3683979817539084320 | 2320233555375956000
              | 2314906421539381280 | 2314885612422832160
              | 2314885531137220640 | 2314885530819698720
              | 2314885530818458400 | 2314885530818453555 => Some(Rbody3),
              _ => None,
            }
          }
        }
        // Auxiliaries
        // b"GROUP / "
        5139257352618258208 => Some(Group),
        _ => None,
      }
    }
  }
}

/// The [`Keywords`](::card::keyword::Keywords) struct to hold the keywords of a
/// [`Lines`](::lines::Lines) struct. Supposed to be kept in sync via
/// [`Keywords::update`](::card::keyword::Keywords::update).
#[derive(Debug, Default)]
pub struct Keywords(Vec<Option<Keyword>>);

impl Keywords {
  pub fn new() -> Self {
    Keywords(vec![])
  }

  pub fn clear(&mut self) {
    self.0.clear()
  }

  /// Extend a [`Keywords`](::card::keyword::Keywords) struct by parsing a
  /// [`Lines`](::lines::Lines) struct.
  pub fn parse_lines(&mut self, lines: &Lines) {
    self.0.extend(lines.iter().map(Keyword::parse))
  }

  /// Update a [`Keywords`](::card::keyword::Keywords) struct by parsing a
  /// `Vec<String>` and splicing in the result on the range `first..last`.
  pub fn update(&mut self, first: i64, last: i64, linedata: &[String]) {
    let range = first as usize..last as usize;
    let _ = self
      .0
      .splice(range, linedata.iter().map(|l| Keyword::parse(l.as_ref())));
  }

  // TODO(KillTheMule): Efficient? This is called a lot ...
  /// Find the index of the first line that starts with a non-comment keyword
  /// before the line with the given number. If the line with the given number
  /// itself starts with a non-comment keyword, its index is returned.
  pub fn first_before(&self, line: i64) -> i64 {
    self
      .get(..=line as usize)
      .unwrap_or(&[])
      .iter()
      .enumerate()
      .rfind(|(_i, k)| k.is_some() && **k != Some(Keyword::Comment))
      .unwrap_or((0, &None))
      .0 as i64
  }

  // TODO(KillTheMule): Efficient? This is called a lot ...
  /// Find the index of the next line that starts with a non-comment keyword
  /// after the line with the given number. If the line with the given number
  /// itself starts with a non-comment keyword, its index is returned.
  pub fn first_after(&self, line: i64) -> i64 {
    self
      .iter()
      .enumerate()
      .skip(line as usize)
      .find(|(_i, k)| k.is_some() && **k != Some(Keyword::Comment))
      .unwrap_or((self.len(), &None))
      .0 as i64
  }
}

impl Deref for Keywords {
  type Target = [Option<Keyword>];

  fn deref(&self) -> &[Option<Keyword>] {
    &self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::card::keyword::{Keyword::*, Keywords};

  #[test]
  fn first() {
    let kw = Keywords(vec![
      None,
      None,
      Some(Node),
      None,
      None,
      Some(Comment),
      Some(Node),
      Some(Comment),
      None,
    ]);

    assert_eq!(2, kw.first_before(2));
    assert_eq!(2, kw.first_after(2));

    assert_eq!(2, kw.first_before(4));
    assert_eq!(6, kw.first_after(4));

    assert_eq!(0, kw.first_before(1));
    assert_eq!(9, kw.first_after(7));
  }

  #[test]
  fn first_oneline() {
    let mut kw = Keywords(vec![Some(Node)]);
    assert_eq!(0, kw.first_before(0));
    assert_eq!(0, kw.first_after(0));

    kw = Keywords(vec![Some(Comment)]);
    assert_eq!(0, kw.first_before(0));
    assert_eq!(1, kw.first_after(0));
    assert_eq!(0, kw.first_before(1));
    assert_eq!(1, kw.first_after(1));
  }

}
