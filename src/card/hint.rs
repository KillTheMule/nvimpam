use strum_macros::IntoStaticStr;

#[derive(IntoStaticStr, Debug, PartialEq, Clone, Copy)]
pub enum Hint {
  TITLE,
  IDNOD,
  X,
  Y,
  Z,
  IFRA,
  DISr,
  DISs,
  DISt,
  Mx,
  My,
  Mz,
  Ix,
  Iy,
  Iz,
  Ixy,
  Iyz,
  Izx,
  IDNSM,
  MASS,
  MLEN,
  MARE,
  MVOL,
  IDOTM,
  IDNODd,
  DOFCOD,
  IMETH,
  RADIUS,
  IELIM,
  ITYP,
  ALPHA,
  WTFAC,
  // Dummy!
  ADDNODWEIGHT,
}

/*
impl From<CellHint> for &'static str {
  fn from(c: CellHint) -> Self {
      IDNOD => "IDNOD",
      X => "X",
      Y => "Y",
      Z => "Z",
  }
}
*/
