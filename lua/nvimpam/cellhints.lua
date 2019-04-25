local curbuf = vim.api.nvim_get_current_buf
local eval = vim.api.nvim_eval

local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

local function cellhint(line, column, buf)
  buf = buf or curbuf()

  if not jobids[buf] then
    nvimpam_err("Update failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  end

  return eval("rpcrequest("..
    jobids[buf]..","..
    "'CellHint',"..
    tostring(line)..","..
    tostring(column)
  ..")")
end

return {
  cellhint = cellhint,
}
