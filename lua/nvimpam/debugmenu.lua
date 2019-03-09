local curbuf = vim.api.nvim_get_current_buf
local command = vim.api.nvim_command

local imp_status, impromptu = pcall(require, "impromptu")

local function debugmenu()
  if not imp_status then
    command("echoerr 'Impromptu not installed, can not show menu!'")
    return nil
  end

  local curbuf = curbuf()

  local opts = {
    printfolds = {
      description = "Print folds",
    },
    printstderr = {
      description = "Print stderr",
    },
    detach = {
      description = "Detach nvimpam",
    },
  }

  impromptu.ask{
    options = opts,
    question = "Debug Menu",
    handler = function(b, opt)
      if opt.description == "Print folds" then
        require("nvimpam.fold").printfolds()
      elseif opt.description == "Print stderr" then
        require("nvimpam.job").printstderr()
      else
        require("nvimpam").detach(curbuf)
      end
      return true
    end
  }     
end

return {
  debugmenu = debugmenu
}
