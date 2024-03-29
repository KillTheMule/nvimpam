function! s:checkBufferUpdatesFeature() abort
    if index(map(api_info().functions, 'v:val.name'), 'nvim_buf_attach') >= 0
        call v:lua.vim.health.ok('Function nvim_buf_attach exists!')
    else
        call v:lua.vim.health.error(
          \ 'Function nvim_buf_attach missing!',
          \ ['Update your neovim.'])
    endif
endfunction

function! s:checkBinary() abort
    let l:path = luaeval('require("nvimpam").locate_binary()')
    if executable(l:path) ==# 1
        call v:lua.vim.health.ok('binary found: ' . l:path)
    else
        call v:lua.vim.health.error(
          \ 'binary is missing or not executable: ' . l:path,
          \ ['Put the nvimpam binary in your $PATH.'])
    endif
endfunction

function! s:checkImpromptu() abort
    let l:imp = luaeval('pcall(require, "impromptu") and true or false')
    if l:imp
        call v:lua.vim.health.ok('`Vigemus/impromptu.nvim` is installed')
    else
        call v:lua.vim.health.error(
          \ '`Vigemus/impromptu.nvim` is not installed',
          \ ['Visit `https://github.com/Vigemus/impromptu.nvim`.'])
    endif
endfunction

function! s:checkPamcards() abort
    let l:pamcards = finddir('lua/nvimpam/pam_cards', &rtp)
    if strlen(l:pamcards) > 0
        call v:lua.vim.health.ok('Directory `pam_cards` found')
    else
        call v:lua.vim.health.error(
          \ 'Directory `pam_cards` not found',
          \ ['This should be a subdirectory of `lua/nvimpam` of the nvimpam',
          \  'installation. Try to reinstall the plugin.'])
    endif
endfunction

function! health#nvimpam#check() abort
    call v:lua.vim.health.start("Buffer updates")
    call s:checkBufferUpdatesFeature()
    call v:lua.vim.health.start("Nvimpam binary")
    call s:checkBinary()
    call v:lua.vim.health.start("Menu availability")
    call s:checkImpromptu()
    call s:checkPamcards()
endfunction

