local curbuf = vim.api.nvim_get_current_buf
local eval = vim.api.nvim_eval
local call = vim.api.nvim_call_function

local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

local cellhint = { "xbc" }

local function update_cellhint(line, column, buf)
  buf = buf or curbuf()
  
  if not jobids[buf] then
    cellhint[1] = ""
  else
    cellhint[1] = call("rpcrequest", { jobids[buf], "CellHint", line, column })
  end
end

return {
  cellhint = cellhint,
  update_cellhint = update_cellhint,
}
