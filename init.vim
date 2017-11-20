set rtp+=$PWD

if has("win32")
  source $PWD\plugin\nvimpam.vim
else
  source $PWD/plugin/nvimpam.vim
endif
