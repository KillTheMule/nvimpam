local curbuf = vim.api.nvim_get_current_buf
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function
local create_buf = vim.api.nvim_create_buf
local buf_set_lines = vim.api.nvim_buf_set_lines
local open_win = vim.api.nvim_open_win
local command = vim.api.nvim_command
local buf_get_lines = vim.api.nvim_buf_get_lines
local buf_set_lines = vim.api.nvim_buf_set_lines
local input = vim.api.nvim_input

local nodehints = require('nvimpam.cellhints.2018.node')
local constrainthints = require('nvimpam.cellhints.2018.constraints')
local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobid = require('nvimpam.job').jobid

local hints = {}

for k, v in pairs(nodehints) do
  if hints[k] then
    error("Card already in hints table: "..k)
  end
  hints[k] = v
end
for k, v in pairs(constrainthints) do
  if hints[k] then
    error("Card already in hints table: "..k)
  end
  hints[k] = v
end

-- first is the Parameter name, second a slightly longer description
local cellhint = { "", "" }
local cardhints = { {} }

local function celldoc()
  local buf = create_buf(false, true)
  local doc = { }
  if cellhint[1] == "Keyword" then
    for _, v in ipairs(cardhints[1]) do
      table.insert(doc, v[1]..":")
      for i=2,#v do
        for line in vim.gsplit(v[i], "\n") do
          table.insert(doc, line)
        end
      end
      table.insert(doc, "-----------------------------")
    end
    table.remove(doc)
  else
    for _, v in ipairs(cardhints[1]) do
      if v[1] and v[1] == cellhint[1] then
        for _, s in ipairs(v) do
          for line in vim.gsplit(s, "\n") do
            table.insert(doc, line)
          end
        end
      end
    end
  end

  buf_set_lines(buf, 0, -1, true, doc)
  local opts = { relative = "win", width = 35, height = #doc, col = 80,
                 row = 0, anchor = "NE" }

  local win = open_win(buf, true, opts)
end

local function update_cellhint(line, column, buf)
  -- this might be called without the plugin loaded, so protect us from errors
  local ok, id = pcall(jobid, buf or curbuf())
  
  if not ok then
    cellhint[1] = ""
    cellhint[2] = ""
    cardhints[1] = { }
  else
    local new_hint = call("rpcrequest", { id, "CellHint", line, column })
    if not new_hint then
      cellhint[1] = ""
      cellhint[2] = ""
      cardhints[1] = { }
    else
      local card, hint = unpack(new_hint)
      card = card:upper()
      cardhints[1] = hints[card]
      cellhint[1] = hint
      cellhint[2] = hint
      for _, v in ipairs(cardhints[1]) do
        if v[1] and v[1] == hint then
          cellhint[2] = hint.. " - "..v[2]
        end
      end
    end
  end
end

local function add_linecomment(line) 
  local id = jobid(buf or curbuf())
  
  local linecomment = call("rpcrequest", { id, "CommentLine", line })

  if linecomment then
    local curline = buf_get_lines(buf, line, line + 1, true)[1]

    if not curline then
      error("Could not get current line")
    end

    buf_set_lines(buf, line, line + 1, true, {linecomment, curline})
  else
    command("echom 'No comment for line "..tostring(line + 1).."'")
  end
end

local function add_cardcomments(line)
  local id = jobid(buf or curbuf())

  local cardrange = call("rpcrequest", { id, "CardRange", line })

  if not cardrange then
    command("echom 'No card for line "..tostring(line + 1).."'")
    return
  end

  local newlines = {}
  local oldlines = buf_get_lines(buf, cardrange[1], cardrange[2] + 1, true)

  for i, l in ipairs(oldlines) do
    local firstchar = l:sub(1,1)

    if firstchar ~= "#" and firstchar ~= "$" then
      local linecomment = call("rpcrequest", { id, "CommentLine", cardrange[1] + i - 1 })

      if linecomment then 
        table.insert(newlines, linecomment)
      end
    end

    table.insert(newlines, l)
  end

  buf_set_lines(buf, cardrange[1], cardrange[2] + 1, true, newlines)
end

local function select_card(line)
  local id = jobid(buf or curbuf())

  local cardrange = call("rpcrequest", { id, "CardRange", line })

  if cardrange then
    input("\\<Esc>V"..tostring(cardrange[2] + 1).."Go"..tostring(cardrange[1] + 1).."G")
  else
    command("echom 'Could not find card containing the current line'")
  end


end

return {
  cellhint = cellhint,
  update_cellhint = update_cellhint,
  cardhints = cardhints,
  celldoc = celldoc,
  parameter = parameter,
  add_linecomment = add_linecomment,
  add_cardcomments = add_cardcomments,
  select_card = select_card,
}
