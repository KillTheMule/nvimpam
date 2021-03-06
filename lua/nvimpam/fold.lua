local curbuf = vim.api.nvim_get_current_buf
local call = vim.api.nvim_call_function
local input = vim.api.nvim_input
local command = vim.api.nvim_command
local get_vvar = vim.api.nvim_get_vvar
local eval = vim.api.nvim_eval

local nvimpam_err = require('nvimpam.job').nvimpam_err
local jobids = require('nvimpam.job').jobids

-- Holds the foldtexts, values of the form {start, end, text}
local foldtexts = {}

local function foldtext()
  local start = get_vvar("foldstart")
  local ende = get_vvar("foldend")

  for _, v in ipairs(foldtexts) do
    if v[1] == start and v[2] == ende then
      return v[3]
    end
  end

  return ""
end

local function printfolds(which)
  which = which or foldtexts
  input("i")
  for _, v in ipairs(which) do
    input(tostring(v[1])..","..tostring(v[2])..": ".. tostring(v[3]).."\n<Escape>")
  end
  input("<Esc>")
end

local function update_folds(texts)
  foldtexts = texts[1]

  local cmd = 'exe "norm! zE"'
  -- The level 1 folds
  for _, v in ipairs(texts[1]) do
    cmd = cmd.."|"..v[1]..","..v[2].."fo" 
  end
  -- The level 2 folds
  for _, v in ipairs(texts[2]) do
    table.insert(foldtexts, v)
    cmd = cmd.."|"..v[1]..","..v[2].."fo" 
  end
  command(cmd)
end

local function refresh_folds(buf)
  buf = buf or curbuf()

  if not jobids[buf] then
    nvimpam_err("Update failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  end

  local newfolds = eval("rpcrequest("..jobids[buf]..", 'RefreshFolds')")
  update_folds(newfolds)
  return true
end

return {
  update_folds = update_folds,
  refresh_folds = refresh_folds,
  foldtext = foldtext,
  printfolds = printfolds,
}
