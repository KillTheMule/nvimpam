local helpers = require('test.functional.helpers')(after_each)
local eq, ok = helpers.eq, helpers.ok
local buffer, command, eval, nvim, next_msg = helpers.buffer,
  helpers.command, helpers.eval, helpers.nvim, helpers.next_msg

local origlines = {"original line 1",
                   "original line 2",
                   "original line 3",
                   "original line 4",
                   "original line 5",
                   "original line 6"}

local function expectn(name, args)
  -- expect the next message to be the specified notification event
  eq({'notification', name, args}, next_msg())
end

local function sendkeys(keys)
  nvim('input', keys)
  -- give neovim some time to process msgpack requests before possibly sending
  -- more key presses - otherwise they all pile up in the queue and get
  -- processed at once
  local ntime = os.clock() + 0.1
  repeat until os.clock() > ntime
end

local function open(activate, lines)
  local filename = helpers.tmpname()
  helpers.write_file(filename, table.concat(lines, "\n").."\n", true)
  command('edit ' .. filename)
  local b = nvim('get_current_buf')
  -- what is the value of b:changedtick?
  local tick = eval('b:changedtick')

  -- turn on live updates, ensure that the nvim_buf_updates_start messages
  -- arrive as expectected
  if activate then
    ok(buffer('event_sub', b, true))
    expectn('nvim_buf_updates_start', {b, tick, lines, false})
  end

  return b, tick, filename
end

local function editoriginal(activate, lines)
  if not lines then
    lines = origlines
  end
  -- load up the file with the correct contents
  helpers.clear()
  return open(activate, lines)
end

local function reopen(buf, expectedlines)
  ok(buffer('event_unsub', buf))
  expectn('nvim_buf_updates_end', {buf})
  -- for some reason the :edit! increments tick by 2
  command('edit!')
  local tick = eval('b:changedtick')
  ok(buffer('event_sub', buf, true))
  expectn('nvim_buf_updates_start', {buf, tick, expectedlines, false})
  command('normal! gg')
  return tick
end

local function reopenwithfolds(b)
  -- discard any changes to the buffer
  local tick = reopen(b, origlines)

  -- use markers for folds, make all folds open by default
  command('setlocal foldmethod=marker foldlevel=20')

  -- add a fold
  command('2,4fold')
  tick = tick + 1
  expectn('nvim_buf_update', {b, tick, 1, 4, {'original line 2/*{{{*/',
                                          'original line 3',
                                          'original line 4/*}}}*/'}})
  -- make a new fold that wraps lines 1-6
  command('1,6fold')
  tick = tick + 1
  expectn('nvim_buf_update', {b, tick, 0, 6, {'original line 1/*{{{*/',
                                          'original line 2/*{{{*/',
                                          'original line 3',
                                          'original line 4/*}}}*/',
                                          'original line 5',
                                          'original line 6/*}}}*/'}})
  return tick
end

describe('buffer events', function()
  it('when you add line to a buffer', function()
    local b, tick = editoriginal(true)

    -- add a new line at the start of the buffer
    command('normal! GyyggP')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 0, {'original line 6'}})

    -- add multiple lines at the start of the file
    command('normal! GkkyGggP')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 0, {'original line 4',
                                           'original line 5',
                                           'original line 6'}})

    -- add one line to the middle of the file, several times
    command('normal! ggYjjp')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 3, {'original line 4'}})
    command('normal! p')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 4, 4, {'original line 4'}})
    command('normal! p')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 5, 5, {'original line 4'}})

    -- add multiple lines to the middle of the file
    command('normal! gg4Yjjp')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 3, {'original line 4',
                                           'original line 5',
                                           'original line 6',
                                           'original line 4'}})

    -- add one line to the end of the file
    command('normal! ggYGp')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 17, 17, {'original line 4'}})

    -- add one line to the end of the file, several times
    command('normal! ggYGppp')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 18, 18, {'original line 4'}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 19, 19, {'original line 4'}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 20, 20, {'original line 4'}})

    -- add several lines to the end of the file, several times
    command('normal! gg4YGp')
    command('normal! Gp')
    command('normal! Gp')
    local firstfour = {'original line 4',
                 'original line 5',
                 'original line 6',
                 'original line 4'}
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 21, 21, firstfour})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 25, 25, firstfour})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 29, 29, firstfour})

    -- create a new empty buffer and wipe out the old one ... this will
    -- turn off live updates
    command('enew!')
    expectn('nvim_buf_updates_end', {b})

    -- add a line at the start of an empty file
    command('enew')
    tick = eval('b:changedtick')
    local b2 = nvim('get_current_buf')
    ok(buffer('event_sub', b2, true))
    expectn('nvim_buf_updates_start', {b2, tick, {""}, false})
    eval('append(0, ["new line 1"])')
    tick = tick + 1
    expectn('nvim_buf_update', {b2, tick, 0, 0, {'new line 1'}})

    -- turn off live updates manually
    buffer('event_unsub', b2)
    expectn('nvim_buf_updates_end', {b2})

    -- add multiple lines to a blank file
    command('enew!')
    local b3 = nvim('get_current_buf')
    ok(buffer('event_sub', b3, true))
    tick = eval('b:changedtick')
    expectn('nvim_buf_updates_start', {b3, tick, {""}, false})
    eval('append(0, ["new line 1", "new line 2", "new line 3"])')
    tick = tick + 1
    expectn('nvim_buf_update', {b3, tick, 0, 0, {'new line 1',
                                            'new line 2',
                                            'new line 3'}})

    -- use the API itself to add a line to the start of the buffer
    buffer('set_lines', b3, 0, 0, true, {'New First Line'})
    tick = tick + 1
    expectn('nvim_buf_update', {b3, tick, 0, 0, {"New First Line"}})
  end)

  it('knows when you remove lines from a buffer', function()
    local b, tick = editoriginal(true)

    -- remove one line from start of file
    command('normal! dd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {}})

    -- remove multiple lines from the start of the file
    command('normal! 4dd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 4, {}})

    -- remove multiple lines from middle of file
    tick = reopen(b, origlines)
    command('normal! jj3dd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 5, {}})

    -- remove one line from the end of the file
    tick = reopen(b, origlines)
    command('normal! Gdd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 5, 6, {}})

    -- remove multiple lines from the end of the file
    tick = reopen(b, origlines)
    command('normal! 4G3dd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 6, {}})

    -- pretend to remove heaps lines from the end of the file but really
    -- just remove two
    tick = reopen(b, origlines)
    command('normal! Gk5dd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 4, 6, {}})
  end)

  it('knows when you modify lines of text', function()
    local b, tick = editoriginal(true)

    -- some normal text editing
    command('normal! A555')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'original line 1555'}})
    command('normal! jj8X')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 3, {'origin3'}})

    -- modify multiple lines at once using visual block mode
    tick = reopen(b, origlines)
    command('normal! jjw')
    sendkeys('<C-v>jjllx')
    tick = tick + 1
    expectn('nvim_buf_update',
            {b, tick, 2, 5, {'original e 3', 'original e 4', 'original e 5'}})

    -- replace part of a line line using :s
    tick = reopen(b, origlines)
    command('3s/line 3/foo/')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 3, {'original foo'}})

    -- replace parts of several lines line using :s
    tick = reopen(b, origlines)
    command('%s/line [35]/foo/')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 5, {'original foo',
                                           'original line 4',
                                           'original foo'}})

    -- type text into the first line of a blank file, one character at a time
    command('enew!')
    tick = 2
    expectn('nvim_buf_updates_end', {b})
    local bnew = nvim('get_current_buf')
    ok(buffer('event_sub', bnew, true))
    expectn('nvim_buf_updates_start', {bnew, tick, {''}, false})
    sendkeys('i')
    sendkeys('h')
    sendkeys('e')
    sendkeys('l')
    sendkeys('l')
    sendkeys('o\nworld')
    expectn('nvim_buf_update', {bnew, tick + 1, 0, 1, {'h'}})
    expectn('nvim_buf_update', {bnew, tick + 2, 0, 1, {'he'}})
    expectn('nvim_buf_update', {bnew, tick + 3, 0, 1, {'hel'}})
    expectn('nvim_buf_update', {bnew, tick + 4, 0, 1, {'hell'}})
    expectn('nvim_buf_update', {bnew, tick + 5, 0, 1, {'hello'}})
    expectn('nvim_buf_update', {bnew, tick + 6, 0, 1, {'hello', ''}})
    expectn('nvim_buf_update', {bnew, tick + 7, 1, 2, {'world'}})
  end)

  it('knows when you replace lines', function()
    local b, tick = editoriginal(true)

    -- blast away parts of some lines with visual mode
    command('normal! jjwvjjllx')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 3, {'original '}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 4, {}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 4, {'e 5'}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 3, {'original e 5'}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 4, {}})

    -- blast away a few lines using :g
    tick = reopen(b, origlines)
    command('global/line [35]/delete')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 2, 3, {}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 4, {}})
  end)

  it('knows when you filter lines', function()
    -- Test filtering lines with !cat
    local b, tick = editoriginal(true, {"A", "C", "E", "B", "D", "F"})

    command('silent 2,5!cat')
    -- the change comes through as two changes:
    -- 1) addition of the new lines after the filtered lines
    -- 2) removal of the original lines
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 5, 5, {"C", "E", "B", "D"}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 5, {}})
  end)

  it('sends a sensible event when you use "o"', function()
    local b, tick = editoriginal(true, {'AAA', 'BBB'})
    command('set noautoindent nosmartindent')

    -- use 'o' to start a new line from a line with no indent
    command('normal! o')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 1, {""}})

    -- undo the change, indent line 1 a bit, and try again
    command('undo')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 2, {}})
    tick = tick + 1
    expectn('nvim_buf_changedtick', {b, tick})
    command('set autoindent')
    command('normal! >>')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {"\tAAA"}})
    command('normal! ommm')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 1, {"\t"}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 2, {"\tmmm"}})

    -- undo the change, and try again with 'O'
    command('undo')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 2, {'\t'}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 2, {}})
    tick = tick + 1
    expectn('nvim_buf_changedtick', {b, tick})
    command('normal! ggOmmm')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 0, {"\t"}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {"\tmmm"}})
  end)

  it('deactivates when your buffer changes outside vim', function()
    -- Test changing file from outside vim and reloading using :edit
    local lines = {"Line 1", "Line 2"};
    local b, tick, filename = editoriginal(true, lines)

    command('normal! x')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'ine 1'}})
    command('undo')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'Line 1'}})
    tick = tick + 1
    expectn('nvim_buf_changedtick', {b, tick})

    -- change the file directly
    local f = io.open(filename, 'a')
    f:write("another line\n")
    f:flush()
    f:close()

    -- reopen the file and watch live updates shut down
    command('edit')
    expectn('nvim_buf_updates_end', {b})
  end)

  it('allows a channel to watch multiple buffers at once', function()
    -- edit 3 buffers, make sure they all have windows visible so that when we
    -- move between buffers, none of them are unloaded
    local b1, tick1 = editoriginal(true, {'A1', 'A2'})
    local b1nr = eval('bufnr("")')
    command('split')
    local b2, tick2 = open(true, {'B1', 'B2'})
    local b2nr = eval('bufnr("")')
    command('split')
    local b3, tick3 = open(true, {'C1', 'C2'})
    local b3nr = eval('bufnr("")')

    -- make a new window for moving between buffers
    command('split')

    command('b'..b1nr)
    command('normal! x')
    tick1 = tick1 + 1
    expectn('nvim_buf_update', {b1, tick1, 0, 1, {'1'}})
    command('undo')
    tick1 = tick1 + 1
    expectn('nvim_buf_update', {b1, tick1, 0, 1, {'A1'}})
    tick1 = tick1 + 1
    expectn('nvim_buf_changedtick', {b1, tick1})

    command('b'..b2nr)
    command('normal! x')
    tick2 = tick2 + 1
    expectn('nvim_buf_update', {b2, tick2, 0, 1, {'1'}})
    command('undo')
    tick2 = tick2 + 1
    expectn('nvim_buf_update', {b2, tick2, 0, 1, {'B1'}})
    tick2 = tick2 + 1
    expectn('nvim_buf_changedtick', {b2, tick2})

    command('b'..b3nr)
    command('normal! x')
    tick3 = tick3 + 1
    expectn('nvim_buf_update', {b3, tick3, 0, 1, {'1'}})
    command('undo')
    tick3 = tick3 + 1
    expectn('nvim_buf_update', {b3, tick3, 0, 1, {'C1'}})
    tick3 = tick3 + 1
    expectn('nvim_buf_changedtick', {b3, tick3})
  end)

  it('doesn\'t get confused when you turn watching on/off many times',
     function()
    local channel = nvim('get_api_info')[1]
    local b, tick = editoriginal(false)

    -- turn on live updates many times
    ok(buffer('event_sub', b, true))
    ok(buffer('event_sub', b, true))
    ok(buffer('event_sub', b, true))
    ok(buffer('event_sub', b, true))
    ok(buffer('event_sub', b, true))
    expectn('nvim_buf_updates_start', {b, tick, origlines, false})
    eval('rpcnotify('..channel..', "Hello There")')
    expectn('Hello There', {})

    -- turn live updates off many times
    ok(buffer('event_unsub', b))
    ok(buffer('event_unsub', b))
    ok(buffer('event_unsub', b))
    ok(buffer('event_unsub', b))
    ok(buffer('event_unsub', b))
    expectn('nvim_buf_updates_end', {b})
    eval('rpcnotify('..channel..', "Hello Again")')
    expectn('Hello Again', {})
  end)

  it('is able to notify several channels at once', function()
    helpers.clear()

    -- create several new sessions, in addition to our main API
    local sessions = {}
    local pipe = helpers.new_pipename()
    eval("serverstart('"..pipe.."')")
    sessions[1] = helpers.connect(pipe)
    sessions[2] = helpers.connect(pipe)
    sessions[3] = helpers.connect(pipe)

    local function request(sessionnr, method, ...)
      local status, rv = sessions[sessionnr]:request(method, ...)
      if not status then
        error(rv[2])
      end
      return rv
    end

    local function wantn(sessionid, name, args)
      local session = sessions[sessionid]
      eq({'notification', name, args}, session:next_message())
    end

    -- edit a new file, but don't turn on live updates
    local lines = {'AAA', 'BBB'}
    local b, tick = open(false, lines)

    -- turn on live updates for sessions 1, 2 and 3
    ok(request(1, 'nvim_buf_event_sub', b, true))
    ok(request(2, 'nvim_buf_event_sub', b, true))
    ok(request(3, 'nvim_buf_event_sub', b, true))
    wantn(1, 'nvim_buf_updates_start', {b, tick, lines, false})
    wantn(2, 'nvim_buf_updates_start', {b, tick, lines, false})
    wantn(3, 'nvim_buf_updates_start', {b, tick, lines, false})

    -- make a change to the buffer
    command('normal! x')
    tick = tick + 1
    wantn(1, 'nvim_buf_update', {b, tick, 0, 1, {'AA'}})
    wantn(2, 'nvim_buf_update', {b, tick, 0, 1, {'AA'}})
    wantn(3, 'nvim_buf_update', {b, tick, 0, 1, {'AA'}})

    -- stop watching on channel 1
    ok(request(1, 'nvim_buf_event_unsub', b))
    wantn(1, 'nvim_buf_updates_end', {b})

    -- undo the change to buffer 1
    command('undo')
    tick = tick + 1
    wantn(2, 'nvim_buf_update', {b, tick, 0, 1, {'AAA'}})
    wantn(3, 'nvim_buf_update', {b, tick, 0, 1, {'AAA'}})
    tick = tick + 1
    wantn(2, 'nvim_buf_changedtick', {b, tick})
    wantn(3, 'nvim_buf_changedtick', {b, tick})

    -- make sure there are no other pending nvim_buf_update messages going to
    -- channel 1
    local channel1 = request(1, 'nvim_get_api_info')[1]
    eval('rpcnotify('..channel1..', "Hello")')
    wantn(1, 'Hello', {})

    -- close the buffer and channels 2 and 3 should get a nvim_buf_updates_end
    -- notification
    command('edit')
    wantn(2, 'nvim_buf_updates_end', {b})
    wantn(3, 'nvim_buf_updates_end', {b})

    -- make sure there are no other pending nvim_buf_update messages going to
    -- channel 1
    channel1 = request(1, 'nvim_get_api_info')[1]
    eval('rpcnotify('..channel1..', "Hello Again")')
    wantn(1, 'Hello Again', {})
  end)

  it('works with :diffput and :diffget', function()
    if os.getenv("APPVEYOR") then
      pending("Fails on appveyor for some reason.", function() end)
    end

    local b1, tick1 = editoriginal(true, {"AAA", "BBB"})
    local channel = nvim('get_api_info')[1]
    command('diffthis')
    command('rightbelow vsplit')
    local b2, tick2 = open(true, {"BBB", "CCC"})
    command('diffthis')
    -- go back to first buffer, and push the 'AAA' line to the second buffer
    command('1wincmd w')
    command('normal! gg')
    command('diffput')
    tick2 = tick2 + 1
    expectn('nvim_buf_update', {b2, tick2, 0, 0, {"AAA"}})

    -- use :diffget to grab the other change from buffer 2
    command('normal! G')
    command('diffget')
    tick1 = tick1 + 1
    expectn('nvim_buf_update', {b1, tick1, 2, 2, {"CCC"}})

    eval('rpcnotify('..channel..', "Goodbye")')
    expectn('Goodbye', {})
  end)

  it('works with :sort', function()
    -- test for :sort
    local b, tick = editoriginal(true, {"B", "D", "C", "A", "E"})
    command('%sort')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 5, {"A", "B", "C", "D", "E"}})
  end)

  it('works with :left', function()
    local b, tick = editoriginal(true, {" A", "  B", "B", "\tB", "\t\tC"})
    command('2,4left')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 4, {"B", "B", "B"}})
  end)

  it('works with :right', function()
    local b, tick = editoriginal(true, {" A",
                                        "\t  B",
                                        "\t  \tBB",
                                        " \tB",
                                        "\t\tC"})
    command('set ts=2 et')
    command('2,4retab')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 4, {"    B", "      BB", "  B"}})
  end)

  it('works with :move', function()
    local b, tick = editoriginal(true, origlines)
    -- move text down towards the end of the file
    command('2,3move 4')
    tick = tick + 2
    expectn('nvim_buf_update', {b, tick, 4, 4, {"original line 2",
                                           "original line 3"}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 3, {}})

    -- move text up towards the start of the file
    tick = reopen(b, origlines)
    command('4,5move 2')
    tick = tick + 2
    expectn('nvim_buf_update', {b, tick, 2, 2, {"original line 4",
                                           "original line 5"}})
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 5, 7, {}})
  end)

  it('sends sensible events when you manually add/remove folds', function()
    local b = editoriginal(true)
    local tick = reopenwithfolds(b)

    -- delete the inner fold
    command('normal! zR3Gzd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 1, 4, {'original line 2',
                                           'original line 3',
                                           'original line 4'}})
    -- delete the outer fold
    command('normal! zd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 6, origlines})

    -- discard changes and put the folds back
    tick = reopenwithfolds(b)

    -- remove both folds at once
    command('normal! ggzczD')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 6, origlines})

    -- discard changes and put the folds back
    tick = reopenwithfolds(b)

    -- now delete all folds at once
    command('normal! zE')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 6, origlines})

    -- create a fold from line 4 to the end of the file
    command('normal! 4GA/*{{{*/')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 4, {'original line 4/*{{{*/'}})

    -- delete the fold which only has one marker
    command('normal! Gzd')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 3, 6, {'original line 4',
                                           'original line 5',
                                           'original line 6'}})
  end)

  it('turns off updates when a buffer is closed', function()
    local b, tick = editoriginal(true, {'AAA'})
    local channel = nvim('get_api_info')[1]

    -- test live updates are working
    command('normal! x')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'AA'}})
    command('undo')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'AAA'}})
    tick = tick + 1
    expectn('nvim_buf_changedtick', {b, tick})

    -- close our buffer by creating a new one
    command('enew')
    expectn('nvim_buf_updates_end', {b})

    -- reopen the original buffer, make sure there are no Live Updates sent
    command('b1')
    command('normal! x')

    eval('rpcnotify('..channel..', "Hello There")')
    expectn('Hello There', {})
  end)

  -- test what happens when a buffer is hidden
  it('keeps updates turned on if the buffer is hidden', function()
    local b, tick = editoriginal(true, {'AAA'})
    local channel = nvim('get_api_info')[1]

    -- test live updates are working
    command('normal! x')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'AA'}})
    command('undo')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'AAA'}})
    tick = tick + 1
    expectn('nvim_buf_changedtick', {b, tick})

    -- close our buffer by creating a new one
    command('set hidden')
    command('enew')

    -- note that no nvim_buf_updates_end is sent
    eval('rpcnotify('..channel..', "Hello There")')
    expectn('Hello There', {})

    -- reopen the original buffer, make sure Live Updates are still active
    command('b1')
    command('normal! x')
    tick = tick + 1
    expectn('nvim_buf_update', {b, tick, 0, 1, {'AA'}})
  end)

  it('turns off live updates when a buffer is unloaded, deleted, or wiped',
     function()
    -- start with a blank nvim
    helpers.clear()
    -- need to make a new window with a buffer because :bunload doesn't let you
    -- unload the last buffer
    for _, cmd in ipairs({'bunload', 'bdelete', 'bwipeout'}) do
      command('new')
      -- open a brand spanking new file
      local b = open(true, {'AAA'})

      -- call :bunload or whatever the command is, and then check that we
      -- receive a nvim_buf_updates_end
      command(cmd)
      expectn('nvim_buf_updates_end', {b})
    end
  end)

  it('doesn\'t send the buffer\'s content when not requested', function()
    helpers.clear()
    local b, tick = editoriginal(false)
    ok(buffer('event_sub', b, false))
    expectn('nvim_buf_updates_start', {b, tick, {}, false})
  end)

end)
