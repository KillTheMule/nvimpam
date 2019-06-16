local curbuf = vim.api.nvim_get_current_buf
local call = vim.api.nvim_call_function

local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobid = require('nvimpam.job').jobid

local function highlight_region(buf, firstline, lastline)
  local id = jobid(buf or curbuf())

  call("rpcnotify", { id, "HighlightRegion", firstline, lastline })
  return true
end

return {
  highlight_region = highlight_region,
}
