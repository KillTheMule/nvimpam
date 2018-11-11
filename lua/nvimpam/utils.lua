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

return {
  locate_binary = locate_binary,
}
