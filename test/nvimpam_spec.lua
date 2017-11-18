local helpers = require('test.functional.helpers')(after_each)
local Screen = require('test.functional.ui.screen')
local clear, command = helpers.clear, helpers.command
local feed, alter_slashes = helpers.feed, helpers.alter_slashes

describe('nvimpam', function()
  local screen

  before_each(function()
    clear()
    screen = Screen.new(81, 15)
    screen:attach()
    screen:set_default_attr_ids({
      [1]  = {foreground = Screen.colors.DarkBlue, background = Screen.colors.LightGray},
    })
  end)

  after_each(function()
    screen:detach()
  end)

  it('basically works', function()
    command('set rtp+=' .. alter_slashes('../'))
    command('source ' .. alter_slashes('../plugin/nvimpam.vim'))
    command('edit ' .. alter_slashes('../aux/example.pc'))
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
      {1:+--714 lines: NODE  /        1              0.            50.5              0.---}|
      $----------------------------------------------------------------                |
      $     NODE DEFINITIONS                                                           |
      $----------------------------------------------------------------                |
      {1:+--  8 lines: NODE  /     1001       66.055756       -0.500003      223.527725---}|
      $----------------------------------------------------------------                |
      $     MATERIAL DEFINITIONS                                                       |
      rust client connected to neovim                                                  |
    ]])
  end)
end)
