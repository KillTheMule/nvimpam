local curbuf = vim.api.nvim_get_current_buf
local curpos = vim.api.nvim_win_get_cursor
local curwin = vim.api.nvim_get_current_win
local set_lines = vim.api.nvim_buf_set_lines
local command = vim.api.nvim_command
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function

local lines_from_file = require('nvimpam.utils').lines_from_file

local imp_status, impromptu = pcall(require, "impromptu")

local displen
if imp_status then
  displen = require('impromptu.utils').displaywidth
end

local cardpath

local function cols(obj, opts, window_ops)
  local width = window_ops.width
  local height = window_ops.height

  -- "Close this prompt" will be added
  local maxlen = 17
  for _, l in pairs(opts) do
    local len = displen(l, 1)
    if len > maxlen then
      maxlen = len
    end
  end

  local rows = height - 2 --leave a line of space at the top & bottom
  local cols_needed = math.ceil(#opts/rows)
  local cols_per_screen = math.max(math.floor(width/maxlen), 1)

  return math.min(cols_needed, cols_per_screen)
end

local function padnum(d) return ("%03d%s"):format(#d, d) end

local function sort(a, b)
  return a.description:gsub("%d+",padnum) < b.description:gsub("%d+",padnum)
end

local function lines_to_grid(opts, window_ops)
  local columns = cols(nil, opts, window_ops)
  local nr_lines = math.ceil(#opts/columns)
  local grid = {}

  for column = 1, columns do
    local col = {}
    table.insert(col, "")
    for line = 1, nr_lines do

      local k = opts[(column-1)*nr_lines + line]
      if k ~= nil then
        local sub1 = string.sub(k, 0, window_ops.width - 1)
        table.insert(col, " "..sub1)
        k = string.sub(k, window_ops.width)

        while k ~= ""
        do
          table.insert(col, "     "..string.sub(k, 0, window_ops.width - 5))
          k = string.sub(k, window_ops.width - 4)
        end
      end

    end
    table.insert(grid, col)
  end

  return grid
end

local function cardmenu()
  if not imp_status then
    command("echoerr 'Impromptu not installed, can not show menu!'")
    return nil
  end

  if not cardpath then
    cardpath = call('finddir', {'lua/nvimpam/pam_cards', eval("&rtp")})
  end

  local curbuf = curbuf()
  local curwin = curwin()
  local curpos = curpos(curwin)

  local opts = {
    Auxiliaries = {
      description = "Auxiliaries",
      children = {
        ["cd.inc"] = { description = "CDATA Card" , file="cd.inc" },
        ["de.inc"] = { description = "DELEM - Deleted Element Card" , file="de.inc" },
        ["fc.inc"] = { description = "FUNCT Function Card" , file="fc.inc" },
        ["fw.inc"] = { description = "FUNCSW Function Switch" , file="fw.inc" },
        ["gr.inc"] = { description = "GROUP Group Definition" , file="gr.inc" },
        ["lo.inc"] = { description = "LOOKU Lookup Table" , file="lo.inc" },
        ["nl.inc"] = { description = "NLAVE Non Local Averadge Definition" , file="nl.inc" },
        ["pf.inc"] = { description = "PYFUNC Python Function" , file="pf.inc" },
        ["sr.inc"] = { description = "SURFA Surface Definition" , file="sr.inc" },
        ["ud.inc"] = { description = "UDATA User Data" , file="ud.inc" },
        ["ve0.inc"] = { description = "VECTOR Type 0" , file="ve0.inc" },
        ["ve1.inc"] = { description = "VECTOR Type 1" , file="ve1.inc" },
        ["PLANE"] = {
          description = "PLANEs",
          children = {
            ["pa0.inc"] = { description = "PLANE Type 0" , file="pa0.inc" },
            ["pa1.inc"] = { description = "PLANE Type 1" , file="pa1.inc" },
            ["pa2.inc"] = { description = "PLANE Type 2" , file="pa2.inc" },
          },
        },
        ["FRAME"] = {
          description = "FRAMEs",
          children = {
            ["fm0.inc"] = { description = "FRAME IAXIS=0 U-based, 2 Vectors" , file="fm0.inc" },
            ["fm1.inc"] = { description = "FRAME IAXIS=1 U-based, 3 Nodes" , file="fm1.inc" },
            ["fm2.inc"] = { description = "FRAME IAXIS=2 T-based, 2 Vectors" , file="fm2.inc" },
            ["fm3.inc"] = { description = "FRAME IAXIS=3 T-based, 3 Nodes" , file="fm3.inc" },
            ["fm4.inc"] = { description = "FRAME IAXIS=4 Cylindrical" , file="fm4.inc" },
            ["fm5.inc"] = { description = "FRAME IAXIS=5 Spherical" , file="fm5.inc" },
          },
        },
        ["FRICT"] = {
          description = "FRICTion Models",
          childen = {
            ["fr1.inc"] = { description = "FRICT Friction Model Type 1" , file="fr1.inc" },
            ["fr10.inc"] = { description = "FRICT Friction Model Type 10" , file="fr10.inc" },
            ["fr11.inc"] = { description = "FRICT Friction Model Type 11" , file="fr11.inc" },
            ["fr12.inc"] = { description = "FRICT Friction Model Type 12" , file="fr12.inc" },
            ["fr13.inc"] = { description = "FRICT Friction Model Type 13" , file="fr13.inc" },
            ["fr2.inc"] = { description = "FRICT Friction Model Type 2" , file="fr2.inc" },
            ["fr3.inc"] = { description = "FRICT Friction Model Type 3" , file="fr3.inc" },
            ["fr4.inc"] = { description = "FRICT Friction Model Type 4" , file="fr4.inc" },
            ["fr5.inc"] = { description = "FRICT Friction Model Type 5" , file="fr5.inc" },
            ["fr6.inc"] = { description = "FRICT Friction Model Type 5" , file="fr6.inc" },
          },
        },
        ["RUPMO"] = {
          description = "RUPMOs",
          childen = {
            ["ru0.inc"] = { description = "RUPMO Type 0" , file="ru0.inc" },
            ["ru1.inc"] = { description = "RUPMO Type 1" , file="ru1.inc" },
            ["ru2.inc"] = { description = "RUPMO Type 2" , file="ru2.inc" },
            ["ru3.inc"] = { description = "RUPMO Type 3" , file="ru3.inc" },
            ["ru5.inc"] = { description = "RUPMO Type 5" , file="ru5.inc" },
            ["ru6.inc"] = { description = "RUPMO Type 6" , file="ru6.inc" },
            ["ru7.inc"] = { description = "RUPMO Type 7" , file="ru7.inc" },
          },
        },
        ["SENSOR"] = {
          description = "SENSORs",
          children = {
            ["se1.inc"] = { description = "SENSOR Type 1" , file="se1.inc" },
            ["se10.inc"] = { description = "SENSOR Type 10" , file="se10.inc" },
            ["se11.inc"] = { description = "SENSOR Type 11" , file="se11.inc" },
            ["se12.inc"] = { description = "SENSOR Type 12" , file="se12.inc" },
            ["se13.inc"] = { description = "SENSOR Type 13" , file="se13.inc" },
            ["se14.inc"] = { description = "SENSOR Type 14" , file="se14.inc" },
            ["se2.inc"] = { description = "SENSOR Type 2" , file="se2.inc" },
            ["se3.inc"] = { description = "SENSOR Type 3" , file="se3.inc" },
            ["se4.inc"] = { description = "SENSOR Type 4" , file="se4.inc" },
            ["se5.inc"] = { description = "SENSOR Type 5" , file="se5.inc" },
            ["se6.inc"] = { description = "SENSOR Type 6" , file="se6.inc" },
            ["se7.inc"] = { description = "SENSOR Type 7" , file="se7.inc" },
            ["se8.inc"] = { description = "SENSOR Type 8" , file="se8.inc" },
            ["se9.inc"] = { description = "SENSOR Type 9" , file="se9.inc" },
          },
        },
      },
    },
    Constraint = {
      description = "Constraint",
      children = {
        ["lc.inc"] = { description = "LINCO Linear Constraint" , file="lc.inc" },
        ["oo.inc"] = { description = "OTMCO Constraint" , file="oo.inc" },
        ["rb0.inc"] = { description = "RBODY Type 0" , file="rb0.inc" },
        ["rb1.inc"] = { description = "RBODY Type 1" , file="rb1.inc" },
        ["rb2.inc"] = { description = "RBODY Type 2" , file="rb2.inc" },
        ["rb3.inc"] = { description = "RBODY Type 3" , file="rb3.inc" },
        ["rb4.inc"] = { description = "RBODY Type 4" , file="rb4.inc" },
        ["sew.inc"] = { description = "SEWING Definition" , file="sew.inc" },
        ["to0.inc"] = { description = "MTOCO (Regular)" , file="to0.inc" },
        ["to1.inc"] = { description = "MTOCO (User Imposed Mass and Intertia)" , file="to1.inc" },
      }
    },
    Contact = {
      description = "Contact",
      children = {
        ["c1.inc"] = { description = "Contact Type 1" , file="c1.inc" },
        ["c10.inc"] = { description = "Contact Type 10" , file="c10.inc" },
        ["c154.inc"] = { description = "Contact Type 154" , file="c154.inc" },
        ["c21.inc"] = { description = "Contact Type 21" , file="c21.inc" },
        ["c33.inc"] = { description = "Contact Type 33" , file="c33.inc" },
        ["c34.inc"] = { description = "Contact Type 34" , file="c34.inc" },
        ["c36.inc"] = { description = "Contact Type 36" , file="c36.inc" },
        ["c37.inc"] = { description = "Contact Type 37" , file="c37.inc" },
        ["c43.inc"] = { description = "Contact Type 43" , file="c43.inc" },
        ["c44.inc"] = { description = "Contact Type 44" , file="c44.inc" },
        ["c46.inc"] = { description = "Contact Type 46" , file="c46.inc" },
        ["c54.inc"] = { description = "Contact Type 54" , file="c54.inc" },
        ["c61.inc"] = { description = "Contact Type 61" , file="c61.inc" },
      }
    },
    Control = {
      description = "Control",
      children = {
        ["dm.inc"] = { description = "DMPEW User DMP Scaling Factors" , file="dm.inc" },
        ["dr.inc"] = { description = "DRAPF Draping File Import" , file="dr.inc" },
        ["ex.inc"] = { description = "EXPORT Card" , file="ex.inc" },
        ["im.inc"] = { description = "IMPORT Card" , file="im.inc" },
        ["in.inc"] = { description = "INCLU Keyword" , file="in.inc" },
        ["me.inc"] = { description = "MERIC Keyword" , file="me.inc" },
        ["mr.inc"] = { description = "MSTRM Mass Trimming" , file="mr.inc" },
        ["or.inc"] = { description = "ORTHF Orientation File Import" , file="or.inc" },
        ["py.inc"] = { description = "PYVAR Variable Definition" , file="py.inc" },
        ["ss.inc"] = { description = "RMSSOL Shell-Solid Remeshing" , file="ss.inc" },
        ["su.inc"] = { description = "SUBDF Substructure Definition" , file="su.inc" },
        ["tr.inc"] = { description = "TRSFM Transformation Card" , file="tr.inc" },
      }
    },
    Element = {
      description = "Element",
      children = {
        ["eba.inc"] = { description = "BAR Element" , file="eba.inc" },
        ["ebe.inc"] = { description = "BEAM Element" , file="ebe.inc" },
        ["ebs.inc"] = { description = "BSHEL Element" , file="ebs.inc" },
        ["ega.inc"] = { description = "GAP Element" , file="ega.inc" },
        ["ehe.inc"] = { description = "HEXA20 Element" , file="ehe.inc" },
        ["ejo.inc"] = { description = "JOINT Element" , file="ejo.inc" },
        ["ekj.inc"] = { description = "KJOIN Element" , file="ekj.inc" },
        ["ele.inc"] = { description = "ELINK Element" , file="ele.inc" },
        ["ell.inc"] = { description = "LLINK Element" , file="ell.inc" },
        ["elp.inc"] = { description = "PLINK Element" , file="elp.inc" },
        ["els.inc"] = { description = "SLINK Element" , file="els.inc" },
        ["eme.inc"] = { description = "MEMBR Element" , file="eme.inc" },
        ["emt.inc"] = { description = "MTOJNT Element" , file="emt.inc" },
        ["ep1.inc"] = { description = "PENTA15 Element" , file="ep1.inc" },
        ["ep6.inc"] = { description = "PENTA6 Element " , file="ep6.inc" },
        ["eph.inc"] = { description = "SPHEL Element" , file="eph.inc" },
        ["epo.inc"] = { description = "SPHELO Element" , file="epo.inc" },
        ["es6.inc"] = { description = "SHEL6 Element" , file="es6.inc" },
        ["es8.inc"] = { description = "SHEL8 Element" , file="es8.inc" },
        ["esb.inc"] = { description = "SPRGBM Element" , file="esb.inc" },
        ["esh.inc"] = { description = "SHELL Element" , file="esh.inc" },
        ["eso.inc"] = { description = "SOLID Element" , file="eso.inc" },
        ["esp.inc"] = { description = "SPRING Element" , file="esp.inc" },
        ["esu.inc"] = { description = "IMPMA Super Element Matrix Import" , file="esu.inc" },
        ["et1.inc"] = { description = "TETR10 Element" , file="et1.inc" },
        ["et4.inc"] = { description = "TETR4 Element" , file="et4.inc" },
        ["ete.inc"] = { description = "TETRA Element" , file="ete.inc" },
        ["eti.inc"] = { description = "TIED Element" , file="eti.inc" },
        ["ets.inc"] = { description = "TSHEL Element" , file="ets.inc" },
      }
    },
    Load = {
      description = "Load",
      children = {
        ["3d.inc"] = { description = "3D Boundary Condition" , file="3d.inc" },
        ["ab.inc"] = { description = "VAABSO Acoustic Absorber" , file="ab.inc" },
        ["ac.inc"] = { description = "ACFLD Acceleration Field" , file="ac.inc" },
        ["am.inc"] = { description = "VAMPSO Acoustic Monopole Source" , file="am.inc" },
        ["ap.inc"] = { description = "Acoustic Plane Wave" , file="ap.inc" },
        ["at.inc"] = { description = "ACTUA - Joint Actuator Definition" , file="at.inc" },
        ["bc.inc"] = { description = "BOUNC Displacement BC" , file="bc.inc" },
        ["bd.inc"] = { description = "BDFOR Body Forces" , file="bd.inc" },
        ["bf.inc"] = { description = "BFLUX Definition" , file="bf.inc" },
        ["bp.inc"] = { description = "PRESBC Pressure Porous BC" , file="bp.inc" },
        ["cl.inc"] = { description = "CONLO Concentrated Load" , file="cl.inc" },
        ["da.inc"] = { description = "DAMP Nodal Damping" , file="da.inc" },
        ["df.inc"] = { description = "DFLUX Definition" , file="df.inc" },
        ["dl.inc"] = { description = "DISLIM Displacement Limitation" , file="dl.inc" },
        ["dp.inc"] = { description = "DETOP Detonation Point" , file="dp.inc" },
        ["fb.inc"] = { description = "FBC3D Prescribed Motion onto Fluid Media" , file="fb.inc" },
        ["fn.inc"] = { description = "FBCFA Prescribed Surface Normal BC onto Fluid" , file="fn.inc" },
        ["hf.inc"] = { description = "HFLUX" , file="hf.inc" },
        ["hs.inc"] = { description = "HTSURF Heat Exchange Surface" , file="hs.inc" },
        ["ib.inc"] = { description = "PREBM Beam Pressure" , file="ib.inc" },
        ["if.inc"] = { description = "PREFA Face Pressure" , file="if.inc" },
        ["ip.inc"] = { description = "INPRES Initial Pressure" , file="ip.inc" },
        ["it.inc"] = { description = "INTEM Initial Temperature" , file="it.inc" },
        ["iv.inc"] = { description = "INVEL Initial Velocity" , file="iv.inc" },
        ["kd.inc"] = { description = "KINDA Kinematic Damping" , file="kd.inc" },
        ["pu.inc"] = { description = "PFSURF Porous Flow Exchange Surface" , file="pu.inc" },
        ["rm.inc"] = { description = "RMLOAD Resudial Mode Load" , file="rm.inc" },
        ["sd.inc"] = { description = "LCPSD Power Spectral Density Function" , file="sd.inc" },
        ["tc.inc"] = { description = "TEMBC Temperature BC" , file="tc.inc" },
        ["tu.inc"] = { description = "TURBL Turbulent Boundary Layer Load" , file="tu.inc" },
      }
    },
    MMC = {
      description = "MMC",
      children = {
        ["mm.inc"] = { description = "MMC Assign Definition" , file="mm.inc" },
      }
    },
    Material = {
      description = "Material",
      children = {
        ["SOLID Materials"] = {
          description = "SOLID Materials",
          children = {
            ["m1.inc"] = { description = "Mater Type 1 (CURVE Definition)" , file="m1.inc" },
            ["m2.inc"] = { description = "Mater Type 2" , file="m2.inc" },
            ["m5.inc"] = { description = "Mater Type 5" , file="m5.inc" },
            ["m6.inc"] = { description = "Mater Type 6" , file="m6.inc" },
            ["m7.inc"] = { description = "Mater Type 7" , file="m7.inc" },
            ["m8.inc"] = { description = "Mater Type 8" , file="m8.inc" },
            ["m11.inc"] = { description = "Mater Type 11" , file="m11.inc" },
            ["m12.inc"] = { description = "Mater Type 12" , file="m12.inc" },
            ["m15.inc"] = { description = "Mater Type 15" , file="m15.inc" },
            ["m16.inc"] = { description = "Mater Type 16 (CURVE Definition)" , file="m16.inc" },
            ["m17.inc"] = { description = "Mater Type 17" , file="m17.inc" },
            ["m18.inc"] = { description = "Mater Type 18" , file="m18.inc" },
            ["m19.inc"] = { description = "Mater Type 19" , file="m19.inc" },
            ["m20.inc"] = { description = "Mater Type 20" , file="m20.inc" },
            ["m21.inc"] = { description = "Mater Type 21" , file="m21.inc" },
            ["m22.inc"] = { description = "Mater Type 22" , file="m22.inc" },
            ["m24.inc"] = { description = "Mater Type 24" , file="m24.inc" },
            ["m25.inc"] = { description = "Mater Type 25" , file="m25.inc" },
            ["m26.inc"] = { description = "Mater Type 26 (CURVE Definition)" , file="m26.inc" },
            ["m28.inc"] = { description = "Mater Type 28" , file="m28.inc" },
            ["m30.inc"] = { description = "Mater Type 30" , file="m30.inc" },
            ["m31.inc"] = { description = "Mater Type 31" , file="m31.inc" },
            ["m35.inc"] = { description = "Mater Type 35 (CURVE Definition)" , file="m35.inc" },
            ["m36.inc"] = { description = "Mater Type 36" , file="m36.inc" },
            ["m37.inc"] = { description = "Mater Type 37" , file="m37.inc" },
            ["m38.inc"] = { description = "Mater Type 38" , file="m38.inc" },
            ["m41.inc"] = { description = "Mater Type 41" , file="m41.inc" },
            ["m42.inc"] = { description = "Mater Type 42" , file="m42.inc" },
            ["m45.inc"] = { description = "Mater Type 45" , file="m45.inc" },
            ["m47.inc"] = { description = "Mater Type 47" , file="m47.inc" },
            ["m51.inc"] = { description = "Mater Type 51" , file="m51.inc" },
            ["m52.inc"] = { description = "Mater Type 52 (CURVE Definition)" , file="m52.inc" },
            ["m61.inc"] = { description = "Mater Type 61" , file="m61.inc" },
            ["m62.inc"] = { description = "Mater Type 62 (CURVE Definition)" , file="m62.inc" },
            ["m71.inc"] = { description = "Mater Type 71 (CURVE Definition)" , file="m71.inc" },
            ["m99.inc"] = { description = "Mater Type 99" , file="m99.inc" },
            ["ma1.inc"] = { description = "VA Mater Type 1" , file="ma1.inc" },
            ["ma2.inc"] = { description = "VA Mater Type 2" , file="ma2.inc" },
            ["ma3.inc"] = { description = "VA Mater Type 3" , file="ma3.inc" },
            ["ma4.inc"] = { description = "VA Mater Type 4" , file="ma4.inc" },
            ["ma5.inc"] = { description = "VA Mater Type 5" , file="ma5.inc" },
          }
        },
        ["SHELL Materials"] = {
          description = "SHELL Materials",
          children = {
            ["m100.inc"] = { description = "Mater Type 100" , file="m100.inc" },
            ["m101.inc"] = { description = "Mater Type 101" , file="m101.inc" },
            ["m102.inc"] = { description = "Mater Type 102 (CURVE Definition)" , file="m102.inc" },
            ["m103.inc"] = { description = "Mater Type 103 (CURVE Definition)" , file="m103.inc" },
            ["m105.inc"] = { description = "Mater Type 105 (CURVE Definition, HSR Damage)" , file="m105.inc" },
            ["m106.inc"] = { description = "Mater Type 106 (CURVE Definition)" , file="m106.inc" },
            ["m108.inc"] = { description = "Mater Type 108 (CURVE Definition)" , file="m108.inc" },
            ["m109.inc"] = { description = "Mater Type 109 (CURVE Definition)" , file="m109.inc" },
            ["m110.inc"] = { description = "Mater Type 110" , file="m110.inc" },
            ["m115.inc"] = { description = "Mater Type 115 (CURVE Definition)" , file="m115.inc" },
            ["m116.inc"] = { description = "Mater Type 116 (CURVE Definition)" , file="m116.inc" },
            ["m117.inc"] = { description = "Mater Type 117 (CURVE Definition)" , file="m117.inc" },
            ["m118.inc"] = { description = "Mater Type 118 (CURVE Definition)" , file="m118.inc" },
            ["m121.inc"] = { description = "Mater Type 121" , file="m121.inc" },
            ["m126.inc"] = { description = "Mater Type 126" , file="m126.inc" },
            ["m127.inc"] = { description = "Mater Type 127 (CURVE Definition)" , file="m127.inc" },
            ["m128.inc"] = { description = "Mater Type 128" , file="m128.inc" },
            ["m130.inc"] = { description = "Mater Type 130" , file="m130.inc" },
            ["m131.inc"] = { description = "Mater Type 131" , file="m131.inc" },
            ["m132.inc"] = { description = "Mater Type 132" , file="m132.inc" },
            ["m143.inc"] = { description = "Mater Type 143" , file="m143.inc" },
            ["m150.inc"] = { description = "Mater Type 150" , file="m150.inc" },
            ["m151.inc"] = { description = "Mater Type 151" , file="m151.inc" },
            ["m152.inc"] = { description = "Mater Type 152" , file="m152.inc" },
            ["m161.inc"] = { description = "Mater Type 161" , file="m161.inc" },
            ["m162.inc"] = { description = "Mater Type 162 (CURVE Definition)" , file="m162.inc" },
            ["m171.inc"] = { description = "Mater Type 171 (CURVE Definition)" , file="m171.inc" },
          }
        },
        ["BEAM BAR Materials"] = {
          description = "BEAM BAR Materials",
          children = {
            ["m200.inc"] = { description = "Mater Type 200" , file="m200.inc" },
            ["m201.inc"] = { description = "Mater Type 201" , file="m201.inc" },
            ["m202.inc"] = { description = "Mater Type 202 (CURVE Definition)" , file="m202.inc" },
            ["m203.inc"] = { description = "Mater Type 203" , file="m203.inc" },
            ["m204.inc"] = { description = "Mater Type 204" , file="m204.inc" },
            ["m205.inc"] = { description = "Mater Type 205" , file="m205.inc" },
            ["m212.inc"] = { description = "Mater Type 212 (CURVE Definition)" , file="m212.inc" },
            ["m213.inc"] = { description = "Mater Type 213 (CURVE Definition)" , file="m213.inc" },
            ["m214.inc"] = { description = "Mater Type 214" , file="m214.inc" },
            ["m220.inc"] = { description = "Mater Type 220" , file="m220.inc" },
            ["m221.inc"] = { description = "Mater Type 221" , file="m221.inc" },
            ["m222.inc"] = { description = "Mater Type 222" , file="m222.inc" },
            ["m223.inc"] = { description = "Mater Type 223" , file="m223.inc" },
            ["m224.inc"] = { description = "Mater Type 224" , file="m224.inc" },
            ["m225.inc"] = { description = "Mater Type 225" , file="m225.inc" },
            ["m226.inc"] = { description = "Mater Type 226" , file="m226.inc" },
            ["m230.inc"] = { description = "Mater Type 230" , file="m230.inc" },
          }
        },
        ["LINK Materials"] = {
          description = "LINK Materials",
          children = {
            ["m301.inc"] = { description = "Mater Type 301" , file="m301.inc" },
            ["m302.inc"] = { description = "Mater Type 302" , file="m302.inc" },
            ["m303.inc"] = { description = "Mater Type 303" , file="m303.inc" },
            ["m304.inc"] = { description = "Mater Type 304" , file="m304.inc" },
            ["m305.inc"] = { description = "Mater Type 305" , file="m305.inc" },
            ["m371.inc"] = { description = "Mater Type 371" , file="m371.inc" },
          }
        },
        ["PLY Data"] = {
          description = "PLY Data",
          children = {
            ["ply0.inc"] = { description = "PLY Material Card" , file="ply0.inc" },
            ["ply1.inc"] = { description = "PLY Type 1" , file="ply1.inc" },
            ["ply2.inc"] = { description = "PLY Type 2" , file="ply2.inc" },
            ["ply3.inc"] = { description = "PLY Type 3" , file="ply3.inc" },
            ["ply4.inc"] = { description = "PLY Type 4" , file="ply4.inc" },
            ["ply5.inc"] = { description = "PLY Type 5" , file="ply5.inc" },
            ["ply6.inc"] = { description = "PLY Type 6" , file="ply6.inc" },
            ["ply7.inc"] = { description = "PLY Type 7" , file="ply7.inc" },
            ["ply8.inc"] = { description = "PLY Type 8" , file="ply8.inc" },
          }
        },
        ["Misc"] = {
          description = "Misc",
          children = {
            ["mp.inc"] = { description = "PFMAT Porous Material" , file="mp.inc" },
            ["mt.inc"] = { description = "THMAT Thermal Material" , file="mt.inc" },
            ["la.inc"] = { description = "LAYER Material Card" , file="la.inc" },
          }
        },
      }
    },
    Node = {
      description = "Node",
      children = {
        ["cn.inc"] = { description = "CNODE Card" , file="cn.inc" },
        ["ms.inc"] = { description = "MASS Card" , file="ms.inc" },
        ["nm.inc"] = { description = "NSMAS - Nonstructural mass" , file="nm.inc" },
        ["nm2.inc"] = { description = "NSMAS2 - Nonstructural mass Type 2" , file="nm2.inc" },
        ["nn.inc"] = { description = "NODE Card" , file="nn.inc" },
      }
    },
    Others = {
      description = "Others",
      children = {
        ["fs.inc"] = { description = "FUNCT Card with Sinus Function" , file="fs.inc" },
        ["gv.inc"] = { description = "Gratitational Acceleration" , file="gv.inc" },
      }
    },
    Output = {
      description = "Output",
      children = {
        ["seg.inc"] = { description = "SENPTG Sensor Point (Global)" , file="seg.inc" },
        ["sel.inc"] = { description = "SELOUT Selective Output" , file="sel.inc" },
        ["sen.inc"] = { description = "SENPT Sensor Point (Local)" , file="sen.inc" },
        ["sfc.inc"] = { description = "SEFCO Type CONTACT" , file="sfc.inc" },
        ["sfl.inc"] = { description = "SECFO Type LINK" , file="sfl.inc" },
        ["sfm.inc"] = { description = "SECFO Type CONT_MS" , file="sfm.inc" },
        ["sfp.inc"] = { description = "SECFO Type PLANE" , file="sfp.inc" },
        ["sfs.inc"] = { description = "SECFO Type SECTION" , file="sfs.inc" },
        ["sfu.inc"] = { description = "SECTION Type SUPPORT" , file="sfu.inc" },
        ["sfv.inc"] = { description = "SECFO Type VOLFRAC" , file="sfv.inc" },
        ["tha.inc"] = { description = "THNAC Acoustic Output" , file="tha.inc" },
        ["thc.inc"] = { description = "THCRS Cross Spectral Density Printout" , file="thc.inc" },
        ["the.inc"] = { description = "THELE Card" , file="the.inc" },
        ["thl.inc"] = { description = "THLOC Output" , file="thl.inc" },
        ["thn.inc"] = { description = "THNOD Output" , file="thn.inc" },
        ["thp.inc"] = { description = "THNPO Porous Nodal Output" , file="thp.inc" },
        ["vap.inc"] = { description = "VAPANL Structural Panel" , file="vap.inc" },
      }
    },
    Part = {
      description = "Part",
      children = {
        ["pba.inc"] = { description = "PART Type BAR" , file="pba.inc" },
        ["pbe.inc"] = { description = "PART Type BEAM" , file="pbe.inc" },
        ["pbs.inc"] = { description = "PART Type BSHEL" , file="pbs.inc" },
        ["pco.inc"] = { description = "PART Type COS3D" , file="pco.inc" },
        ["pga.inc"] = { description = "PART Type GAP" , file="pga.inc" },
        ["pjo.inc"] = { description = "PART Type JOINT" , file="pjo.inc" },
        ["pkj.inc"] = { description = "PART Type KJOIN" , file="pkj.inc" },
        ["ple.inc"] = { description = "PART Type ELINK" , file="ple.inc" },
        ["pll.inc"] = { description = "PART Type LLINK" , file="pll.inc" },
        ["plp.inc"] = { description = "PART Type PLINK" , file="plp.inc" },
        ["pls.inc"] = { description = "PART Type SLINK" , file="pls.inc" },
        ["pme.inc"] = { description = "PART Type MEMBR" , file="pme.inc" },
        ["pmk.inc"] = { description = "PART Type MBKJN" , file="pmk.inc" },
        ["pms.inc"] = { description = "PART Type MBSPR" , file="pms.inc" },
        ["pmt.inc"] = { description = "PART Type MTOJNT" , file="pmt.inc" },
        ["poo.inc"] = { description = "MPART Type SOLID" , file="poo.inc" },
        ["pos.inc"] = { description = "MPART Type SHELL" , file="pos.inc" },
        ["psb.inc"] = { description = "PART Type SPRGBM" , file="psb.inc" },
        ["pse.inc"] = { description = "PART Type SPHEL" , file="pse.inc" },
        ["psh.inc"] = { description = "PART Type SHELL" , file="psh.inc" },
        ["pso.inc"] = { description = "PART Type SOLID" , file="pso.inc" },
        ["psp.inc"] = { description = "PART Type SPRING" , file="psp.inc" },
        ["pte.inc"] = { description = "PART Type TETRA" , file="pte.inc" },
        ["pti.inc"] = { description = "PART Type TIED" , file="pti.inc" },
        ["pts.inc"] = { description = "PART Type TSHEL" , file="pts.inc" },
      }
    },
    Safety = {
      description = "Safety",
      children = {
        ["bag.inc"] = { description = "BAGIN Definition" , file="bag.inc" },
        ["gas.inc"] = { description = "GASPEC Gas Specification" , file="gas.inc" },
        ["mus.inc"] = { description = "MUSC1 Muscle Element Definition" , file="mus.inc" },
        ["ret.inc"] = { description = "RETRA Belt Retractor Definition" , file="ret.inc" },
        ["sli.inc"] = { description = "SLIPR Slipring Definition" , file="sli.inc" },
      }
    },
  }

  impromptu.ask{
    options = opts,
    compact_columns = true,
    lines_to_grid = lines_to_grid,
    sort = sort,
    handler = function(b, opt)
      file = cardpath.."/"..b.breadcrumbs[1].."/"..opt.file
      set_lines(curbuf, curpos[1], curpos[1], false, lines_from_file(file))
      return true
    end
  }     
end

return {
  cardmenu = cardmenu
}
