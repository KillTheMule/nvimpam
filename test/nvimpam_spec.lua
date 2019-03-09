local helpers = require('test.functional.helpers')(after_each)
local Screen = require('test.functional.ui.screen')
local clear, command = helpers.clear, helpers.command
local feed, alter_slashes = helpers.feed, helpers.alter_slashes
local insert = helpers.insert
local meths = helpers.meths
local eq = helpers.eq
local dedent = helpers.dedent
local eval = helpers.eval

local is_ci = os.getenv("TRAVIS") or os.getenv("APPVEYOR")

local function sleep(time)
  if is_ci then
    helpers.sleep(20 * time)
  else
    helpers.sleep(time)
  end
end

describe('nvimpam', function()
  local screen

  before_each(function()
    clear()
    screen = Screen.new(81, 15)
    screen:attach()
    screen:set_default_attr_ids({
      [1] = {foreground = Screen.colors.DarkBlue, background = Screen.colors.LightGray},
      [2] = {bold = true, foreground = Screen.colors.Blue1},
      [3] = {reverse = true,},
      [4] = {bold = true, reverse = true},
      [5] = {background = Screen.colors.LightGrey, underline = true},
      [6] = {bold = true},
      [7] = {foreground = Screen.colors.Grey3, background = 6291200},
      [8] = {bold = true, foreground = 8871680},
      [9] = {background = 16777167},
      [10] = {background = 15000804},
      [11] = {foreground = Screen.colors.Grey100, background = Screen.colors.Red},
      [12] = {foreground = Screen.colors.Grey100, background = 11468800},
      [13] = {foreground = Screen.colors.Red},
      [14] = {foreground = Screen.colors.Grey0, background = Screen.colors.Yellow},
    })
    command('set rtp+=../')
    command('source ../init.vim')
  end)

  after_each(function()
    screen:detach()
  end)

  local input = [[
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.
      #Comment here
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      #Comment
      #Comment
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      $Comment
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      $Comment
      #Comment
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      SHELL /     3129       1       1    2967    2971    2970
      ]]

  it('basically works', function()
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    command('NvimPamUpdateFolds')
    feed("28G")

    screen:expect([[
       ERFOUTPUT        3        0                                                     |
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      ^$#         IDNOD               X               Y               Z                 |
      {1: 725 lines: Node ································································}|
      $----------------------------------------------------------------                |
      $     MATERIAL DEFINITIONS                                                       |
      $----------------------------------------------------------------                |
      $ boxbeam                                                                        |
      $#         IDMAT   MATYP             RHO   ISINT    ISHG  ISTRAT   IFROZ         |
      MATER /        3     103         7.85E-6       0       0       0       0         |
      {IGNORE}|
    ]])

    feed("809G")
    screen:expect([[
                                                                                       |
                                                                                       |
      $----------------------------------------------------------------                |
      $     PART AND ELEMENT DEFINITIONS                                               |
      $----------------------------------------------------------------                |
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {1:^ 2 PartShells ···································································}|
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {1: 11 lines: PartPlink ····························································}|
      $#          IDEL   IDPRT   IDNOD    MORE   NLAYR                                 |
      {1: 15 lines: Plink ································································}|
      $                                                                                |
      $----------------------------------------------------------------                |
      $     RIGID BODIES                                                               |
      {IGNORE}|
    ]])

    feed("zo")
    screen:expect([[
                                                                                       |
                                                                                       |
      $----------------------------------------------------------------                |
      $     PART AND ELEMENT DEFINITIONS                                               |
      $----------------------------------------------------------------                |
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {1:^ 13 lines: PartShell ····························································}|
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {1: 13 lines: PartShell ····························································}|
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {1: 11 lines: PartPlink ····························································}|
      $#          IDEL   IDPRT   IDNOD    MORE   NLAYR                                 |
      {1: 15 lines: Plink ································································}|
      $                                                                                |
      {IGNORE}|
    ]])

    feed("zE")
    command("NvimPamHighlightScreen")
    screen:expect([[
                                                                                       |
                                                                                       |
      $----------------------------------------------------------------                |
      $     PART AND ELEMENT DEFINITIONS                                               |
      $----------------------------------------------------------------                |
      $#         IDPRT   ATYPE   IDMAT IDVAMAT IDTHMAT  IDPMAT                         |
      {8:^PART  / }{9:       1}{10:   SHELL}{9:       3}{10:       0}{9:       0}{10:       0}                         |
      $#                                                                         TITLE |
      {10:NAME}{9: Box section                                                                } |
      $#  DTELIM    TSCALF                                                             |
      {10:        0.}{9:          }                                                             |
      $#   TCONT    EPSINI  COULFRIC                                                   |
      {10:          }{9:          }{10:          }                                                   |
      $#       H NINT    OFFSETNINTh                                                   |
      {IGNORE}|
    ]])
  end)

  it('can deal with insertions', function()
    insert(input)
    feed("1G")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    sleep(10)
    command('NvimPamUpdateFolds')
    feed("1G")

    screen:expect([[
      {1:^ 4 lines: Node ··································································}|
      #Comment here                                                                    |
      {1: 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    command("echo") -- clear the command line
    feed("zo")
    feed("yyP")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:^ 5 lines: Node ··································································}|
      #Comment here                                                                    |
      {1: 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    feed("zE")
    command("NvimPamHighlightScreen")
    screen:expect([[
      {8:^NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      #Comment here                                                                    |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      #Comment                                                                         |
      #Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      $Comment                                                                         |
      {IGNORE}|
    ]])
  end)

  it('can deal with deletions', function()
    insert(input)
    command('set ft=pamcrash')
    command('NvimPamAttach')
    feed("1G")

    command("7,9d")
    sleep(10)
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1: 4 lines: Node ··································································}|
      #Comment here                                                                    |
      {1:^ 7 lines: Shell ·································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    feed("zE")
    command("NvimPamHighlightScreen")
    screen:expect([[
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      #Comment here                                                                    |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      ^#Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      $Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      $Comment                                                                         |
      #Comment                                                                         |
      {IGNORE}|
    ]])
  end)

  it('can deal with updates and undo', function()
    insert(input)
    command("set nohls")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    helpers.sleep(10)
    command(":7,9s/^SHELL/NODE ")
    feed("1G")
    sleep(10)
    command("NvimPamUpdateFolds")

    screen:expect([[
      {1:^ 4 lines: Node ··································································}|
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {1: 2 lines: Node ··································································}|
      #Comment                                                                         |
      #Comment                                                                         |
      {1: 5 lines: Shell ·································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    feed("u")
    command("NvimPamUpdateFolds")
    command("echo") -- clear the command line
    screen:expect([[
      {1: 4 lines: Node ··································································}|
      #Comment here                                                                    |
      {1:^ 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    -- trigger the subsitution again to get the error highlighting colors
    feed("<C-r>")
    feed("zR")
    command("NvimPamHighlightScreen")
    screen:expect([[
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      #Comment here                                                                    |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:^NODE  / }{9:    3129}{11:       1       1}{12:    2967    2971}{10:    2970}                         |
      {8:NODE  / }{9:    3129}{11:       1       1}{12:    2967    2971}{10:    2970}                         |
      #Comment                                                                         |
      #Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      $Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {IGNORE}|
    ]])
  end)

  it('starts a new instance for a new buffer', function()
    command("set nowrap")
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    sleep(10)
    command('NvimPamUpdateFolds')
    feed("28G")
    command("vs " .. alter_slashes("../files/example2.pc"))
    command("NvimPamAttach")
    sleep(10)
    command('NvimPamUpdateFolds')
    feed("28G")

    screen:expect([[
       ERFOUTPUT        3        0            {3:│} ERFOUTPUT        3        0            |
      NODPLOT    DFLT                         {3:│}NODPLOT    DFLT                         |
      SOLPLOT     ALL                         {3:│}SOLPLOT     ALL                         |
       SHLPLOT   DFLT                         {3:│} SHLPLOT   DFLT                         |
      END_OCTRL                               {3:│}END_OCTRL                               |
      $                                       {3:│}$                                       |
      ^$#         IDNOD               X        {3:│}$#         IDNOD               X        |
      $                                       {3:│}{1: 725 lines: Node ·······················}|
      $                                       {3:│}$---------------------------------------|
      $                                       {3:│}$     MATERIAL DEFINITIONS              |
      {1: 2 lines: Node ·························}{3:│}$---------------------------------------|
      {1: 2 lines: Shell ························}{3:│}$ boxbeam                               |
      {1: 721 lines: Node ·······················}{3:│}$#         IDMAT   MATYP             RHO|
      ]]
      ..
      alter_slashes(
      "{4:../files/example2.pc                     }{3:../files/example.pc                     }|"
      )
      ..
      "\n"
      ..
      [[
      {IGNORE}|
      ]]
      )

    --[[
    local clientinfo = {
      client = {
        [attributes] = {
          [author] = 'KillTheMule <KillTheMule@users.noreply.github.com',
          [documentation] = 'https://KillTheMule.github.io/nvimpam/nvimpam',
          [license] = 'Apache-2.0 OR MIT',
          [repository] = 'https://github.com/KillTheMule/nvimpam',
        },
        [methods] = {
          [NvimPamAttach] = {},
          [NvimPamDetach] = {},
          [NvimPamUpdateFolds] = {}
        },
        [name] = 'nvimpam',
        [type] = 'remote',
        [version] = {
          [major] = '0',
          [minor] = '1',
          [patch] = '2',
          [prerelease] = 'alpha.0',
        },
      },
      [id] = 3,
      [mode] = 'rpc',
      [stream] = 'job',
    }
    --]]

    local chans = meths.list_chans()
    local client1 = chans[3].client
    local client2 = chans[4].client
    eq(client1.name, 'nvimpam')
    eq(client2.name, 'nvimpam')

    command('call luaeval("require(\'nvimpam\').detach()")')
    sleep(100)
    chans = meths.list_chans()
    eq(chans[3].client.name, 'nvimpam')
    eq(nil, chans[4])

  end)

  it('quits on DetachEvent', function()
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    -- sleep needed to let the attaching happen
    -- needs to be this long for the debug binary
    sleep(100)

    feed("Ax<Esc>")
    
    local chans = meths.list_chans()
    local client = chans[3].client
    eq(client.name, 'nvimpam')
    eq({ {1, 3} }, meths.execute_lua([[
         local t = {}
         for k, v in pairs(require('nvimpam.job').jobids) do
           table.insert(t, {k, v}) end
         return t
       ]], {}))

    command("enew!")
    -- sleep needed to let the detaching happen
    sleep(100)
    chans = meths.list_chans()
    eq(nil, chans[3])
    eq({ }, meths.execute_lua([[
         local t = {}
         for k, v in pairs(require('nvimpam.job').jobids) do
           table.insert(t, {k, v}) end
         return t
       ]], {}))

  end)

  -- note: this also checks that we're using the debug binary
  it('includes a proper healthcheck', function()
    os.remove(alter_slashes("../target/release/nvimpam"))
    command('checkhealth nvimpam')
    -- needed to get rid of the indeterminism warning
    feed("G")

    --workaround for now, will fail if run on non-appveyor windows
    if os.getenv("APPVEYOR") then
      screen:expect([[
        {5: [No Name] }{6: [No Name] }{3:                                                          }{5:X}|
        ========================================================================         |
        ## Buffer updates                                                                |
          - {7:OK:} Function nvim_buf_attach exists!                                         |
                                                                                         |
        ## Nvimpam binary                                                                |
          - {7:OK:} binary found: nvimpam                                                    |
                                                                                         |
        ## Menu availability                                                             |
          - ERROR: `Vigemus/impromptu.nvim` is not installed                             |
            - ADVICE:                                                                    |
              - Visit `https://github.com/Vigemus/impromptu.nvim`.                       |
          - {7:OK:} Directory `pam_cards` found                                              |
        ^                                                                                 |
                                                                                         |
      ]])
    else
      screen:expect([[
        {5: [No Name] }{6: [No Name] }{3:                                                          }{5:X}|
        ========================================================================         |
        ## Buffer updates                                                                |
          - {7:OK:} Function nvim_buf_attach exists!                                         |
                                                                                         |
        ## Nvimpam binary                                                                |
          - {7:OK:} binary found: .././target/debug/nvimpam                                  |
                                                                                         |
        ## Menu availability                                                             |
          - ERROR: `Vigemus/impromptu.nvim` is not installed                             |
            - ADVICE:                                                                    |
              - Visit `https://github.com/Vigemus/impromptu.nvim`.                       |
          - {7:OK:} Directory `pam_cards` found                                              |
        ^                                                                                 |
                                                                                         |
      ]])
    end
  end)

  it('provides a cardmenu', function()
    -- impromptu uses this HL group, but the runtime isn't loaded on the
    -- neovim test runner
    command("hi def Comment cterm=NONE")

    command("set rtp+=../../impromptu.nvim")
    command("set nowrap")
    command('edit ' .. alter_slashes('../files/example.pc'))


    command('NvimPamMenu')
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ]]
      ..
      alter_slashes(
      "{3:../files/example.pc                                                              }|"
      )
      ..
      "\n"
      ..
      [[
      ^                                                                                 |
       [r] Auxiliaries [i] Material                                                    |
       [n] Constraint  [N] Node                                                        |
       [t] Contact     [h] Others                                                      |
       [l] Control     [O] Output                                                      |
       [e] Element     [a] Part                                                        |
       [d] Load        [f] Safety                                                      |
       [M] MMC         [q] Close this prompt                                           |
                                                                                       |
                                                                                       |
                                                                                       |
      {4:[Scratch] [RO]                                                                   }|
                                                                                       |
    ]])
    feed("r")
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ]]
      ..
      alter_slashes(
      "{3:../files/example.pc                                                              }|"
      )
      ..
      "\n"
      ..
      [[
      ^                                                                                 |
       [h] Move up one level                   [L] PLANEs                              |
       [r] CDATA Card                          [y] PYFUNC Python Function              |
       [d] DELEM - Deleted Element Card        [R] RUPMOs                              |
       [s] FRAMEs                              [S] SENSORs                             |
       [i] FRICTion Models                     [e] SURFA Surface Definition            |
       [n] FUNCSW Function Switch              [a] UDATA User Data                     |
       [c] FUNCT Function Card                 [0] VECTOR Type 0                       |
       [f] GROUP Group Definition              [1] VECTOR Type 1                       |
       [b] LOOKU Lookup Table                  [q] Close this prompt                   |
       [v] NLAVE Non Local Averadge Definition                                         |
      {4:[Scratch] [RO]                                                                   }|
                                                                                       |
    ]])
    feed("r")
    screen:expect([[
      ^INPUTVERSION 2011                                                                |
      $CDATA Card                                                                      |
      CDATA /         1                                                                |
      NAME CDATA Card                                                                  |
      END_CDATA                                                                        |
      ANALYSIS EXPLICIT                                                                |
      SOLVER    CRASH                                                                  |
      $                                                                                |
      $----------------------------------------------------------------                |
      $     PAM-SOLID SOLVER CONTROLS                                                  |
      $----------------------------------------------------------------                |
      UNIT       MM       KG       MS   KELVIN                                         |
      SIGNAL      YES                                                                  |
      $                                                                                |
                                                                                       |
    ]])

  end)

  it('provides a filter-based cardmenu', function()
    -- impromptu uses this HL group, but the runtime isn't loaded on the
    -- neovim test runner
    command("hi def Keyword cterm=NONE")

    command("set rtp+=../../impromptu.nvim")
    command("set nowrap")
    command('edit ' .. alter_slashes('../files/example.pc'))


    command('call luaeval("require(\'nvimpam.filter_cards\').filter_cards()")')
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ]]
      ..
      alter_slashes(
      "{3:../files/example.pc                                                              }|"
      )
      ..
      "\n"
      ..
      [[
      Select a card                                                                    |
      ─────────────────────────────────────────────────────────────────────────────────|
      {13: →} 3D Boundary Condition                                                         |
         ACFLD Acceleration Field                                                      |
         ACTUA - Joint Actuator Definition                                             |
         Acoustic Plane Wave                                                           |
         BAGIN Definition                                                              |
         BAR Element                                                                   |
         BDFOR Body Forces                                                             |
      ─────────────────────────────────────────────────────────────────────────────────|
      ^                                                                                 |
      {4:[Scratch]                                                                        }|
                                                                                       |
    ]])

    feed("i3d")
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ]]
      ..
      alter_slashes(
      "{3:../files/example.pc                                                              }|"
      )
      ..
      "\n"
      ..
      [[
      Select a card                                                                    |
      ─────────────────────────────────────────────────────────────────────────────────|
      {13: →} 3D Boundary Condition                                                         |
         FBC3D Prescribed Motion onto Fluid Media                                      |
         PART Type COS3D                                                               |
                                                                                       |
                                                                                       |
                                                                                       |
                                                                                       |
      ─────────────────────────────────────────────────────────────────────────────────|
      3d^                                                                               |
      {4:[Scratch]                                                                        }|
      {6:-- INSERT --}                                                                     |
    ]])

    feed("<C-j><Enter>")
    screen:expect([[
      ^INPUTVERSION 2011                                                                |
      #FBC3D - Prescribed Motion onto Fluid Media                                      |
      $#         IDNOQUALIFIER   IFUN1   IFUN2   IFUN3   SFAC1   SFAC2   SFAC3    IFAND|
      FBC3D /        0ACCE           0       0       0      1.      1.      1.       0 |
      $#                                                                         TITLE |
      NAME FBC3D / ->1                                                                 |
              END                                                                      |
      ANALYSIS EXPLICIT                                                                |
      SOLVER    CRASH                                                                  |
      $                                                                                |
      $----------------------------------------------------------------                |
      $     PAM-SOLID SOLVER CONTROLS                                                  |
      $----------------------------------------------------------------                |
      UNIT       MM       KG       MS   KELVIN                                         |
                                                                                       |
    ]])

  end)

  it('updates error highlighting', function()
    insert("NODE  /        1              0.             0.5              0.")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    command("NvimPamHighlightScreen")
    sleep(10)

    feed("1G")
    feed("f0rx")
    sleep(10)
    screen:expect([[
      {8:NODE  / }{9:       1}{11:              ^x.}{9:             0.5}{10:              0.}                 |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
                                                                                       |
    ]])

    feed("u")
    sleep(10)
    screen:expect([[
      {8:NODE  / }{9:       1}{10:              ^0.}{9:             0.5}{10:              0.}                 |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])
  end)

  it('deletes highlighting when a line becomes invalid', function()
    insert([[
      NODE  /        1              0.             0.5              0.
      NODE  /        1              0.             0.5              0.]]
    )
    command('set ft=pamcrash')
    command('NvimPamAttach')
    sleep(10)
    command("NvimPamHighlightScreen")
    sleep(10)

    feed("1G0")
    feed("<C-v>jx")
    sleep(10)
    screen:expect([[
      ^ODE  /        1              0.             0.5              0.                  |
      ODE  /        1              0.             0.5              0.                  |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
                                                                                       |
    ]])

  end)

  it('keeps highlights after undo', function()
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    feed("29G")
    command('NvimPamHighlightScreen')
    feed("yy2P")
    sleep(10)
    screen:expect([[
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      $#         IDNOD               X               Y               Z                 |
      {8:^NODE  / }{9:       1}{10:              0.}{9:            50.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:            50.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:            50.5}{10:              0.}                 |
      {8:NODE  / }{9:       2}{10:              0.}{9:            50.5}{10:             10.}                 |
      {8:NODE  / }{9:       3}{10:              0.}{9:            50.5}{10:             20.}                 |
      {8:NODE  / }{9:       4}{10:              0.}{9:            50.5}{10:             30.}                 |
      {8:NODE  / }{9:       5}{10:              0.}{9:            50.5}{10:             40.}                 |
      {8:NODE  / }{9:       6}{10:              0.}{9:            50.5}{10:             50.}                 |
                                                                                       |
    ]])

    feed("u")
    sleep(10)

    screen:expect([[
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      $#         IDNOD               X               Y               Z                 |
      {8:^NODE  / }{9:       1}{10:              0.}{9:            50.5}{10:              0.}                 |
      {8:NODE  / }{9:       2}{10:              0.}{9:            50.5}{10:             10.}                 |
      {8:NODE  / }{9:       3}{10:              0.}{9:            50.5}{10:             20.}                 |
      {8:NODE  / }{9:       4}{10:              0.}{9:            50.5}{10:             30.}                 |
      {8:NODE  / }{9:       5}{10:              0.}{9:            50.5}{10:             40.}                 |
      {8:NODE  / }{9:       6}{10:              0.}{9:            50.5}{10:             50.}                 |
      NODE  /        7              0.            50.5             60.                 |
      NODE  /        8              0.            50.5             70.                 |
      {IGNORE}|
    ]])

  end)

  it("doesn't mind deleting the whole buffer", function()
    insert(input)
    feed("1G")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    sleep(10)
    command('NvimPamUpdateFolds')
    feed("1G")

    screen:expect([[
      {1:^ 4 lines: Node ··································································}|
      #Comment here                                                                    |
      {1: 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    feed("dG")
    command('NvimPamUpdateFolds')
    feed("p")

    screen:expect([[
                                                                                       |
      {8:^NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      #Comment here                                                                    |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      #Comment                                                                         |
      #Comment                                                                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      $Comment                                                                         |
      {IGNORE}|
    ]])
  end)

  it("works when pasting at the end of the buffer", function()
    insert(input)
    feed("G")
    feed("dd")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    sleep(10)

    feed("yy")
    feed("zz")
    feed("4p")
    sleep(10)
    screen:expect([[
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {8:^SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {8:SHELL / }{9:    3129}{10:       1}{9:       1}{10:    2967}{9:    2971}{10:    2970}                         |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    command('NvimPamUpdateFolds')
    screen:expect([[
      {1: 3 lines: Node ··································································}|
      {1:^ 8 lines: Shell ·································································}|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      4 more lines                                                                     |
    ]])

    feed("zE")
    feed("u")
    sleep(10)
    screen:expect([[
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      ^SHELL /     3129       1       1    2967    2971    2970                         |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

    command('NvimPamUpdateFolds')
    screen:expect([[
      {1: 3 lines: Node ··································································}|
      {1:^ 4 lines: Shell ·································································}|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      4 fewer lines; before #3  0 seconds ago                                          |
    ]])
  end)

  it("works when pasting at the beginning of the buffer", function()
    insert(input)
    feed("1G")
    command('set ft=pamcrash')
    command('NvimPamAttach')
    sleep(10)

    feed("yy3P")
    screen:expect([[
      {8:^NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      {8:NODE  / }{9:       1}{10:              0.}{9:             0.5}{10:              0.}                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      #Comment                                                                         |
      #Comment                                                                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {IGNORE}|
    ]])

    command('NvimPamUpdateFolds')
    screen:expect([[
      {1:^ 7 lines: Node ··································································}|
      #Comment here                                                                    |
      {1: 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      3 more lines                                                                     |
    ]])

    feed("zE")
    feed("u")
    sleep(10)
    screen:expect([[
      ^NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      NODE  /        1              0.             0.5              0.                 |
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      #Comment                                                                         |
      #Comment                                                                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      $Comment                                                                         |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {IGNORE}|
    ]])

    feed("1G")
    command('NvimPamUpdateFolds')

    screen:expect([[
      {1:^ 4 lines: Node ··································································}|
      #Comment here                                                                    |
      {1: 10 lines: Shell ································································}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1: 3 lines: Node ··································································}|
      {1: 4 lines: Shell ·································································}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {IGNORE}|
    ]])

  end)

  it('removes highlights and folds after a DetachEvent', function()
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    feed("29G")
    command('NvimPamHighlightScreen')
    sleep(10)
    command("NvimPamUpdateFolds")
    command("edit!")

    screen:expect([[
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      $#         IDNOD               X               Y               Z                 |
      ^NODE  /        1              0.            50.5              0.                 |
      NODE  /        2              0.            50.5             10.                 |
      NODE  /        3              0.            50.5             20.                 |
      NODE  /        4              0.            50.5             30.                 |
      NODE  /        5              0.            50.5             40.                 |
      NODE  /        6              0.            50.5             50.                 |
      NODE  /        7              0.            50.5             60.                 |
      NODE  /        8              0.            50.5             70.                 |
                                                                                       |
    ]])
  end)

  it('properly undoes ftplugin settings', function()
    command('edit ' .. alter_slashes('../files/example.pc'))
    eq(eval("&foldtext"), "Nvimpam_foldtext()")

    feed(":NvimPam<Tab>")
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ANALYSIS EXPLICIT                                                                |
      SOLVER    CRASH                                                                  |
      $                                                                                |
      $----------------------------------------------------------------                |
      $     PAM-SOLID SOLVER CONTROLS                                                  |
      $----------------------------------------------------------------                |
      UNIT       MM       KG       MS   KELVIN                                         |
      SIGNAL      YES                                                                  |
      $                                                                                |
      TITLE /  BoxBeam fine meshed model                                               |
      RUNEND/                                                                          |
       TIME      15.01                                                                 |
      {14:NvimPamAttach}{4:  NvimPamHighlightScreen  NvimPamMenu  NvimPamUpdateFolds           }|
      :NvimPamAttach^                                                                   |
    ]])

    feed("<Esc>")
    command('set ft=text')
    eq(eval("&foldtext"), "foldtext()")
    feed(":NvimPam<Tab>")
    screen:expect([[
      INPUTVERSION 2011                                                                |
      ANALYSIS EXPLICIT                                                                |
      SOLVER    CRASH                                                                  |
      $                                                                                |
      $----------------------------------------------------------------                |
      $     PAM-SOLID SOLVER CONTROLS                                                  |
      $----------------------------------------------------------------                |
      UNIT       MM       KG       MS   KELVIN                                         |
      SIGNAL      YES                                                                  |
      $                                                                                |
      TITLE /  BoxBeam fine meshed model                                               |
      RUNEND/                                                                          |
       TIME      15.01                                                                 |
      END_RUNEND                                                                       |
      :NvimPam^                                                                         |
    ]])
  end)

end)
