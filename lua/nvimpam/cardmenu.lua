local curbuf = vim.api.nvim_get_current_buf
local curpos = vim.api.nvim_win_get_cursor
local curwin = vim.api.nvim_get_current_win
local set_lines = vim.api.nvim_buf_set_lines
local command = vim.api.nvim_command

local lines_from_file = require('nvimpam.utils').lines_from_file
local cardpath
local rtp

local function cardmenu()
  local status, impromptu = pcall(require, "impromptu")

  if not status then
    command("echoerr 'Impromptu not installed, can not show menu!'")
    return nil
  end

  if not cardpath then
    local rtp = vim.api.nvim_eval("&rtp")

    for str in string.gmatch(rtp, "([^,]+)") do
      if not cardpath or cardpath == "" then
        cardpath = vim.api.nvim_call_function('finddir', {'pam_cards', str.."/**" })
      end
    end
  end

  local curbuf = curbuf()
  local curwin = curwin()
  local curpos = curpos(curwin)

  local opts = {
    Auxiliaries = {
      description = "Auxiliaries",
      children = {
        ["cd.inc"] = { description = "CDATA Card" },
        ["de.inc"] = { description = "DELEM - Deleted Element Card" },
        ["fc.inc"] = { description = "FUNCT Function Card" },
        ["fm0.inc"] = { description = "FRAME IAXIS=0 U-based, 2 Vectors" },
        ["fm1.inc"] = { description = "FRAME IAXIS=1 U-based, 3 Nodes" },
        ["fm2.inc"] = { description = "FRAME IAXIS=2 T-based, 2 Vectors" },
        ["fm3.inc"] = { description = "FRAME IAXIS=3 T-based, 3 Nodes" },
        ["fm4.inc"] = { description = "FRAME IAXIS=4 Cylindrical" },
        ["fm5.inc"] = { description = "FRAME IAXIS=5 Spherical" },
        ["fr1.inc"] = { description = "FRICT Friction Model Type 1" },
        ["fr10.inc"] = { description = "FRICT Friction Model Type 10" },
        ["fr11.inc"] = { description = "FRICT Friction Model Type 11" },
        ["fr12.inc"] = { description = "FRICT Friction Model Type 12" },
        ["fr13.inc"] = { description = "FRICT Friction Model Type 13" },
        ["fr2.inc"] = { description = "FRICT Friction Model Type 2" },
        ["fr3.inc"] = { description = "FRICT Friction Model Type 3" },
        ["fr4.inc"] = { description = "FRICT Friction Model Type 4" },
        ["fr5.inc"] = { description = "FRICT Friction Model Type 5" },
        ["fr6.inc"] = { description = "FRICT Friction Model Type 5" },
        ["fw.inc"] = { description = "FUNCSW Function Switch" },
        ["gr.inc"] = { description = "GROUP Group Definition" },
        ["lo.inc"] = { description = "LOOKU Lookup Table" },
        ["nl.inc"] = { description = "NLAVE Non Local Averadge Definition" },
        ["pa0.inc"] = { description = "PLANE Type 0" },
        ["pa1.inc"] = { description = "PLANE Type 1" },
        ["pa2.inc"] = { description = "PLANE Type 2" },
        ["pf.inc"] = { description = "PYFUNC Python Function" },
        ["ru0.inc"] = { description = "RUPMO Type 0" },
        ["ru1.inc"] = { description = "RUPMO Type 1" },
        ["ru2.inc"] = { description = "RUPMO Type 2" },
        ["ru3.inc"] = { description = "RUPMO Type 3" },
        ["ru5.inc"] = { description = "RUPMO Type 5" },
        ["ru6.inc"] = { description = "RUPMO Type 6" },
        ["ru7.inc"] = { description = "RUPMO Type 7" },
        ["se1.inc"] = { description = "SENSOR Type 1" },
        ["se10.inc"] = { description = "SENSOR Type 10" },
        ["se11.inc"] = { description = "SENSOR Type 11" },
        ["se12.inc"] = { description = "SENSOR Type 12" },
        ["se13.inc"] = { description = "SENSOR Type 13" },
        ["se14.inc"] = { description = "SENSOR Type 14" },
        ["se2.inc"] = { description = "SENSOR Type 2" },
        ["se3.inc"] = { description = "SENSOR Type 3" },
        ["se4.inc"] = { description = "SENSOR Type 4" },
        ["se5.inc"] = { description = "SENSOR Type 5" },
        ["se6.inc"] = { description = "SENSOR Type 6" },
        ["se7.inc"] = { description = "SENSOR Type 7" },
        ["se8.inc"] = { description = "SENSOR Type 8" },
        ["se9.inc"] = { description = "SENSOR Type 9" },
        ["sr.inc"] = { description = "SURFA Surface Definition" },
        ["ud.inc"] = { description = "UDATA User Data" },
        ["ve0.inc"] = { description = "VECTOR Type 0" },
        ["ve1.inc"] = { description = "VECTOR Type 1" },
      }
    },
    Constraint = {
      description = "Constraint",
      children = {
        ["lc.inc"] = { description = "LINCO Linear Constraint" },
        ["oo.inc"] = { description = "OTMCO Constraint" },
        ["rb0.inc"] = { description = "RBODY Type 0" },
        ["rb1.inc"] = { description = "RBODY Type 1" },
        ["rb2.inc"] = { description = "RBODY Type 2" },
        ["rb3.inc"] = { description = "RBODY Type 3" },
        ["rb4.inc"] = { description = "RBODY Type 4" },
        ["sew.inc"] = { description = "SEWING Definition" },
        ["to0.inc"] = { description = "MTOCO (Regular)" },
        ["to1.inc"] = { description = "MTOCO (User Imposed Mass and Intertia)" },
      }
    },
    Contact = {
      description = "Contact",
      children = {
        ["c1.inc"] = { description = "Contact Type 1" },
        ["c10.inc"] = { description = "Contact Type 10" },
        ["c154.inc"] = { description = "Contact Type 154" },
        ["c21.inc"] = { description = "Contact Type 21" },
        ["c33.inc"] = { description = "Contact Type 33" },
        ["c34.inc"] = { description = "Contact Type 34" },
        ["c36.inc"] = { description = "Contact Type 36" },
        ["c37.inc"] = { description = "Contact Type 37" },
        ["c43.inc"] = { description = "Contact Type 43" },
        ["c44.inc"] = { description = "Contact Type 44" },
        ["c46.inc"] = { description = "Contact Type 46" },
        ["c54.inc"] = { description = "Contact Type 54" },
        ["c61.inc"] = { description = "Contact Type 61" },
      }
    },
    Control = {
      description = "Control",
      children = {
        ["dm.inc"] = { description = "DMPEW User DMP Scaling Factors" },
        ["dr.inc"] = { description = "DRAPF Draping File Import" },
        ["ex.inc"] = { description = "EXPORT Card" },
        ["im.inc"] = { description = "IMPORT Card" },
        ["in.inc"] = { description = "INCLU Keyword" },
        ["me.inc"] = { description = "MERIC Keyword" },
        ["mr.inc"] = { description = "MSTRM Mass Trimming" },
        ["or.inc"] = { description = "ORTHF Orientation File Import" },
        ["py.inc"] = { description = "PYVAR Variable Definition" },
        ["ss.inc"] = { description = "RMSSOL Shell-Solid Remeshing" },
        ["su.inc"] = { description = "SUBDF Substructure Definition" },
        ["tr.inc"] = { description = "TRSFM Transformation Card" },
      }
    },
    Element = {
      description = "Element",
      children = {
        ["eba.inc"] = { description = "BAR Element" },
        ["ebe.inc"] = { description = "BEAM Element" },
        ["ebs.inc"] = { description = "BSHEL Element" },
        ["ega.inc"] = { description = "GAP Element" },
        ["ehe.inc"] = { description = "HEXA20 Element" },
        ["ejo.inc"] = { description = "JOINT Element" },
        ["ekj.inc"] = { description = "KJOIN Element" },
        ["ele.inc"] = { description = "ELINK Element" },
        ["ell.inc"] = { description = "LLINK Element" },
        ["elp.inc"] = { description = "PLINK Element" },
        ["els.inc"] = { description = "SLINK Element" },
        ["eme.inc"] = { description = "MEMBR Element" },
        ["emt.inc"] = { description = "MTOJNT Element" },
        ["ep1.inc"] = { description = "PENTA15 Element" },
        ["ep6.inc"] = { description = "PENTA6 Element " },
        ["eph.inc"] = { description = "SPHEL Element" },
        ["epo.inc"] = { description = "SPHELO Element" },
        ["es6.inc"] = { description = "SHEL6 Element" },
        ["es8.inc"] = { description = "SHEL8 Element" },
        ["esb.inc"] = { description = "SPRGBM Element" },
        ["esh.inc"] = { description = "SHELL Element" },
        ["eso.inc"] = { description = "SOLID Element" },
        ["esp.inc"] = { description = "SPRING Element" },
        ["esu.inc"] = { description = "IMPMA Super Element Matrix Import" },
        ["et1.inc"] = { description = "TETR10 Element" },
        ["et4.inc"] = { description = "TETR4 Element" },
        ["ete.inc"] = { description = "TETRA Element" },
        ["eti.inc"] = { description = "TIED Element" },
        ["ets.inc"] = { description = "TSHEL Element" },
      }
    },
    Load = {
      description = "Load",
      children = {
        ["3d.inc"] = { description = "3D Boundary Condition" },
        ["ab.inc"] = { description = "VAABSO Acoustic Absorber" },
        ["ac.inc"] = { description = "ACFLD Acceleration Field" },
        ["am.inc"] = { description = "VAMPSO Acoustic Monopole Source" },
        ["ap.inc"] = { description = "Acoustic Plane Wave" },
        ["at.inc"] = { description = "ACTUA - Joint Actuator Definition" },
        ["bc.inc"] = { description = "BOUNC Displacement BC" },
        ["bd.inc"] = { description = "BDFOR Body Forces" },
        ["bf.inc"] = { description = "BFLUX Definition" },
        ["bp.inc"] = { description = "PRESBC Pressure Porous BC" },
        ["cl.inc"] = { description = "CONLO Concentrated Load" },
        ["da.inc"] = { description = "DAMP Nodal Damping" },
        ["df.inc"] = { description = "DFLUX Definition" },
        ["dl.inc"] = { description = "DISLIM Displacement Limitation" },
        ["dp.inc"] = { description = "DETOP Detonation Point" },
        ["fb.inc"] = { description = "FBC3D Prescribed Motion onto Fluid Media" },
        ["fn.inc"] = { description = "FBCFA Prescribed Surface Normal BC onto Fluid" },
        ["hf.inc"] = { description = "HFLUX" },
        ["hs.inc"] = { description = "HTSURF Heat Exchange Surface" },
        ["ib.inc"] = { description = "PREBM Beam Pressure" },
        ["if.inc"] = { description = "PREFA Face Pressure" },
        ["ip.inc"] = { description = "INPRES Initial Pressure" },
        ["it.inc"] = { description = "INTEM Initial Temperature" },
        ["iv.inc"] = { description = "INVEL Initial Velocity" },
        ["kd.inc"] = { description = "KINDA Kinematic Damping" },
        ["pu.inc"] = { description = "PFSURF Porous Flow Exchange Surface" },
        ["rm.inc"] = { description = "RMLOAD Resudial Mode Load" },
        ["sd.inc"] = { description = "LCPSD Power Spectral Density Function" },
        ["tc.inc"] = { description = "TEMBC Temperature BC" },
        ["tu.inc"] = { description = "TURBL Turbulent Boundary Layer Load" },
      }
    },
    MMC = {
      description = "MMC",
      children = {
        ["mm.inc"] = { description = "MMC Assign Definition" },
      }
    },
    Material = {
      description = "Material",
      children = {
        ["SOLID Materials"] = {
          description = "SOLID Materials",
          children = {
            ["m1.inc"] = { description = "Mater Type 1 (CURVE Definition)" },
            ["m2.inc"] = { description = "Mater Type 2" },
            ["m5.inc"] = { description = "Mater Type 5" },
            ["m6.inc"] = { description = "Mater Type 6" },
            ["m7.inc"] = { description = "Mater Type 7" },
            ["m8.inc"] = { description = "Mater Type 8" },
            ["m11.inc"] = { description = "Mater Type 11" },
            ["m12.inc"] = { description = "Mater Type 12" },
            ["m15.inc"] = { description = "Mater Type 15" },
            ["m16.inc"] = { description = "Mater Type 16 (CURVE Definition)" },
            ["m17.inc"] = { description = "Mater Type 17" },
            ["m18.inc"] = { description = "Mater Type 18" },
            ["m19.inc"] = { description = "Mater Type 19" },
            ["m20.inc"] = { description = "Mater Type 20" },
            ["m21.inc"] = { description = "Mater Type 21" },
            ["m22.inc"] = { description = "Mater Type 22" },
            ["m24.inc"] = { description = "Mater Type 24" },
            ["m25.inc"] = { description = "Mater Type 25" },
            ["m26.inc"] = { description = "Mater Type 26 (CURVE Definition)" },
            ["m28.inc"] = { description = "Mater Type 28" },
            ["m30.inc"] = { description = "Mater Type 30" },
            ["m31.inc"] = { description = "Mater Type 31" },
            ["m35.inc"] = { description = "Mater Type 35 (CURVE Definition)" },
            ["m36.inc"] = { description = "Mater Type 36" },
            ["m37.inc"] = { description = "Mater Type 37" },
            ["m38.inc"] = { description = "Mater Type 38" },
            ["m41.inc"] = { description = "Mater Type 41" },
            ["m42.inc"] = { description = "Mater Type 42" },
            ["m45.inc"] = { description = "Mater Type 45" },
            ["m47.inc"] = { description = "Mater Type 47" },
            ["m51.inc"] = { description = "Mater Type 51" },
            ["m52.inc"] = { description = "Mater Type 52 (CURVE Definition)" },
            ["m61.inc"] = { description = "Mater Type 61" },
            ["m62.inc"] = { description = "Mater Type 62 (CURVE Definition)" },
            ["m71.inc"] = { description = "Mater Type 71 (CURVE Definition)" },
            ["m99.inc"] = { description = "Mater Type 99" },
            ["ma1.inc"] = { description = "VA Mater Type 1" },
            ["ma2.inc"] = { description = "VA Mater Type 2" },
            ["ma3.inc"] = { description = "VA Mater Type 3" },
            ["ma4.inc"] = { description = "VA Mater Type 4" },
            ["ma5.inc"] = { description = "VA Mater Type 5" },
          }
        },
        ["SHELL Materials"] = {
          description = "SHELL Materials",
          children = {
            ["m100.inc"] = { description = "Mater Type 100" },
            ["m101.inc"] = { description = "Mater Type 101" },
            ["m102.inc"] = { description = "Mater Type 102 (CURVE Definition)" },
            ["m103.inc"] = { description = "Mater Type 103 (CURVE Definition)" },
            ["m105.inc"] = { description = "Mater Type 105 (CURVE Definition, HSR Damage)" },
            ["m106.inc"] = { description = "Mater Type 106 (CURVE Definition)" },
            ["m108.inc"] = { description = "Mater Type 108 (CURVE Definition)" },
            ["m109.inc"] = { description = "Mater Type 109 (CURVE Definition)" },
            ["m110.inc"] = { description = "Mater Type 110" },
            ["m115.inc"] = { description = "Mater Type 115 (CURVE Definition)" },
            ["m116.inc"] = { description = "Mater Type 116 (CURVE Definition)" },
            ["m117.inc"] = { description = "Mater Type 117 (CURVE Definition)" },
            ["m118.inc"] = { description = "Mater Type 118 (CURVE Definition)" },
            ["m121.inc"] = { description = "Mater Type 121" },
            ["m126.inc"] = { description = "Mater Type 126" },
            ["m127.inc"] = { description = "Mater Type 127 (CURVE Definition)" },
            ["m128.inc"] = { description = "Mater Type 128" },
            ["m130.inc"] = { description = "Mater Type 130" },
            ["m131.inc"] = { description = "Mater Type 131" },
            ["m132.inc"] = { description = "Mater Type 132" },
            ["m143.inc"] = { description = "Mater Type 143" },
            ["m150.inc"] = { description = "Mater Type 150" },
            ["m151.inc"] = { description = "Mater Type 151" },
            ["m152.inc"] = { description = "Mater Type 152" },
            ["m161.inc"] = { description = "Mater Type 161" },
            ["m162.inc"] = { description = "Mater Type 162 (CURVE Definition)" },
            ["m171.inc"] = { description = "Mater Type 171 (CURVE Definition)" },
          }
        },
        ["BEAM BAR Materials"] = {
          description = "BEAM BAR Materials",
          children = {
            ["m200.inc"] = { description = "Mater Type 200" },
            ["m201.inc"] = { description = "Mater Type 201" },
            ["m202.inc"] = { description = "Mater Type 202 (CURVE Definition)" },
            ["m203.inc"] = { description = "Mater Type 203" },
            ["m204.inc"] = { description = "Mater Type 204" },
            ["m205.inc"] = { description = "Mater Type 205" },
            ["m212.inc"] = { description = "Mater Type 212 (CURVE Definition)" },
            ["m213.inc"] = { description = "Mater Type 213 (CURVE Definition)" },
            ["m214.inc"] = { description = "Mater Type 214" },
            ["m220.inc"] = { description = "Mater Type 220" },
            ["m221.inc"] = { description = "Mater Type 221" },
            ["m222.inc"] = { description = "Mater Type 222" },
            ["m223.inc"] = { description = "Mater Type 223" },
            ["m224.inc"] = { description = "Mater Type 224" },
            ["m225.inc"] = { description = "Mater Type 225" },
            ["m226.inc"] = { description = "Mater Type 226" },
            ["m230.inc"] = { description = "Mater Type 230" },
          }
        },
        ["LINK Materials"] = {
          description = "LINK Materials",
          children = {
            ["m301.inc"] = { description = "Mater Type 301" },
            ["m302.inc"] = { description = "Mater Type 302" },
            ["m303.inc"] = { description = "Mater Type 303" },
            ["m304.inc"] = { description = "Mater Type 304" },
            ["m305.inc"] = { description = "Mater Type 305" },
            ["m371.inc"] = { description = "Mater Type 371" },
          }
        },
        ["PLY Data"] = {
          description = "PLY Data",
          children = {
            ["ply0.inc"] = { description = "PLY Material Card" },
            ["ply1.inc"] = { description = "PLY Type 1" },
            ["ply2.inc"] = { description = "PLY Type 2" },
            ["ply3.inc"] = { description = "PLY Type 3" },
            ["ply4.inc"] = { description = "PLY Type 4" },
            ["ply5.inc"] = { description = "PLY Type 5" },
            ["ply6.inc"] = { description = "PLY Type 6" },
            ["ply7.inc"] = { description = "PLY Type 7" },
            ["ply8.inc"] = { description = "PLY Type 8" },
          }
        },
        ["Misc"] = {
          description = "Misc",
          children = {
            ["mp.inc"] = { description = "PFMAT Porous Material" },
            ["mt.inc"] = { description = "THMAT Thermal Material" },
            ["la.inc"] = { description = "LAYER Material Card" },
          }
        },
      }
    },
    Node = {
      description = "Node",
      children = {
        ["cn.inc"] = { description = "CNODE Card" },
        ["ms.inc"] = { description = "MASS Card" },
        ["nm.inc"] = { description = "NSMAS - Nonstructural mass" },
        ["nm2.inc"] = { description = "NSMAS2 - Nonstructural mass Type 2" },
        ["nn.inc"] = { description = "NODE Card" },
      }
    },
    Others = {
      description = "Others",
      children = {
        ["fs.inc"] = { description = "FUNCT Card with Sinus Function" },
        ["gv.inc"] = { description = "Gratitational Acceleration" },
      }
    },
    Output = {
      description = "Output",
      children = {
        ["seg.inc"] = { description = "SENPTG Sensor Point (Global)" },
        ["sel.inc"] = { description = "SELOUT Selective Output" },
        ["sen.inc"] = { description = "SENPT Sensor Point (Local)" },
        ["sfc.inc"] = { description = "SEFCO Type CONTACT" },
        ["sfl.inc"] = { description = "SECFO Type LINK" },
        ["sfm.inc"] = { description = "SECFO Type CONT_MS" },
        ["sfp.inc"] = { description = "SECFO Type PLANE" },
        ["sfs.inc"] = { description = "SECFO Type SECTION" },
        ["sfu.inc"] = { description = "SECTION Type SUPPORT" },
        ["sfv.inc"] = { description = "SECFO Type VOLFRAC" },
        ["tha.inc"] = { description = "THNAC Acoustic Output" },
        ["thc.inc"] = { description = "THCRS Cross Spectral Density Printout" },
        ["the.inc"] = { description = "THELE Card" },
        ["thl.inc"] = { description = "THLOC Output" },
        ["thn.inc"] = { description = "THNOD Output" },
        ["thp.inc"] = { description = "THNPO Porous Nodal Output" },
        ["vap.inc"] = { description = "VAPANL Structural Panel" },
      }
    },
    Part = {
      description = "Part",
      children = {
        ["pba.inc"] = { description = "PART Type BAR" },
        ["pbe.inc"] = { description = "PART Type BEAM" },
        ["pbs.inc"] = { description = "PART Type BSHEL" },
        ["pco.inc"] = { description = "PART Type COS3D" },
        ["pga.inc"] = { description = "PART Type GAP" },
        ["pjo.inc"] = { description = "PART Type JOINT" },
        ["pkj.inc"] = { description = "PART Type KJOIN" },
        ["ple.inc"] = { description = "PART Type ELINK" },
        ["pll.inc"] = { description = "PART Type LLINK" },
        ["plp.inc"] = { description = "PART Type PLINK" },
        ["pls.inc"] = { description = "PART Type SLINK" },
        ["pme.inc"] = { description = "PART Type MEMBR" },
        ["pmk.inc"] = { description = "PART Type MBKJN" },
        ["pms.inc"] = { description = "PART Type MBSPR" },
        ["pmt.inc"] = { description = "PART Type MTOJNT" },
        ["poo.inc"] = { description = "MPART Type SOLID" },
        ["pos.inc"] = { description = "MPART Type SHELL" },
        ["psb.inc"] = { description = "PART Type SPRGBM" },
        ["pse.inc"] = { description = "PART Type SPHEL" },
        ["psh.inc"] = { description = "PART Type SHELL" },
        ["pso.inc"] = { description = "PART Type SOLID" },
        ["psp.inc"] = { description = "PART Type SPRING" },
        ["pte.inc"] = { description = "PART Type TETRA" },
        ["pti.inc"] = { description = "PART Type TIED" },
        ["pts.inc"] = { description = "PART Type TSHEL" },
      }
    },
    Safety = {
      description = "Safety",
      children = {
        ["bag.inc"] = { description = "BAGIN Definition" },
        ["gas.inc"] = { description = "GASPEC Gas Specification" },
        ["mus.inc"] = { description = "MUSC1 Muscle Element Definition" },
        ["ret.inc"] = { description = "RETRA Belt Retractor Definition" },
        ["sli.inc"] = { description = "SLIPR Slipring Definition" },
      }
    },
  }

  impromptu.core.ask{
    options = opts,
    handler = function(b, opt)
      file = cardpath.."/"..b.breadcrumbs[1].."/"..opt
      set_lines(curbuf, curpos[1], curpos[1], false, lines_from_file(file))
      return true
    end
  }     
end

return {
  cardmenu = cardmenu
}
