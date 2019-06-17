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

local function align_line(line) 
  id = jobid(buf or curbuf())
  
  local aligned = call("rpcrequest", { id, "AlignLine", line })

  if aligned then
    buf_set_lines(buf, line, line + 1, true, { aligned })
  end
end

local function align_card(line)
  local buf = buf or curbuf()
  id = jobid(buf)

  local cardrange = call("rpcrequest", { id, "CardRange", line })

  if cardrange[1] and cardrange[2] then
    for i = cardrange[1], cardrange[2] do
      local aligned = call("rpcrequest", { id, "AlignLine", i })

      if aligned then
        buf_set_lines(buf, i, i + 1, true, { aligned })
      end
    end
  else
    command("echom 'range: "..tostring(cardrange[1])..":"..tostring(cardrange[2]).."'")
  end
end

return {
  align_line = align_line,
  align_card = align_card,
}
