if exists('g:nvimpam_loaded')
  finish
endif
let g:nvimpam_loaded = 1

command! NvimPamAttach call luaeval(
      \ 'require("nvimpam").attach(_A.f)',
      \ { 'f': expand('%:p') }
      \ )
command! NvimPamDetach call luaeval('require("nvimpam").detach()')
command! NvimPamUpdateFolds call luaeval('require("nvimpam").refresh_folds()')
command! NvimPamPrintfolds call luaeval('require("nvimpam").printfolds()')
command! NvimPamPrintstderr call luaeval('require("nvimpam").printstderr()')
command! NvimPamHighlightScreen call luaeval(
      \ 'require("nvimpam").highlight_region(_A.b, _A.f, _A.l)',
      \ { 'b': bufnr('%'), 'f': line('w0')-1, 'l': line('w$')-1 }
      \ )
command! NvimPamMenu call luaeval('require("nvimpam.cardmenu").cardmenu()')

augroup nvimpam_leave
  " clear all previous autocommands
  autocmd!
  autocmd VimLeavePre * call luaeval('require("nvimpam").detach_all()')
augroup end

function! Nvimpam_foldtext()
  return luaeval('require("nvimpam").foldtext()')
endfunction

set foldtext=Nvimpam_foldtext()

if &background == "dark"
  highlight default PamCellEven ctermbg=229 guibg=#ffffcf
  highlight default PamCellOdd ctermbg=254 guibg=#e4e4e4
  highlight default PamErrorCellEven ctermfg=15 ctermbg=124 guifg=#ffffff guibg=#af0000
  highlight default PamErrorCellOdd ctermfg=15 ctermbg=9 guifg=#ffffff guibg=#ff0000
  highlight default PamKeyword cterm=bold ctermfg=94 gui=bold guifg=#875f00
else
  highlight default PamCellEven ctermbg=229 guibg=#ffffcf
  highlight default PamCellOdd ctermbg=254 guibg=#e4e4e4
  highlight default PamErrorCellEven ctermfg=15 ctermbg=124 guifg=#ffffff guibg=#af0000
  highlight default PamErrorCellOdd ctermfg=15 ctermbg=9 guifg=#ffffff guibg=#ff0000
  highlight default PamKeyword cterm=bold ctermfg=94 gui=bold guifg=#875f00
endif
