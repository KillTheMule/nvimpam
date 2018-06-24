function! s:checkBufferUpdatesFeature() abort
    if index(map(api_info().functions, 'v:val.name'), 'nvim_buf_attach') >= 0
        call health#report_ok('Function nvim_buf_attach exists!')
    else
        call health#report_error('Function nvim_buf_attach missing!')
    endif
endfunction

function! s:checkBinary() abort
    let l:path = luaeval('require("nvimpam").locate_binary()')
    if executable(l:path) ==# 1
        call health#report_ok('binary found: ' . l:path)
    else
        call health#report_error(
                    \ 'binary is missing or not executable: ' .
                    \ l:path)
    endif
endfunction

function! health#nvimpam#check() abort
    call s:checkBufferUpdatesFeature()
    call s:checkBinary()
endfunction
