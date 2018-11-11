local curbuf = vim.api.nvim_get_current_buf
local call = vim.api.nvim_call_function

local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

local function highlight_region(buf, firstline, lastline)
  buf = buf or curbuf()

  if not jobids[buf] then
    nvimpam_err("highlight_region failed: No jobid entry for buffer "
                ..tostring(buf).."!")
    return false
  end

  call("rpcnotify", { jobids[buf], "HighlightRegion", firstline, lastline })
  return true
end

return {
  highlight_region = highlight_region,
}
