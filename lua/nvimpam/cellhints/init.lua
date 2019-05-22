local curbuf = vim.api.nvim_get_current_buf
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function

local nodehints = require('nvimpam.cellhints.2018.node')
local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

local cellhint = { "" }
local cardhints = { }
local hints = nodehints -- need to merge other tables eventually

local function update_cellhint(line, column, buf)
  buf = buf or curbuf()
  
  if not jobids[buf] then
    cellhint[1] = ""
    cardhints = { }
  else
    local card, hint = unpack(call("rpcrequest", { jobids[buf], "CellHint", line, column }))
    if hint == "" then
      cellhint[1] = ""
      cardhints = { }
    else
      card = card:upper()
      cardhints = hints[card]

      if cardhints and cardhints[hint] and cardhints[hint][1] then
        cellhint[1] = hint.." - "..cardhints[hint][1]
      else
        cellhint[1] = hint
      end
    end
  end
end

return {
  cellhint = cellhint,
  update_cellhint = update_cellhint,
  cardhints = cardhints
}
