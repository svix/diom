let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
argglobal
%argdel
$argadd ~/projects/svix/svix-webhooks/server/svix-server/src/main.rs
$argadd ~/projects/svix/svix-webhooks/server/svix-server/src/worker.rs
edit ~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
wincmd =
argglobal
if bufexists("~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs") | buffer ~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs | else | edit ~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs | endif
if &buftype ==# 'terminal'
  silent file ~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs
endif
balt ~/projects/svix/svix-webhooks/server/svix-server/src/worker.rs
let s:l = 19 - ((18 * winheight(0) + 34) / 68)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 19
normal! 026|
lcd ~/projects/svix/svix-webhooks/server/svix-server/src
wincmd w
argglobal
if bufexists("~/projects/svix/svix-webhooks/server/svix-server/src/error.rs") | buffer ~/projects/svix/svix-webhooks/server/svix-server/src/error.rs | else | edit ~/projects/svix/svix-webhooks/server/svix-server/src/error.rs | endif
if &buftype ==# 'terminal'
  silent file ~/projects/svix/svix-webhooks/server/svix-server/src/error.rs
endif
balt ~/projects/svix/svix-webhooks/server/svix-server/src/core/cache/mod.rs
let s:l = 60 - ((33 * winheight(0) + 34) / 68)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 60
normal! 05|
lcd ~/projects/svix/svix-webhooks/server/svix-server/src
wincmd w
wincmd =
tabnext 1
badd +67 ~/projects/svix/svix-webhooks/server/svix-server/src/main.rs
badd +1 ~/projects/svix/svix-webhooks/server/svix-server/src/worker.rs
badd +98 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/application.rs
badd +46 ~/projects/svix/svix-webhooks/server/svix-server/src/core/security.rs
badd +156 ~/projects/svix/svix-webhooks/server/svix-server/src/core/types.rs
badd +245 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/utils.rs
badd +1 ~/projects/svix/svix-webhooks/server/svix-server/src/test_util.rs
badd +147 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/endpoint/mod.rs
badd +269 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/message.rs
badd +1 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/mod.rs
badd +79 ~/projects/svix/svix-webhooks/server/svix-server/src/cfg.rs
badd +541 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/attempt.rs
badd +75 ~/projects/svix/svix-webhooks/server/svix-server/src/db/models/message.rs
badd +81 ~/projects/svix/svix-webhooks/server/svix-server/src/db/models/messagedestination.rs
badd +53 ~/projects/svix/svix-webhooks/server/svix-server/src/db/models/messageattempt.rs
badd +128 ~/projects/svix/svix-webhooks/server/svix-server/tests/e2e_application.rs
badd +13 ~/projects/svix/svix-webhooks/server/svix-server/Cargo.toml
badd +25 ~/projects/svix/svix-webhooks/server/svix-server/src/lib.rs
badd +230 ~/projects/svix/svix-webhooks/server/svix-server/tests/utils/mod.rs
badd +56 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/endpoint/crud.rs
badd +40 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/endpoint/secrets.rs
badd +6 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/mod.rs
badd +1 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/endpoint
badd +1 ~/projects/svix/svix-webhooks/server/svix-server/tests/e2e_message.rs
badd +252 ~/projects/svix/svix-webhooks/server/svix-server/tests/e2e_endpoint.rs
badd +85 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/endpoint/headers.rs
badd +146 ~/projects/svix/svix-webhooks/server/svix-server/tests/e2e_attempt.rs
badd +56 ~/projects/svix/svix-webhooks/server/svix-server/tests/utils/common_calls.rs
badd +137 ~/projects/svix/svix-webhooks/server/README.md
badd +213 ~/projects/svix/svix-webhooks/server/svix-server/src/v1/endpoints/event_type.rs
badd +61 ~/projects/svix/svix-webhooks/server/svix-server/tests/e2e_event_type.rs
badd +23 ~/projects/svix/svix-webhooks/server/svix-server/src/queue/mod.rs
badd +44 ~/projects/svix/svix-webhooks/server/svix-server/src/core/cache/redis.rs
badd +8 ~/projects/svix/svix-webhooks/server/svix-server/src/core/cache/mod.rs
badd +60 ~/projects/svix/svix-webhooks/server/svix-server/src/error.rs
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=filnxtToOFc
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
