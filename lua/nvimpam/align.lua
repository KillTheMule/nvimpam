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
local jobids = require('nvimpam.job').jobids




local function align_line(line) 
  buf = buf or curbuf()
  
  if not jobids[buf] then
    error("No job entry for buffer "..tostring(buf))
    return
  end

  local aligned = call("rpcrequest", { jobids[buf], "AlignLine", line })

  if aligned then
    buf_set_lines(buf, line, line + 1, true, { aligned })
  end
end



return {
  align_line = align_line,
}
