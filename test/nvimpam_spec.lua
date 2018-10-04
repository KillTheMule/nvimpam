local helpers = require('test.functional.helpers')(after_each)
local Screen = require('test.functional.ui.screen')
local clear, command = helpers.clear, helpers.command
local feed, alter_slashes = helpers.feed, helpers.alter_slashes
local insert = helpers.insert
local meths = helpers.meths
local eq = helpers.eq
local dedent = helpers.dedent

local is_ci = os.getenv("TRAVIS") or os.getenv("APPVEYOR")

-- canonical order of ext keys, used  to generate asserts
local ext_keys = {
  'popupmenu', 'cmdline', 'cmdline_block', 'wildmenu_items', 'wildmenu_pos'
}
local function isempty(v)
  return type(v) == 'table' and next(v) == nil
end
-- Override this function to ignore the last line, i.e. the command
-- line, since it seems increasingly non-deterministic, and we don't
-- care a lot about it anyways
function Screen:expect(expected, attr_ids, attr_ignore)
  local grid, condition = nil, nil
  local expected_rows = {}
  if type(expected) == "table" then
    assert(not (attr_ids ~= nil or attr_ignore ~= nil))
    local is_key = {grid=true, attr_ids=true, attr_ignore=true, condition=true,
                    any=true, mode=true}
    for _, v in ipairs(ext_keys) do
      is_key[v] = true
    end
    for k, _ in pairs(expected) do
      if not is_key[k] then
        error("Screen:expect: Unknown keyword argument '"..k.."'")
      end
    end
    grid = expected.grid
    attr_ids = expected.attr_ids
    attr_ignore = expected.attr_ignore
    condition = expected.condition
    assert(not (expected.any ~= nil and grid ~= nil))
  elseif type(expected) == "string" then
    grid = expected
    expected = {}
  elseif type(expected) == "function" then
    assert(not (attr_ids ~= nil or attr_ignore ~= nil))
    condition = expected
    expected = {}
  else
    assert(false)
  end

  if grid ~= nil then
    -- Remove the last line and dedent. Note that gsub returns more then one
    -- value.
    grid = dedent(grid:gsub('\n[ ]+$', ''), 0)
    for row in grid:gmatch('[^\n]+') do
      row = row:sub(1, #row - 1) -- Last char must be the screen delimiter.
      table.insert(expected_rows, row)
    end
  end
  local attr_state = {
      ids = attr_ids or self._default_attr_ids,
      ignore = attr_ignore or self._default_attr_ignore,
  }
  if self._options.ext_hlstate then
    attr_state.id_to_index = self:hlstate_check_attrs(attr_state.ids or {})
  end
  self._new_attrs = false
  self:wait(function()
    if condition ~= nil then
      local status, res = pcall(condition)
      if not status then
        return tostring(res)
      end
    end

    if grid ~= nil and self._height ~= #expected_rows then
      return ("Expected screen state's row count(" .. #expected_rows
              .. ') differs from configured height(' .. self._height .. ') of Screen.')
    end

    if self._options.ext_hlstate and self._new_attrs then
      attr_state.id_to_index = self:hlstate_check_attrs(attr_state.ids or {})
    end

    local actual_rows = {}
    for i = 1, self._height do
      actual_rows[i] = self:_row_repr(self._rows[i], attr_state)
    end

    if expected.any ~= nil then
      -- Search for `any` anywhere in the screen lines.
      local actual_screen_str = table.concat(actual_rows, '\n')
      if nil == string.find(actual_screen_str, expected.any) then
        return (
          'Failed to match any screen lines.\n'
          .. 'Expected (anywhere): "' .. expected.any .. '"\n'
          .. 'Actual:\n  |' .. table.concat(actual_rows, '|\n  |') .. '|\n\n')
      end
    end

    if grid ~= nil then
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

    -- Extension features. The default expectations should cover the case of
    -- the ext_ feature being disabled, or the feature currently not activated
    -- (for instance no external cmdline visible). Some extensions require
    -- preprocessing to prepresent highlights in a reproducible way.
    local extstate = self:_extstate_repr(attr_state)

    -- convert assertion errors into invalid screen state descriptions
    local status, res = pcall(function()
      for _, k in ipairs(ext_keys) do
        -- Empty states is considered the default and need not be mentioned
        if not (expected[k] == nil and isempty(extstate[k])) then
          eq(expected[k], extstate[k], k)
        end
      end
      if expected.mode ~= nil then
        eq(expected.mode, self.mode, "mode")
      end
    end)
    if not status then
      return tostring(res)
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
      [3] = {reverse = true,},
      [4] = {bold = true, reverse = true},
      [5] = {background = Screen.colors.LightGrey, underline = true},
      [6] = {bold = true},
      [7] = {foreground = Screen.colors.Grey3, background = 6291200}
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
      rust client connected to neovim                                                 |
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
                                                                                       |
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
                                                                                       |
    ]])
  end)

  it('can deal with insertions', function()
    insert(input)
    command('set ft=pamcrash')
    command('NvimPamAttach')
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
      rust client connected to neovim                                                  |
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
                                                                                       |
    ]])
  end)

  it('can deal with deletions', function()
    insert(input)
    command('set ft=pamcrash')
    command('NvimPamAttach')
    feed("1G")

    command("7,9d")
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
      rust client connected to neovim                                                  |
    ]])
  end)

  it('can deal with updates and undo', function()
    insert(input)
    command('set ft=pamcrash')
    command('NvimPamAttach')
    feed("1G")
    command("set nohls")

    feed("zR")
    command(":7,9s/^SHELL/NODE ")
    command("NvimPamUpdateFolds")
    screen:expect([[
      {1: 4 lines: Node ··································································}|
      #Comment here                                                                    |
      SHELL /     3129       1       1    2967    2971    2970                         |
      {1:^ 2 lines: Node ··································································}|
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
      rust client connected to neovim                                                  |
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
                                                                                       |
    ]])
  end)

  it('starts a new instance for a new buffer', function()
    command("set nowrap")
    command('edit ' .. alter_slashes('../files/example.pc'))
    command('NvimPamAttach')
    if is_ci then
      helpers.sleep(1000)
    else
      helpers.sleep(1000)
    end
    feed("28G")
    command("vs " .. alter_slashes("../files/example2.pc"))
    command("NvimPamAttach")
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
      rust client connected to neovim                                                  |
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

    command("NvimPamDetach")
    if is_ci then
      helpers.sleep(1000)
    else
      helpers.sleep(10)
    end
    chans = meths.list_chans()
    eq(chans[3].client.name, 'nvimpam')
    eq(nil, chans[4])

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
                                                                                         |
        health#nvimpam#check                                                             |
        ========================================================================         |
          - {7:OK:} Function nvim_buf_attach exists!                                         |
          - {7:OK:} binary found: nvimpam                                                    |
        ^                                                                                 |
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
                                                                                         |
      ]])
    else
      screen:expect([[
        {5: [No Name] }{6: [No Name] }{3:                                                          }{5:X}|
                                                                                         |
        health#nvimpam#check                                                             |
        ========================================================================         |
          - {7:OK:} Function nvim_buf_attach exists!                                         |
          - {7:OK:} binary found: ../target/debug/nvimpam                                    |
        ^                                                                                 |
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
        {2:~                                                                                }|
                                                                                         |
      ]])
    end
  end)

end)
