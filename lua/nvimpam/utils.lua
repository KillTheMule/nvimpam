local call = vim.api.nvim_call_function

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

-- http://lua-users.org/wiki/FileInputOutput

-- see if the file exists
local function file_exists(file)
  local f = io.open(file, "rb")
  if f then f:close() end
  return f ~= nil
end

-- get all lines from a file, returns an empty 
-- list/table if the file does not exist
local function lines_from_file(file)
  if not file_exists(file) then return {} end
  lines = {}
  for line in io.lines(file) do 
    lines[#lines + 1] = line
  end
  return lines
end

return {
  locate_binary = locate_binary,
  lines_from_file = lines_from_file,
}
