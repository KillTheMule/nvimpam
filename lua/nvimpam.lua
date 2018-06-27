local command = vim.api.nvim_command
local call = vim.api.nvim_call_function
local curbuf = vim.api.nvim_get_current_buf
local get_vvar = vim.api.nvim_get_vvar
local input = vim.api.nvim_input

-- Holds buffer -> jobid associations
local jobids = {}

-- Holds the foldtexts, values of the form {start, end, text}
local foldtexts = {}

-- TODO: Must this be so ugly?
local function locate_binary()
  local locations = { "nvimpam", "target/release/nvimpam",
                      "target/debug/nvimpam" }

  local tmp = {}

  for _, path in ipairs(locations) do
    table.insert(tmp, "../"..path)
    local path2 = path:gsub("/","\\")..".exe"
    table.insert(tmp, path2)
    table.insert(tmp, "..\\"..path2)
  end

  for _, path in ipairs(tmp) do
    table.insert(locations, path)
  end

  for _, path in ipairs(locations) do
    if call("executable", { path }) == 1 then
      return path
    elseif call("executable", { "../"..path }) == 1 then
      return "../"..path
    end
  end

  return nil
end

local function nv_err(msg)
  command("echoerr '"..msg.."'")
end

local function attach()
  local buf = curbuf()

  if  jobids[buf] then
    nv_err("Attach failed: Nvimpam already attached to buffer "
           ..tostring(buf).."!")
    return false
  end

  local binary = locate_binary()
  if not binary then
    nv_err("Attach failed: No executable found!")
    return nil
  end

  local jobid = call("jobstart", { binary, { rpc=true } })

  if jobid == 0 then
    nv_err("Attach failed: Invalid args to jobstart on buffer "
            .. tostring(buf) .. "!")
  elseif jobid == -1 then
    nv_err("Attach failed: Command "..binary.."not executable!")
  else
    jobids[buf] = jobid
    return true
  end
end

local function detach(buf)
  buf = buf or curbuf()

  if not jobids[buf] then
    nv_err("Detach failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  end

  call("rpcnotify", { jobids[buf], "quit" })
  jobids[buf] = nil
end

local function detach_all()
  for buf, _ in pairs(jobids) do
    detach(buf)
  end
end

local function update_folds(buf)
  buf = buf or curbuf()

  if not jobids[buf] then
    nv_err("Update failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  end

  call("rpcnotify", { jobids[buf], "RefreshFolds" })
end

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

local function update_foldtexts(texts)
  foldtexts = texts
end

return {
  attach = attach,
  detach = detach,
  detach_all = detach_all,
  update_folds = update_folds,
  locate_binary = locate_binary,
  update_foldtexts = update_foldtexts,
  foldtext = foldtext,
  foldtexts = foldtexts,
  printfolds = printfolds
}
