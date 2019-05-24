local curbuf = vim.api.nvim_get_current_buf
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function
local create_buf = vim.api.nvim_create_buf
local buf_set_lines = vim.api.nvim_buf_set_lines
local open_win = vim.api.nvim_open_win

local nodehints = require('nvimpam.cellhints.2018.node')
local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

local hints = nodehints -- need to merge other tables eventually

-- first is the Parameter name, second a slightly longer description
local cellhint = { "", "" }
local cardhints = { {} }

local function celldoc()
  local buf = create_buf(false, true)
  local doc = { }
  if cellhint[1] == cellhint[1]:upper() then
    for _, v in ipairs(cardhints[1]) do
      if v[1] and v[1] == cellhint[1] then
        doc = v
      end
    end
  else
    for _, v in ipairs(cardhints[1]) do
      table.insert(doc, v[1]..":")
      for i=2,#v do
        table.insert(doc, v[i])
      end
      table.insert(doc, "-----------------------------")
    end
    table.remove(doc)
  end

  buf_set_lines(buf, 0, -1, true, doc)
  local opts = { relative = "win", width = 35, height = #doc, col = 80,
                 row = 0, anchor = "NE" }

  local win = open_win(buf, true, opts)
end

local function update_cellhint(line, column, buf)
  buf = buf or curbuf()
  
  if not jobids[buf] then
    cellhint[1] = ""
    cellhint[2] = ""
    cardhints[1] = { }
  else
    local card, hint = unpack(call("rpcrequest", { jobids[buf], "CellHint", line, column }))
    if hint == "" then
      cellhint[1] = ""
      cellhint[2] = ""
      cardhints[1] = { }
    else
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

return {
  cellhint = cellhint,
  update_cellhint = update_cellhint,
  cardhints = cardhints,
  celldoc = celldoc,
  parameter = parameter,
}
