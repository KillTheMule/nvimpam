if exists('g:nvimpam_loaded')
  finish
endif
let g:nvimpam_loaded = 1

command! NvimPamAttach call luaeval('require("nvimpam").attach(_A.f)', { 'f': expand('%:p') })
"'.expand('%:p').'")')
command! NvimPamDetach call luaeval('require("nvimpam").detach()')
command! NvimPamUpdateFolds call luaeval('require("nvimpam").refresh_folds()')
command! NvimPamPrintfolds call luaeval('require("nvimpam").printfolds()')
command! NvimPamPrintstderr call luaeval('require("nvimpam").printstderr()')
command! NvimPamHighlightScreen call luaeval('require("nvimpam").highlight_region('.(line('w0')-1).', '.(line('w$')-1).')')

augroup nvimpam_leave
  " clear all previous autocommands
  autocmd!
  autocmd VimLeavePre * call luaeval('require("nvimpam").detach_all()')
augroup end

function! Nvimpam_foldtext()
  return luaeval('require("nvimpam").foldtext()')
endfunction

set foldtext=Nvimpam_foldtext()
