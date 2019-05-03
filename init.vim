set rtp+=$PWD
set rtp+=$PWD/../impromptu.nvim

filetype plugin on
"if has("win32")
"  source $PWD\plugin\nvimpam.vim
"else
"  source $PWD/plugin/nvimpam.vim
"endif
  augroup nvimpam
    au!
    autocmd FileType pamcrash nnoremap <F5> :NvimPamUpdateFolds<CR>
    "autocmd FileType pamcrash :NvimPamAttach
    "autocmd FileType pamcrash call luaeval('require("nvimpam").attach()')
  augroup END

function! CellHint()
  return luaeval(
      \ 'require("nvimpam.cellhints").cellhint(_A.pos[2]-1, _A.pos[3]-1)',
      \ { 'pos': getpos('.') }
      \ )
endfunction
set statusline=%{CellHint()}

