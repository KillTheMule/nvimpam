local helpers = require('test.functional.helpers')(after_each)
local Screen = require('test.functional.ui.screen')
local clear, command = helpers.clear, helpers.command
local feed, alter_slashes = helpers.feed, helpers.alter_slashes
local insert = helpers.insert

describe('nvimpam', function()
  local screen

  before_each(function()
    clear()
    screen = Screen.new(81, 15)
    screen:attach()
    screen:set_default_attr_ids({
      [1] = {foreground = Screen.colors.DarkBlue, background = Screen.colors.LightGray},
      [2] = {bold = true, foreground = Screen.colors.Blue1},
    })
    command('set rtp+=' .. alter_slashes('../'))
    command('source ' .. alter_slashes('../plugin/nvimpam.vim'))
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
    command('NvimPamConnect')
    feed("28G")

    screen:expect([[
       ERFOUTPUT        3        0                                                     |
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      ^$#         IDNOD               X               Y               Z                 |
      {1:+--725 lines: NODE  /        1              0.            50.5              0.---}|
      $----------------------------------------------------------------                |
      $     MATERIAL DEFINITIONS                                                       |
      $----------------------------------------------------------------                |
      $ boxbeam                                                                        |
      $#         IDMAT   MATYP             RHO   ISINT    ISHG  ISTRAT   IFROZ         |
      MATER /        3     103         7.85E-6       0       0       0       0         |
      rust client connected to neovim                                                  |
    ]])
  end)

  it('can deal with insertions', function()
    insert(input)
    command('NvimPamConnect')
    feed("1G")

    screen:expect([[
      {1:^+--  4 lines: NODE  /        1              0.             0.5              0.---}|
      #Comment here                                                                    |
      {1:+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.---}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      rust client connected to neovim                                                  |
    ]])

    command("echo") -- clear the command line
    feed("zo")
    feed("yyP")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:^+--  5 lines: NODE  /        1              0.             0.5              0.---}|
      #Comment here                                                                    |
      {1:+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.---}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
                                                                                       |
    ]])
  end)

  it('can deal with deletions', function()
    insert(input)
    command('NvimPamConnect')
    feed("1G")

    command("7,9d")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.---}|
      #Comment here                                                                    |
      {1:^+--  7 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.---}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      rust client connected to neovim                                                  |
    ]])
  end)

  it('can deal with updates and undo', function()
    insert(input)
    command('NvimPamConnect')
    feed("1G")
    command("set nohls")

    command(":7,9s/^SHELL/NODE")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.---}|
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {1:^+--  2 lines: NODE /     3129       1       1    2967    2971    2970------------}|
      #Comment                                                                         |
      #Comment                                                                         |
      {1:+--  5 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.---}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      rust client connected to neovim                                                  |
    ]])

    feed("u")
    command("NvimPamUpdateFolds")
    command("echo") -- clear the command line
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.---}|
      #Comment here                                                                    |
      {1:^+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.---}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970-----------}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
      {2:~                                                                                }|
                                                                                       |
    ]])
  end)
end)
