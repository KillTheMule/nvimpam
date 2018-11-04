local command = vim.api.nvim_command
local call = vim.api.nvim_call_function
local curbuf = vim.api.nvim_get_current_buf
local get_vvar = vim.api.nvim_get_vvar
local input = vim.api.nvim_input

-- Holds the binary to start
local binary = nil

-- Holds buffer -> jobid associations
local jobids = {}

-- Holds the foldtexts, values of the form {start, end, text}
local foldtexts = {}

-- Holds nvimpam stderr output
local stderr = {}
-- Saves the value of NVIMPAM_STDERR
local stderr_file

-- Tracks if we've defined the callback functions already
local callbacks_defined = { }

-- TODO: Must this be so ugly?
local function locate_binary()
  local locations = { "nvimpam", "./target/release/nvimpam",
                      "./target/debug/nvimpam" }

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

local function on_stderr(id, data, event)
  if not stderr[id] then stderr[id] = {} end

  for i, s in ipairs(data) do
    if i == 1 and stderr[id][#stderr[id]] then
      stderr[id][#stderr[id]] = stderr[id][#stderr[id]] .. s
    elseif s ~= "" then
      table.insert(stderr[id], s)
    end
  end
end

local function nvimpam_err(msg, id)
  command("echoerr '"..msg.."'")

  id = id or "NONE" 
  on_stderr(id, {{msg}})
end

local function on_exit(id, exitcode)
  local bufname

  for buffer, jobid in pairs(jobids) do
    if jobid == id then
      bufname = buffer
    end
  end
  
  if bufname then
    jobids[bufname] = nil
  end
end

local function attach(filename)
  local buf = curbuf()

  if jobids[buf] then
    nvimpam_err("Attach failed: Nvimpam already attached to buffer "
                ..tostring(buf).."!")
    return false
  end

  if binary == nil then
    binary = locate_binary()
  end

  if not binary then
    nvimpam_err("Attach to buffer "..tostring(buf).." failed: No "
                .."executable found!")
    return false
  end

  if filename == nil then
    args = ""
  else
    args = filename
  end

  if not callbacks_defined["onexit"] then
    command([[
      function Nvimpam_onexit(id, exitcode, event) 
         let func = "require(\"nvimpam\").on_exit(_A.i, _A.e)"
         let args = "{'i':a:id, 'e':a:exitcode}"
         execute "call luaeval('" . func . "'," . args . ")"
      endfunction
    ]])
    callbacks_defined["onexit"] = true
  end

  stderr_file = os.getenv("NVIMPAM_STDERR")
  local jobid

  if stderr_file ~= nil then 
    if not callbacks_defined["onstderr"] then
      command([[
        function Nvimpam_onstderr(id, data, event) 
           let func = "require(\"nvimpam\").on_stderr(_A.i, _A.d, _A.e)"
           let args = "{'i':a:id, 'd':a:data, 'e':a:event}"
           execute "call luaeval('" . func . "'," . args . ")"
        endfunction
      ]])
      callbacks_defined["onstderr"] = true
    end

    jobid = call("jobstart", {
      { binary, args },
      { rpc=true, on_stderr='Nvimpam_onstderr', on_exit='Nvimpam_onexit'}
    })
  else
    jobid = call("jobstart", {
      { binary, args },
      { rpc=true, on_exit='Nvimpam_onexit'}
    })
  end

  if jobid == 0 then
    nvimpam_err("Attach failed: Invalid args to jobstart on buffer "
                .. tostring(buf) .. "!")
    return false
  elseif jobid == -1 then
    nvimpam_err("Attach on buffer "..tostring(buf).." failed: Command \""
                ..binary.."\" not executable!")
    return false
  else
    jobids[buf] = jobid
    return true
  end
end

local function detach(buf)
  buf = buf or curbuf()
  local jobid = jobids[buf]

  if not jobid then
    nvimpam_err("Detach failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  else
    call("rpcnotify", { jobids[buf], "quit" })
    return true
  end
end

local function detach_all()
  for buf, jobid in pairs(jobids) do
    detach(buf)
  end

  if stderr_file ~= nil then
    -- check if stderr file is writeable
    local f, msg = io.open(stderr_file, "w")

    if f == nil then
      nvimpam_err("Could not open $NVIMPAM_STDERR(='"..stderr_file.."') "
                  .."for writing: "..msg)
    else
      local written = false

      for i, t in pairs(stderr) do
        if #t > 0 then 
          for _, l in ipairs(t) do
            f:write("Channel "..tostring(i)..": "..l..'\n')
            written = true
          end
          io.close(f)
        end
      end

      if not written then
        os.remove(stderr_file)
      end
    end

  end
end

local function refresh_folds(buf)
  buf = buf or curbuf()

  if not jobids[buf] then
    nvimpam_err("Update failed: No jobid entry for buffer "..tostring(buf).."!")
    return false
  end

  call("rpcnotify", { jobids[buf], "RefreshFolds" })
  return true
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

local function update_folds(texts)
  foldtexts = texts

  local cmd = 'exe "norm! zE"'
  for _, v in ipairs(foldtexts) do
    cmd = cmd.."|"..v[1]..","..v[2].."fo" 
  end
  command(cmd)
end

local function highlight_region(firstline, lastline)
  buf = buf or curbuf()

  if not jobids[buf] then
    nvimpam_err("highlight_region failed: No jobid entry for buffer "
                ..tostring(buf).."!")
    return false
  end

  call("rpcnotify", { jobids[buf], "HighlightRegion", firstline, lastline })
  return true
end

local function printstderr()
  input("i")
  for i, t in pairs(stderr) do
    input("Jobid " .. tostring(i))
    for j, s in ipairs(t) do
      input("String #"..tostring(j).." is '"..tostring(s).."'\n")
    end
  end
end

return {
  attach = attach,
  detach = detach,
  detach_all = detach_all,
  update_folds = update_folds,
  locate_binary = locate_binary,
  refresh_folds = refresh_folds,
  foldtext = foldtext,
  foldtexts = foldtexts,
  printfolds = printfolds,
  on_stderr = on_stderr,
  on_exit = on_exit,
  printstderr = printstderr,
  highlight_region = highlight_region,
}
