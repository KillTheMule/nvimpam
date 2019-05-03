" We're not setting this ourselves, because the file needs to be loaded for
" each new buffer. It's just here to give the user the ability to disable it.
if exists('g:nvimpam_loaded')
  finish
endif

command -buffer NvimPamAttach call luaeval(
      \ 'require("nvimpam").attach(_A.f)',
      \ { 'f': expand('%:p') }
      \ )
command -buffer NvimPamUpdateFolds call luaeval('require("nvimpam").refresh_folds()')
command -buffer NvimPamHighlightScreen call luaeval(
      \ 'require("nvimpam").highlight_region(_A.b, _A.f, _A.l)',
      \ { 'b': bufnr('%'), 'f': line('w0')-1, 'l': line('w$')-1 }
      \ )
command -buffer NvimPamMenu call luaeval('require("nvimpam.cardmenu").cardmenu()')

augroup nvimpam_leave
  " clear all previous autocommands
  autocmd!
  autocmd VimLeavePre * call luaeval('require("nvimpam").detach_all()')
augroup end

function! Nvimpam_foldtext()
  return luaeval('require("nvimpam").foldtext()')
endfunction

let s:save_foldtext = &foldtext
setlocal foldtext=Nvimpam_foldtext()

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

if !exists('b:undo_ftplugin')
  let b:undo_ftplugin = ''
endif

let b:undo_ftplugin .= '|setlocal foldtext='.s:save_foldtext
      \ . '|delcommand NvimPamAttach'
      \ . '|delcommand NvimPamUpdateFolds'
      \ . '|delcommand NvimPamHighlightScreen'
      \ . '|delcommand NvimPamMenu'
