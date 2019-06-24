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
  IDMTO,
  IDNODi,
  IFRA1,
  ITMTO,
  ISENS,
  DOFTYP,
  IT1FL,
  MMTO,
  IDNODcog,
  Ixx,
  Iyy,
  Izz,
  Ixz,
  IFRA2,
  IDRB,
  ITRB,
  TFAIL,
  FAILD,
  AFAILN,
  AFAILS,
  A1,
  A2,
  INTF,
  D1,
  D2,
  TFAILD,
  IDNOD1,
  IDNOD2,
  IT3FL,
  Mrb
}
