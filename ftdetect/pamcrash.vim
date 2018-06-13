augroup pamcrash_ft
  au!
  au BufRead,BufNewFile *.inc :filetype plugin on | set filetype=pamcrash
  au BufRead,BufNewFile *.pc :filetype plugin on | set filetype=pamcrash 
  au BufRead,BufNewFile *.mat :filetype plugin on | set filetype=pamcrash 
  au BufRead,BufNewFile *.metric :filetype plugin on | set filetype=pamcrash 
augroup END
