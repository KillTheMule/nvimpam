local curbuf = vim.api.nvim_get_current_buf
local curpos = vim.api.nvim_win_get_cursor
local curwin = vim.api.nvim_get_current_win
local set_lines = vim.api.nvim_buf_set_lines
local command = vim.api.nvim_command
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function

local lines_from_file = require('nvimpam.utils').lines_from_file

local imp_status, impromptu = pcall(require, "impromptu")

local clone
if imp_status then
  clone = require('impromptu.utils').clone
end

local cardpath

local function padnum(d) return ("%03d%s"):format(#d, d) end

local function sort(a, b)
  return a.description:gsub("%d+",padnum) < b.description:gsub("%d+",padnum)
end

local function  filter_fn(filter_exprs, lines)
  local current = clone(lines)

  for _, filter_expr in ipairs(filter_exprs) do
    local tmp = {}

    for _, line in ipairs(current) do
      if string.find(line.description:lower(), filter_expr:lower()) then
        table.insert(tmp, line)
      end
    end

    current = tmp
  end

  table.sort(current, sort)
  return current
end

local function filter_cards()
  if not imp_status then
    command("echoerr 'Impromptu not installed, can not show filtered menu!'")
    return nil
  end

  if not cardpath then
    cardpath = call('finddir', {'lua/nvimpam/pam_cards', eval("&rtp")})
  end

  local curbuf = curbuf()
  local curwin = curwin()
  local curpos = curpos(curwin)

  local opts = {
    { description = "MMC Assign Definition", path = "MMC/mm.inc"},
    { description = "RUPMO Type 1", path = "Auxiliaries/ru1.inc"},
    { description = "LOOKU Lookup Table", path = "Auxiliaries/lo.inc"},
    { description = "FRICT Friction Model Type 13", path = "Auxiliaries/fr13.inc"},
    { description = "CDATA Card", path = "Auxiliaries/cd.inc"},
    { description = "PLANE Type 0", path = "Auxiliaries/pa0.inc"},
    { description = "SENSOR Type 14", path = "Auxiliaries/se14.inc"},
    { description = "UDATA User Data", path = "Auxiliaries/ud.inc"},
    { description = "RUPMO Type 3", path = "Auxiliaries/ru3.inc"},
    { description = "FRICT Friction Model Type 4", path = "Auxiliaries/fr4.inc"},
    { description = "RUPMO Type 5", path = "Auxiliaries/ru5.inc"},
    { description = "VECTOR Type 1", path = "Auxiliaries/ve1.inc"},
    { description = "SENSOR Type 1", path = "Auxiliaries/se1.inc"},
    { description = "FRICT Friction Model Type 5", path = "Auxiliaries/fr5.inc"},
    { description = "VECTOR Type 0", path = "Auxiliaries/ve0.inc"},
    { description = "RUPMO Type 0", path = "Auxiliaries/ru0.inc"},
    { description = "PLANE Type 2", path = "Auxiliaries/pa2.inc"},
    { description = "SENSOR Type 10", path = "Auxiliaries/se10.inc"},
    { description = "SENSOR Type 3", path = "Auxiliaries/se3.inc"},
    { description = "RUPMO Type 6", path = "Auxiliaries/ru6.inc"},
    { description = "RUPMO Type 7", path = "Auxiliaries/ru7.inc"},
    { description = "SENSOR Type 12", path = "Auxiliaries/se12.inc"},
    { description = "FRICT Friction Model Type 10", path = "Auxiliaries/fr10.inc"},
    { description = "SENSOR Type 2", path = "Auxiliaries/se2.inc"},
    { description = "FUNCSW Function Switch", path = "Auxiliaries/fw.inc"},
    { description = "SENSOR Type 11", path = "Auxiliaries/se11.inc"},
    { description = "SENSOR Type 4", path = "Auxiliaries/se4.inc"},
    { description = "FRAME IAXIS=0 U-based, 2 Vectors", path = "Auxiliaries/fm0.inc"},
    { description = "FRICT Friction Model Type 2", path = "Auxiliaries/fr2.inc"},
    { description = "SENSOR Type 5", path = "Auxiliaries/se5.inc"},
    { description = "FRAME IAXIS=4 Cylindrical", path = "Auxiliaries/fm4.inc"},
    { description = "SENSOR Type 6", path = "Auxiliaries/se6.inc"},
    { description = "FRICT Friction Model Type 3", path = "Auxiliaries/fr3.inc"},
    { description = "RUPMO Type 2", path = "Auxiliaries/ru2.inc"},
    { description = "SENSOR Type 7", path = "Auxiliaries/se7.inc"},
    { description = "SENSOR Type 8", path = "Auxiliaries/se8.inc"},
    { description = "FRICT Friction Model Type 11", path = "Auxiliaries/fr11.inc"},
    { description = "SURFA Surface Definition", path = "Auxiliaries/sr.inc"},
    { description = "SENSOR Type 9", path = "Auxiliaries/se9.inc"},
    { description = "PYFUNC Python Function", path = "Auxiliaries/pf.inc"},
    { description = "DELEM - Deleted Element Card", path = "Auxiliaries/de.inc"},
    { description = "FUNCT Function Card", path = "Auxiliaries/fc.inc"},
    { description = "GROUP Group Definition", path = "Auxiliaries/gr.inc"},
    { description = "FRICT Friction Model Type 5", path = "Auxiliaries/fr6.inc"},
    { description = "SENSOR Type 13", path = "Auxiliaries/se13.inc"},
    { description = "FRAME IAXIS=2 T-based, 2 Vectors", path = "Auxiliaries/fm2.inc"},
    { description = "FRICT Friction Model Type 12", path = "Auxiliaries/fr12.inc"},
    { description = "FRICT Friction Model Type 1", path = "Auxiliaries/fr1.inc"},
    { description = "FRAME IAXIS=5 Spherical", path = "Auxiliaries/fm5.inc"},
    { description = "NLAVE Non Local Averadge Definition", path = "Auxiliaries/nl.inc"},
    { description = "FRAME IAXIS=3 T-based, 3 Nodes", path = "Auxiliaries/fm3.inc"},
    { description = "PLANE Type 1", path = "Auxiliaries/pa1.inc"},
    { description = "FRAME IAXIS=1 U-based, 3 Nodes", path = "Auxiliaries/fm1.inc"},
    { description = "MERIC Keyword", path = "Control/me.inc"},
    { description = "IMPORT Card", path = "Control/im.inc"},
    { description = "DRAPF Draping File Import", path = "Control/dr.inc"},
    { description = "EXPORT Card", path = "Control/ex.inc"},
    { description = "MSTRM Mass Trimming", path = "Control/mr.inc"},
    { description = "PYVAR Variable Definition", path = "Control/py.inc"},
    { description = "INCLU Keyword", path = "Control/in.inc"},
    { description = "SUBDF Substructure Definition", path = "Control/su.inc"},
    { description = "TRSFM Transformation Card", path = "Control/tr.inc"},
    { description = "RMSSOL Shell-Solid Remeshing", path = "Control/ss.inc"},
    { description = "ORTHF Orientation File Import", path = "Control/or.inc"},
    { description = "DMPEW User DMP Scaling Factors", path = "Control/dm.inc"},
    { description = "Gratitational Acceleration", path = "Others/gv.inc"},
    { description = "FUNCT Card with Sinus Function", path = "Others/fs.inc"},
    { description = "NSMAS2 - Nonstructural mass Type 2", path = "Node/nm2.inc"},
    { description = "NSMAS - Nonstructural mass", path = "Node/nm.inc"},
    { description = "MASS Card", path = "Node/ms.inc"},
    { description = "NODE Card", path = "Node/nn.inc"},
    { description = "CNODE Card", path = "Node/cn.inc"},
    { description = "Contact Type 54", path = "Contact/c54.inc"},
    { description = "Contact Type 33", path = "Contact/c33.inc"},
    { description = "Contact Type 10", path = "Contact/c10.inc"},
    { description = "Contact Type 46", path = "Contact/c46.inc"},
    { description = "Contact Type 21", path = "Contact/c21.inc"},
    { description = "Contact Type 154", path = "Contact/c154.inc"},
    { description = "Contact Type 61", path = "Contact/c61.inc"},
    { description = "Contact Type 44", path = "Contact/c44.inc"},
    { description = "Contact Type 36", path = "Contact/c36.inc"},
    { description = "Contact Type 1", path = "Contact/c1.inc"},
    { description = "Contact Type 43", path = "Contact/c43.inc"},
    { description = "Contact Type 37", path = "Contact/c37.inc"},
    { description = "Contact Type 34", path = "Contact/c34.inc"},
    { description = "Mater Type 62 (CURVE Definition)", path = "Material/m62.inc"},
    { description = "Mater Type 17", path = "Material/m17.inc"},
    { description = "Mater Type 18", path = "Material/m18.inc"},
    { description = "Mater Type 128", path = "Material/m128.inc"},
    { description = "Mater Type 102 (CURVE Definition)", path = "Material/m102.inc"},
    { description = "VA Mater Type 5", path = "Material/ma5.inc"},
    { description = "Mater Type 24", path = "Material/m24.inc"},
    { description = "Mater Type 105 (CURVE Definition, HSR Damage)", path = "Material/m105.inc"},
    { description = "Mater Type 109 (CURVE Definition)", path = "Material/m109.inc"},
    { description = "Mater Type 22", path = "Material/m22.inc"},
    { description = "Mater Type 131", path = "Material/m131.inc"},
    { description = "Mater Type 35 (CURVE Definition)", path = "Material/m35.inc"},
    { description = "Mater Type 101", path = "Material/m101.inc"},
    { description = "Mater Type 103 (CURVE Definition)", path = "Material/m103.inc"},
    { description = "Mater Type 115 (CURVE Definition)", path = "Material/m115.inc"},
    { description = "Mater Type 201", path = "Material/m201.inc"},
    { description = "Mater Type 301", path = "Material/m301.inc"},
    { description = "Mater Type 121", path = "Material/m121.inc"},
    { description = "VA Mater Type 3", path = "Material/ma3.inc"},
    { description = "Mater Type 7", path = "Material/m7.inc"},
    { description = "Mater Type 99", path = "Material/m99.inc"},
    { description = "Mater Type 41", path = "Material/m41.inc"},
    { description = "Mater Type 162 (CURVE Definition)", path = "Material/m162.inc"},
    { description = "Mater Type 226", path = "Material/m226.inc"},
    { description = "Mater Type 15", path = "Material/m15.inc"},
    { description = "Mater Type 161", path = "Material/m161.inc"},
    { description = "Mater Type 71 (CURVE Definition)", path = "Material/m71.inc"},
    { description = "Mater Type 200", path = "Material/m200.inc"},
    { description = "Mater Type 118 (CURVE Definition)", path = "Material/m118.inc"},
    { description = "VA Mater Type 1", path = "Material/ma1.inc"},
    { description = "LAYER Material Card", path = "Material/la.inc"},
    { description = "Mater Type 26 (CURVE Definition)", path = "Material/m26.inc"},
    { description = "Mater Type 8", path = "Material/m8.inc"},
    { description = "Mater Type 305", path = "Material/m305.inc"},
    { description = "Mater Type 204", path = "Material/m204.inc"},
    { description = "Mater Type 205", path = "Material/m205.inc"},
    { description = "Mater Type 212 (CURVE Definition)", path = "Material/m212.inc"},
    { description = "Mater Type 151", path = "Material/m151.inc"},
    { description = "Mater Type 126", path = "Material/m126.inc"},
    { description = "Mater Type 213 (CURVE Definition)", path = "Material/m213.inc"},
    { description = "Mater Type 214", path = "Material/m214.inc"},
    { description = "Mater Type 221", path = "Material/m221.inc"},
    { description = "Mater Type 19", path = "Material/m19.inc"},
    { description = "Mater Type 6", path = "Material/m6.inc"},
    { description = "PLY Type 8", path = "Material/ply8.inc"},
    { description = "Mater Type 223", path = "Material/m223.inc"},
    { description = "Mater Type 1 (CURVE Definition)", path = "Material/m1.inc"},
    { description = "Mater Type 12", path = "Material/m12.inc"},
    { description = "Mater Type 171 (CURVE Definition)", path = "Material/m171.inc"},
    { description = "Mater Type 303", path = "Material/m303.inc"},
    { description = "Mater Type 203", path = "Material/m203.inc"},
    { description = "Mater Type 371", path = "Material/m371.inc"},
    { description = "Mater Type 21", path = "Material/m21.inc"},
    { description = "Mater Type 225", path = "Material/m225.inc"},
    { description = "Mater Type 25", path = "Material/m25.inc"},
    { description = "Mater Type 28", path = "Material/m28.inc"},
    { description = "VA Mater Type 4", path = "Material/ma4.inc"},
    { description = "PLY Material Card", path = "Material/ply0.inc"},
    { description = "PLY Type 1", path = "Material/ply1.inc"},
    { description = "PLY Type 2", path = "Material/ply2.inc"},
    { description = "PLY Type 4", path = "Material/ply4.inc"},
    { description = "Mater Type 110", path = "Material/m110.inc"},
    { description = "PLY Type 6", path = "Material/ply6.inc"},
    { description = "Mater Type 152", path = "Material/m152.inc"},
    { description = "Mater Type 61", path = "Material/m61.inc"},
    { description = "THMAT Thermal Material", path = "Material/mt.inc"},
    { description = "Mater Type 106 (CURVE Definition)", path = "Material/m106.inc"},
    { description = "Mater Type 38", path = "Material/m38.inc"},
    { description = "Mater Type 202 (CURVE Definition)", path = "Material/m202.inc"},
    { description = "Mater Type 304", path = "Material/m304.inc"},
    { description = "Mater Type 42", path = "Material/m42.inc"},
    { description = "Mater Type 30", path = "Material/m30.inc"},
    { description = "Mater Type 132", path = "Material/m132.inc"},
    { description = "Mater Type 230", path = "Material/m230.inc"},
    { description = "VA Mater Type 2", path = "Material/ma2.inc"},
    { description = "Mater Type 143", path = "Material/m143.inc"},
    { description = "Mater Type 127 (CURVE Definition)", path = "Material/m127.inc"},
    { description = "Mater Type 224", path = "Material/m224.inc"},
    { description = "Mater Type 100", path = "Material/m100.inc"},
    { description = "Mater Type 220", path = "Material/m220.inc"},
    { description = "Mater Type 52 (CURVE Definition)", path = "Material/m52.inc"},
    { description = "PLY Type 7", path = "Material/ply7.inc"},
    { description = "Mater Type 20", path = "Material/m20.inc"},
    { description = "Mater Type 37", path = "Material/m37.inc"},
    { description = "PFMAT Porous Material", path = "Material/mp.inc"},
    { description = "Mater Type 108 (CURVE Definition)", path = "Material/m108.inc"},
    { description = "Mater Type 130", path = "Material/m130.inc"},
    { description = "Mater Type 302", path = "Material/m302.inc"},
    { description = "Mater Type 36", path = "Material/m36.inc"},
    { description = "PLY Type 5", path = "Material/ply5.inc"},
    { description = "PLY Type 3", path = "Material/ply3.inc"},
    { description = "Mater Type 222", path = "Material/m222.inc"},
    { description = "Mater Type 51", path = "Material/m51.inc"},
    { description = "Mater Type 47", path = "Material/m47.inc"},
    { description = "Mater Type 116 (CURVE Definition)", path = "Material/m116.inc"},
    { description = "Mater Type 150", path = "Material/m150.inc"},
    { description = "Mater Type 2", path = "Material/m2.inc"},
    { description = "Mater Type 5", path = "Material/m5.inc"},
    { description = "Mater Type 117 (CURVE Definition)", path = "Material/m117.inc"},
    { description = "Mater Type 16 (CURVE Definition)", path = "Material/m16.inc"},
    { description = "Mater Type 31", path = "Material/m31.inc"},
    { description = "Mater Type 45", path = "Material/m45.inc"},
    { description = "Mater Type 11", path = "Material/m11.inc"},
    { description = "MTOCO (Regular)", path = "Constraint/to0.inc"},
    { description = "SEWING Definition", path = "Constraint/sew.inc"},
    { description = "LINCO Linear Constraint", path = "Constraint/lc.inc"},
    { description = "RBODY Type 3", path = "Constraint/rb3.inc"},
    { description = "OTMCO Constraint", path = "Constraint/oo.inc"},
    { description = "MTOCO (User Imposed Mass and Intertia)", path = "Constraint/to1.inc"},
    { description = "RBODY Type 1", path = "Constraint/rb1.inc"},
    { description = "RBODY Type 0", path = "Constraint/rb0.inc"},
    { description = "RBODY Type 4", path = "Constraint/rb4.inc"},
    { description = "RBODY Type 2", path = "Constraint/rb2.inc"},
    { description = "PART Type GAP", path = "Part/pga.inc"},
    { description = "PART Type MBSPR", path = "Part/pms.inc"},
    { description = "PART Type JOINT", path = "Part/pjo.inc"},
    { description = "PART Type LLINK", path = "Part/pll.inc"},
    { description = "PART Type ELINK", path = "Part/ple.inc"},
    { description = "PART Type SLINK", path = "Part/pls.inc"},
    { description = "PART Type MEMBR", path = "Part/pme.inc"},
    { description = "MPART Type SHELL", path = "Part/pos.inc"},
    { description = "PART Type SPRING", path = "Part/psp.inc"},
    { description = "PART Type TETRA", path = "Part/pte.inc"},
    { description = "MPART Type SOLID", path = "Part/poo.inc"},
    { description = "PART Type TIED", path = "Part/pti.inc"},
    { description = "PART Type BSHEL", path = "Part/pbs.inc"},
    { description = "PART Type BAR", path = "Part/pba.inc"},
    { description = "PART Type SPRGBM", path = "Part/psb.inc"},
    { description = "PART Type MBKJN", path = "Part/pmk.inc"},
    { description = "PART Type MTOJNT", path = "Part/pmt.inc"},
    { description = "PART Type PLINK", path = "Part/plp.inc"},
    { description = "PART Type SHELL", path = "Part/psh.inc"},
    { description = "PART Type TSHEL", path = "Part/pts.inc"},
    { description = "PART Type KJOIN", path = "Part/pkj.inc"},
    { description = "PART Type BEAM", path = "Part/pbe.inc"},
    { description = "PART Type SPHEL", path = "Part/pse.inc"},
    { description = "PART Type COS3D", path = "Part/pco.inc"},
    { description = "PART Type SOLID", path = "Part/pso.inc"},
    { description = "THCRS Cross Spectral Density Printout", path = "Output/thc.inc"},
    { description = "THNAC Acoustic Output", path = "Output/tha.inc"},
    { description = "THNOD Output", path = "Output/thn.inc"},
    { description = "VAPANL Structural Panel", path = "Output/vap.inc"},
    { description = "SECFO Type CONT_MS", path = "Output/sfm.inc"},
    { description = "THLOC Output", path = "Output/thl.inc"},
    { description = "SECFO Type PLANE", path = "Output/sfp.inc"},
    { description = "SEFCO Type CONTACT", path = "Output/sfc.inc"},
    { description = "SECFO Type VOLFRAC", path = "Output/sfv.inc"},
    { description = "SECFO Type SECTION", path = "Output/sfs.inc"},
    { description = "THNPO Porous Nodal Output", path = "Output/thp.inc"},
    { description = "SELOUT Selective Output", path = "Output/sel.inc"},
    { description = "SENPTG Sensor Point (Global)", path = "Output/seg.inc"},
    { description = "SENPT Sensor Point (Local)", path = "Output/sen.inc"},
    { description = "THELE Card", path = "Output/the.inc"},
    { description = "SECFO Type LINK", path = "Output/sfl.inc"},
    { description = "SECTION Type SUPPORT", path = "Output/sfu.inc"},
    { description = "SOLID Element", path = "Element/eso.inc"},
    { description = "HEXA20 Element", path = "Element/ehe.inc"},
    { description = "BAR Element", path = "Element/eba.inc"},
    { description = "SPRGBM Element", path = "Element/esb.inc"},
    { description = "ELINK Element", path = "Element/ele.inc"},
    { description = "BSHEL Element", path = "Element/ebs.inc"},
    { description = "SPRING Element", path = "Element/esp.inc"},
    { description = "SHEL8 Element", path = "Element/es8.inc"},
    { description = "IMPMA Super Element Matrix Import", path = "Element/esu.inc"},
    { description = "SHELL Element", path = "Element/esh.inc"},
    { description = "LLINK Element", path = "Element/ell.inc"},
    { description = "KJOIN Element", path = "Element/ekj.inc"},
    { description = "TETR4 Element", path = "Element/et4.inc"},
    { description = "PENTA6 Element ", path = "Element/ep6.inc"},
    { description = "TETRA Element", path = "Element/ete.inc"},
    { description = "TIED Element", path = "Element/eti.inc"},
    { description = "SPHEL Element", path = "Element/eph.inc"},
    { description = "TSHEL Element", path = "Element/ets.inc"},
    { description = "MEMBR Element", path = "Element/eme.inc"},
    { description = "TETR10 Element", path = "Element/et1.inc"},
    { description = "SHEL6 Element", path = "Element/es6.inc"},
    { description = "PLINK Element", path = "Element/elp.inc"},
    { description = "GAP Element", path = "Element/ega.inc"},
    { description = "BEAM Element", path = "Element/ebe.inc"},
    { description = "PENTA15 Element", path = "Element/ep1.inc"},
    { description = "JOINT Element", path = "Element/ejo.inc"},
    { description = "SPHELO Element", path = "Element/epo.inc"},
    { description = "MTOJNT Element", path = "Element/emt.inc"},
    { description = "SLINK Element", path = "Element/els.inc"},
    { description = "MUSC1 Muscle Element Definition", path = "Safety/mus.inc"},
    { description = "SLIPR Slipring Definition", path = "Safety/sli.inc"},
    { description = "GASPEC Gas Specification", path = "Safety/gas.inc"},
    { description = "RETRA Belt Retractor Definition", path = "Safety/ret.inc"},
    { description = "BAGIN Definition", path = "Safety/bag.inc"},
    { description = "CONLO Concentrated Load", path = "Load/cl.inc"},
    { description = "VAMPSO Acoustic Monopole Source", path = "Load/am.inc"},
    { description = "FBC3D Prescribed Motion onto Fluid Media", path = "Load/fb.inc"},
    { description = "BDFOR Body Forces", path = "Load/bd.inc"},
    { description = "DETOP Detonation Point", path = "Load/dp.inc"},
    { description = "ACFLD Acceleration Field", path = "Load/ac.inc"},
    { description = "DFLUX Definition", path = "Load/df.inc"},
    { description = "BOUNC Displacement BC", path = "Load/bc.inc"},
    { description = "FBCFA Prescribed Surface Normal BC onto Fluid", path = "Load/fn.inc"},
    { description = "3D Boundary Condition", path = "Load/3d.inc"},
    { description = "DISLIM Displacement Limitation", path = "Load/dl.inc"},
    { description = "ACTUA - Joint Actuator Definition", path = "Load/at.inc"},
    { description = "PRESBC Pressure Porous BC", path = "Load/bp.inc"},
    { description = "VAABSO Acoustic Absorber", path = "Load/ab.inc"},
    { description = "Acoustic Plane Wave", path = "Load/ap.inc"},
    { description = "DAMP Nodal Damping", path = "Load/da.inc"},
    { description = "PREBM Beam Pressure", path = "Load/ib.inc"},
    { description = "PREFA Face Pressure", path = "Load/if.inc"},
    { description = "KINDA Kinematic Damping", path = "Load/kd.inc"},
    { description = "RMLOAD Resudial Mode Load", path = "Load/rm.inc"},
    { description = "LCPSD Power Spectral Density Function", path = "Load/sd.inc"},
    { description = "HTSURF Heat Exchange Surface", path = "Load/hs.inc"},
    { description = "HFLUX", path = "Load/hf.inc"},
    { description = "TEMBC Temperature BC", path = "Load/tc.inc"},
    { description = "TURBL Turbulent Boundary Layer Load", path = "Load/tu.inc"},
    { description = "BFLUX Definition", path = "Load/bf.inc"},
    { description = "INTEM Initial Temperature", path = "Load/it.inc"},
    { description = "PFSURF Porous Flow Exchange Surface", path = "Load/pu.inc"},
    { description = "INVEL Initial Velocity", path = "Load/iv.inc"},
    { description = "INPRES Initial Pressure", path = "Load/ip.inc"},
  }

  table.sort(opts, sort)

  impromptu.filter{
    title = "HEADER",
    options =  opts,
    handler = function(b, opt)
      set_lines(curbuf, curpos[1], curpos[1], false, lines_from_file(cardpath.."/"..opt.path))
      return true
    end,
    filter_fn = filter_fn,
  }     
end

return {
  filter_cards = filter_cards,
}
