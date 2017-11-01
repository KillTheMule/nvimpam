if ! exists('s:jobid')
  let s:jobid = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/debug/nvimpam'

function! nvimpam#init()
  call nvimpam#connect()
endfunction

function! nvimpam#connect()
  let result = s:StartJob()

  if 0 == result
    echoerr "Nvimpam: cannot start rpc process"
  elseif -1 == result
    echoerr "Nvimpam: rpc process is not executable"
  else
    let s:jobid = result
    call s:ConfigureJob(result)
  endif
endfunction

function! nvimpam#reset()
  let s:jobid = 0
endfunction

function! s:ConfigureJob(jobid)
  augroup nvimPam
    " clear all previous autocommands
    autocmd!

    autocmd VimLeavePre * :call s:StopJob()

    "autocmd InsertChange * :call s:NotifyInsertChange()
    "autocmd InsertEnter * :call s:NotifyInsertEnter()
    "autocmd InsertLeave * :call s:NotifyInsertLeave()

    "autocmd CursorMovedI * :call s:NotifyCursorMovedI()
  augroup END
endfunction

"function! s:NotifyCursorMovedI()
"  let [ bufnum, lnum, column, off ] = getpos('.')
"  call rpcnotify(s:jobid, 'cursor-moved-i', lnum, column)
"endfunction

"function! s:NotifyInsertChange()
"  let [ bufnum, lnum, column, off ] = getpos('.')
"  call rpcnotify(s:jobid, 'insert-change', v:insertmode, lnum, column)
"endfunction
"
"function! s:NotifyInsertEnter()
"  let [ bufnum, lnum, column, off ] = getpos('.')
"  call rpcnotify(s:jobid, 'insert-enter', v:insertmode, lnum, column)
"endfunction
"
"function! s:NotifyInsertLeave()
"  call rpcnotify(s:jobid, 'insert-leave')
"endfunction

function! s:OnStderr(id, data, event) dict
  redir >> nvimpam.stderr
  echo '' . join(a:data, "")
  echo "\n"
  redir END
endfunction

function! s:StartJob()
  if 0 == s:jobid
    let id = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return id
  else
    return 0
  endif
endfunction

function! s:StopJob()
  if 0 < s:jobid
    augroup nvimPam
      autocmd!    " clear all previous autocommands
    augroup END

    call rpcnotify(s:jobid, 'quit')
    let result = jobwait(s:jobid, 500)

    if -1 == result
      " kill the job
      call jobstop(s:jobid)
    endif

    " reset job id back to zero
    let s:jobid = 0
  endif
endfunction
