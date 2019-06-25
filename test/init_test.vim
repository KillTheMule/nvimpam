"let s:localpath = expand('<sfile>:h') 
"
"exe 'set rtp+='.s:localpath
"exe 'set rtp+='.s:localpath.'/../impromptu.nvim'
"
""if has("win32")
""  source $PWD\plugin\nvimpam.vim
""else
""  source $PWD/plugin/nvimpam.vim
""endif
"  augroup nvimpam
"    au!
"    autocmd FileType pamcrash nnoremap <F5> :NvimPamUpdateFolds<CR>
"    "autocmd FileType pamcrash :NvimPamAttach
"    "autocmd FileType pamcrash call luaeval('require("nvimpam").attach()')
"  augroup END

filetype plugin on

function! CellHint()
  return luaeval('require("nvimpam.cellhints").cellhint[2]')
endfunction

set statusline=%{CellHint()}

