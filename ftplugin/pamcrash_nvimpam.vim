if exists('g:nvimpam_loaded')
  finish
endif
let g:nvimpam_loaded = 1

command! NvimPamAttach call luaeval('require("nvimpam").attach()')
command! NvimPamDetach call luaeval('require("nvimpam").detach()')
command! NvimPamUpdateFolds call luaeval('require("nvimpam").update_folds()')
command! NvimPamPrintfolds call luaeval('require("nvimpam").printfolds()')

augroup nvimpam_leave
  " clear all previous autocommands
  autocmd!
  autocmd VimLeavePre * call luaeval('require("nvimpam").detach_all()')
augroup end

function! Nvimpam_foldtext()
  return luaeval('require("nvimpam").foldtext()')
endfunction

set foldtext=Nvimpam_foldtext()
