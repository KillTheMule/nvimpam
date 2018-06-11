local helpers = require('test.functional.helpers')(after_each)
local Screen = require('test.functional.ui.screen')
local clear, command = helpers.clear, helpers.command
local feed, alter_slashes = helpers.feed, helpers.alter_slashes
local insert = helpers.insert

-- Override this function to ignore the last line, i.e. the command
-- line, since it seems increasingly non-deterministic, and we don't
-- care a lot about it anyways
function Screen:expect(expected, attr_ids, attr_ignore, condition, any)
  local expected_rows = {}
  if type(expected) ~= "string" then
    assert(not (attr_ids or attr_ignore or condition or any))
    condition = expected
    expected = nil
  else
    -- Remove the last line and dedent. Note that gsub returns more then one
    -- value.
    expected = helpers.dedent(expected:gsub('\n[ ]+$', ''), 0)
    for row in expected:gmatch('[^\n]+') do
      row = row:sub(1, #row - 1) -- Last char must be the screen delimiter.
      table.insert(expected_rows, row)
    end
    if not any then
      assert(self._height == #expected_rows,
        "Expected screen state's row count(" .. #expected_rows
        .. ') differs from configured height(' .. self._height .. ') of Screen.')
    end
  end
  local ids = attr_ids or self._default_attr_ids
  local ignore = attr_ignore or self._default_attr_ignore
  self:wait(function()
    if condition ~= nil then
      local status, res = pcall(condition)
      if not status then
        return tostring(res)
      end
    end
    local actual_rows = {}
    for i = 1, self._height do
      actual_rows[i] = self:_row_repr(self._rows[i], ids, ignore)
    end

    if expected == nil then
      return
    elseif any then
      -- Search for `expected` anywhere in the screen lines.
      local actual_screen_str = table.concat(actual_rows, '\n')
      if nil == string.find(actual_screen_str, expected) then
        return (
          'Failed to match any screen lines.\n'
          .. 'Expected (anywhere): "' .. expected .. '"\n'
          .. 'Actual:\n  |' .. table.concat(actual_rows, '|\n  |') .. '|\n\n')
      end
    else
      -- `expected` must match the screen lines exactly.
      for i = 1, self._height-1 do
        if expected_rows[i] ~= actual_rows[i] then
          local msg_expected_rows = {}
          for j = 1, #expected_rows do
            msg_expected_rows[j] = expected_rows[j]
          end
          msg_expected_rows[i] = '*' .. msg_expected_rows[i]
          actual_rows[i] = '*' .. actual_rows[i]
          return (
            'Row ' .. tostring(i) .. ' did not match.\n'
            ..'Expected:\n  |'..table.concat(msg_expected_rows, '|\n  |')..'|\n'
            ..'Actual:\n  |'..table.concat(actual_rows, '|\n  |')..'|\n\n'..[[
To print the expect() call that would assert the current screen state, use
screen:snapshot_util(). In case of non-deterministic failures, use
screen:redraw_debug() to show all intermediate screen states.  ]])
        end
      end
    end
  end)
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
    command('NvimPamAttach')
    feed("28G")

    screen:expect([[
       ERFOUTPUT        3        0                                                     |
      NODPLOT    DFLT                                                                  |
      SOLPLOT     ALL                                                                  |
       SHLPLOT   DFLT                                                                  |
      END_OCTRL                                                                        |
      $                                                                                |
      ^$#         IDNOD               X               Y               Z                 |
      {1:+--725 lines: NODE  /        1              0.            50.5              0.···}|
      $----------------------------------------------------------------                |
      $     MATERIAL DEFINITIONS                                                       |
      $----------------------------------------------------------------                |
      $ boxbeam                                                                        |
      $#         IDMAT   MATYP             RHO   ISINT    ISHG  ISTRAT   IFROZ         |
      MATER /        3     103         7.85E-6       0       0       0       0         |
      rust client connected to neovim                                                 |
    ]])
  end)

  it('can deal with insertions', function()
    insert(input)
    command('NvimPamAttach')
    feed("1G")

    screen:expect([[
      {1:^+--  4 lines: NODE  /        1              0.             0.5              0.···}|
      #Comment here                                                                    |
      {1:+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.···}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
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
      {1:^+--  5 lines: NODE  /        1              0.             0.5              0.···}|
      #Comment here                                                                    |
      {1:+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.···}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
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
    command('NvimPamAttach')
    feed("1G")

    command("7,9d")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.···}|
      #Comment here                                                                    |
      {1:^+--  7 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.···}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
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
    command('NvimPamAttach')
    feed("1G")
    command("set nohls")

    feed("zR")
    command(":7,9s/^SHELL/NODE ")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.···}|
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {1:^+--  2 lines: NODE  /     3129       1       1    2967    2971    2970···········}|
      #Comment                                                                         |
      #Comment                                                                         |
      {1:+--  5 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.···}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
                                                                                       |
      {2:~                                                                                }|
      {2:~                                                                                }|
      rust client connected to neovim                                                  |
    ]])

    feed("u")
    command("NvimPamUpdateFolds")
    command("echo") -- clear the command line
    screen:expect([[
      {1:+--  4 lines: NODE  /        1              0.             0.5              0.···}|
      #Comment here                                                                    |
      {1:^+-- 10 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
      $Comment                                                                         |
      #Comment                                                                         |
      {1:+--  3 lines: NODE  /        1              0.             0.5              0.···}|
      {1:+--  4 lines: SHELL /     3129       1       1    2967    2971    2970···········}|
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
