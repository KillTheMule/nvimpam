if exists('g:nvimpam_loaded')
  finish
endif
let g:nvimpam_loaded = 1

command! NvimPamAttach call luaeval('require("nvimpam").attach()')
command! NvimPamDetach call luaeval('require("nvimpam").detach()')
command! NvimPamUpdateFolds call luaeval('require("nvimpam").update_folds()')

augroup nvimpam
  " clear all previous autocommands
  autocmd!
  autocmd VimLeavePre * call luaeval('require("nvimpam").detach_all()')
augroup end
