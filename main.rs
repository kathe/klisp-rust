#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]

use libc::{strlen, memcpy, write, exit, close, read, signal, tolower, memcmp, strcmp, open, creat};

pub type size_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub const MHALT: C2RustUnnamed_0 = 0;
pub const MEXPR: C2RustUnnamed_0 = 1;
pub const MPROG: C2RustUnnamed_0 = 9;
pub const MSETQ: C2RustUnnamed_0 = 8;
pub const MRETN: C2RustUnnamed_0 = 4;
pub const MAPPL: C2RustUnnamed_0 = 5;
pub const MNOTP: C2RustUnnamed_0 = 7;
pub const MPRED: C2RustUnnamed_0 = 6;
pub const MBETA: C2RustUnnamed_0 = 3;
pub const MLIST: C2RustUnnamed_0 = 2;
pub type C2RustUnnamed_0 = libc::c_uint;
#[no_mangle]
pub static mut Tag: [libc::c_uchar; 8192] = [0; 8192];
#[no_mangle]
pub static mut Car: [libc::c_short; 8192] = [0; 8192];
#[no_mangle]
pub static mut Cdr: [libc::c_short; 8192] = [0; 8192];
#[no_mangle]
pub static mut Freelist: libc::c_int = 0;
#[no_mangle]
pub static mut Acc: libc::c_int = 0;
#[no_mangle]
pub static mut Env: libc::c_int = 0;
#[no_mangle]
pub static mut Stack: libc::c_int = 0;
#[no_mangle]
pub static mut Mstack: libc::c_int = 0;
#[no_mangle]
pub static mut Tmp: libc::c_int = 0;
#[no_mangle]
pub static mut Tmpcar: libc::c_int = 0;
#[no_mangle]
pub static mut Tmpcdr: libc::c_int = 0;
#[no_mangle]
pub static mut Input: libc::c_int = 0;
#[no_mangle]
pub static mut Inp: libc::c_int = 0;
#[no_mangle]
pub static mut Ink: libc::c_int = 0;
#[no_mangle]
pub static mut Inbuf: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut Buffer: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut Rejected: libc::c_int = 0;
#[no_mangle]
pub static mut Output: libc::c_int = 0;
#[no_mangle]
pub static mut Outp: libc::c_int = 0;
#[no_mangle]
pub static mut Outbuf: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut Symbols: libc::c_int = 0;
#[no_mangle]
pub static mut Id: libc::c_int = 0;
#[no_mangle]
pub static mut Restart: jmp_buf =
    [__jmp_buf_tag{__jmpbuf: [0; 8],
                   __mask_was_saved: 0,
                   __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
#[no_mangle]
pub static mut Error: libc::c_int = 0;
#[no_mangle]
pub static mut Parens: libc::c_int = 0;
#[no_mangle]
pub static mut Loads: libc::c_int = 0;
#[no_mangle]
pub static mut Verbose_GC: libc::c_int = 0;
#[no_mangle]
pub static mut S_apply: libc::c_int = 0;
#[no_mangle]
pub static mut S_if: libc::c_int = 0;
#[no_mangle]
pub static mut S_ifnot: libc::c_int = 0;
#[no_mangle]
pub static mut S_lambda: libc::c_int = 0;
#[no_mangle]
pub static mut S_lamstar: libc::c_int = 0;
#[no_mangle]
pub static mut S_macro: libc::c_int = 0;
#[no_mangle]
pub static mut S_prog: libc::c_int = 0;
#[no_mangle]
pub static mut S_quote: libc::c_int = 0;
#[no_mangle]
pub static mut S_qquote: libc::c_int = 0;
#[no_mangle]
pub static mut S_unquote: libc::c_int = 0;
#[no_mangle]
pub static mut S_splice: libc::c_int = 0;
#[no_mangle]
pub static mut S_setq: libc::c_int = 0;
#[no_mangle]
pub static mut S_t: libc::c_int = 0;
#[no_mangle]
pub static mut S_cons: libc::c_int = 0;
#[no_mangle]
pub static mut S_car: libc::c_int = 0;
#[no_mangle]
pub static mut S_cdr: libc::c_int = 0;
#[no_mangle]
pub static mut S_atom: libc::c_int = 0;
#[no_mangle]
pub static mut S_eq: libc::c_int = 0;
#[no_mangle]
pub static mut S_gensym: libc::c_int = 0;
#[no_mangle]
pub static mut S_it: libc::c_int = 0;
#[no_mangle]
pub static mut S_suspend: libc::c_int = 0;
#[no_mangle]
pub static mut S_gc: libc::c_int = 0;
#[no_mangle]
pub static mut S_eofp: libc::c_int = 0;
#[no_mangle]
pub static mut S_load: libc::c_int = 0;
#[no_mangle]
pub static mut S_setcar: libc::c_int = 0;
#[no_mangle]
pub static mut S_setcdr: libc::c_int = 0;
#[no_mangle]
pub static mut S_read: libc::c_int = 0;
#[no_mangle]
pub static mut S_prin: libc::c_int = 0;
#[no_mangle]
pub static mut S_prin1: libc::c_int = 0;
#[no_mangle]
pub static mut S_print: libc::c_int = 0;
#[no_mangle]
pub static mut S_error: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn car(mut x: libc::c_int) -> libc::c_int {
    return Car[x as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdr(mut x: libc::c_int) -> libc::c_int {
    return Cdr[x as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setcar(mut x: libc::c_int, mut v: libc::c_int) {
    Car[x as usize] = v as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn setcdr(mut x: libc::c_int, mut v: libc::c_int) {
    Cdr[x as usize] = v as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn atomp(mut n: libc::c_int) -> libc::c_int {
    return (n >= 8192 as libc::c_int ||
                car(n) < 8192 as libc::c_int &&
                    Tag[car(n) as usize] as libc::c_int & 0x1 as libc::c_int
                        != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn symbolp(mut n: libc::c_int) -> libc::c_int {
    return (n < 8192 as libc::c_int && car(n) < 8192 as libc::c_int &&
                Tag[car(n) as usize] as libc::c_int & 0x1 as libc::c_int != 0)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn caar(mut x: libc::c_int) -> libc::c_int {
    return Car[Car[x as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cadr(mut x: libc::c_int) -> libc::c_int {
    return Car[Cdr[x as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdar(mut x: libc::c_int) -> libc::c_int {
    return Cdr[Car[x as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cddr(mut x: libc::c_int) -> libc::c_int {
    return Cdr[Cdr[x as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn caadr(mut x: libc::c_int) -> libc::c_int {
    return Car[Car[Cdr[x as usize] as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cadar(mut x: libc::c_int) -> libc::c_int {
    return Car[Cdr[Car[x as usize] as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn caddr(mut x: libc::c_int) -> libc::c_int {
    return Car[Cdr[Cdr[x as usize] as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdadr(mut x: libc::c_int) -> libc::c_int {
    return Cdr[Car[Cdr[x as usize] as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cddar(mut x: libc::c_int) -> libc::c_int {
    return Cdr[Cdr[Car[x as usize] as usize] as usize] as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn caddar(mut x: libc::c_int) -> libc::c_int {
    return Car[Cdr[Cdr[Car[x as usize] as usize] as usize] as usize] as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cdddar(mut x: libc::c_int) -> libc::c_int {
    return Cdr[Cdr[Cdr[Car[x as usize] as usize] as usize] as usize] as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn pr(mut s: *mut libc::c_char) {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    k = strlen(s) as libc::c_int;
    while Outp + k >= 128 as libc::c_int {
        if Outp > 0 as libc::c_int {
            n = 128 as libc::c_int - Outp;
            memcpy(Outbuf.as_mut_ptr().offset(Outp as isize) as
                       *mut libc::c_void, s as *const libc::c_void,
                   n.try_into().unwrap());
            write(Output, Outbuf.as_mut_ptr() as *const libc::c_void,
                  128 as libc::c_int as size_t);
            k -= n;
            s = s.offset(n as isize);
            Outp = 0 as libc::c_int
        } else {
            write(Output, s as *const libc::c_void,
                  128 as libc::c_int as size_t);
            k -= 128 as libc::c_int;
            s = s.offset(128 as libc::c_int as isize)
        }
    }
    memcpy(Outbuf.as_mut_ptr().offset(Outp as isize) as *mut libc::c_void,
           s as *const libc::c_void, k.try_into().unwrap());
    Outp += k;
}
#[no_mangle]
pub unsafe extern "C" fn flush() {
    write(Output, Outbuf.as_mut_ptr() as *const libc::c_void, Outp as size_t);
    Outp = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn nl() {
    pr(b"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    flush();
}
#[no_mangle]
pub unsafe extern "C" fn ntoa(mut n: libc::c_int) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 20] = [0; 20];
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    q =
        &mut *buf.as_mut_ptr().offset((::std::mem::size_of::<[libc::c_char; 20]>()
                                           as
                                           libc::c_ulong).wrapping_sub(1 as
                                                                           libc::c_int
                                                                           as
                                                                           libc::c_ulong)
                                          as isize) as *mut libc::c_char;
    p = q;
    *p = 0 as libc::c_int as libc::c_char;
    while n != 0 || p == q {
        p = p.offset(-1);
        *p = (n % 10 as libc::c_int + '0' as i32) as libc::c_char;
        n = n / 10 as libc::c_int
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn prnum(mut n: libc::c_int) { pr(ntoa(n)); }
#[no_mangle]
pub unsafe extern "C" fn error(mut m: *mut libc::c_char, mut n: libc::c_int) {
    pr(b"? \x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    pr(m);
    if n != 8192 as libc::c_int + 1 as libc::c_int {
        pr(b": \x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
        print(n);
    }
    nl();
    Input = 0 as libc::c_int;
    Inbuf = Buffer.as_mut_ptr();
    Ink = 0 as libc::c_int;
    Inp = Ink;
    Rejected = 8192 as libc::c_int + 4 as libc::c_int;
    //longjmp(Restart.as_mut_ptr(), 1 as libc::c_int);
    panic!("error");
}
#[no_mangle]
pub unsafe extern "C" fn fatal(mut m: *mut libc::c_char) {
    error(m, 8192 as libc::c_int + 1 as libc::c_int);
    pr(b"? aborting\x00" as *const u8 as *const libc::c_char as
           *mut libc::c_char);
    nl();
    exit(1 as libc::c_int);
}
/* Deutsch/Schorr/Waite graph marker */
#[no_mangle]
pub unsafe extern "C" fn mark(mut n: libc::c_int) {
    let mut p: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    p = 8192 as libc::c_int + 5 as libc::c_int;
    loop  {
        if n >= 8192 as libc::c_int ||
               Tag[n as usize] as libc::c_int & 0x2 as libc::c_int != 0 {
            if 8192 as libc::c_int + 5 as libc::c_int == p { break ; }
            if Tag[p as usize] as libc::c_int & 0x4 as libc::c_int != 0 {
                x = cdr(p);
                setcdr(p, car(p));
                setcar(p, n);
                Tag[p as usize] =
                    (Tag[p as usize] as libc::c_int & !(0x4 as libc::c_int))
                        as libc::c_uchar;
                n = x
            } else { x = p; p = cdr(x); setcdr(x, n); n = x }
        } else if Tag[n as usize] as libc::c_int & 0x1 as libc::c_int != 0 {
            x = cdr(n);
            setcdr(n, p);
            p = n;
            n = x;
            Tag[p as usize] =
                (Tag[p as usize] as libc::c_int | 0x2 as libc::c_int) as
                    libc::c_uchar
        } else {
            x = car(n);
            setcar(n, p);
            Tag[n as usize] =
                (Tag[n as usize] as libc::c_int | 0x2 as libc::c_int) as
                    libc::c_uchar;
            p = n;
            n = x;
            Tag[p as usize] =
                (Tag[p as usize] as libc::c_int | 0x4 as libc::c_int) as
                    libc::c_uchar
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gc(mut v: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    mark(Acc);
    mark(Env);
    mark(Symbols);
    mark(Stack);
    mark(Mstack);
    mark(Tmpcar);
    mark(Tmpcdr);
    mark(Tmp);
    Freelist = 8192 as libc::c_int + 5 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8192 as libc::c_int {
        if 0 as libc::c_int ==
               Tag[i as usize] as libc::c_int & 0x2 as libc::c_int {
            setcdr(i, Freelist);
            Freelist = i;
            k += 1
        } else {
            Tag[i as usize] =
                (Tag[i as usize] as libc::c_int & !(0x2 as libc::c_int)) as
                    libc::c_uchar
        }
        i += 1
    }
    if v != 0 || Verbose_GC != 0 {
        prnum(k);
        pr(b" nodes reclaimed\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
        nl();
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn cons3(mut a: libc::c_int, mut d: libc::c_int,
                               mut t: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    if 8192 as libc::c_int + 5 as libc::c_int == Freelist {
        Tmpcdr = d;
        if 0 as libc::c_int == t { Tmpcar = a }
        gc(0 as libc::c_int);
        Tmpcdr = 8192 as libc::c_int + 5 as libc::c_int;
        Tmpcar = Tmpcdr;
        if 8192 as libc::c_int + 5 as libc::c_int == Freelist {
            error(b"out of nodes\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  8192 as libc::c_int + 1 as libc::c_int);
        }
    }
    n = Freelist;
    Freelist = cdr(Freelist);
    setcar(n, a);
    setcdr(n, d);
    Tag[n as usize] = t as libc::c_uchar;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn cons(mut a: libc::c_int, mut d: libc::c_int)
 -> libc::c_int {
    return cons3(a, d, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn nrev(mut n: libc::c_int) -> libc::c_int {
    let mut m: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    m = 8192 as libc::c_int + 5 as libc::c_int;
    while n != 8192 as libc::c_int + 5 as libc::c_int {
        h = cdr(n);
        setcdr(n, m);
        m = n;
        n = h
    }
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn save(mut n: libc::c_int) { Stack = cons(n, Stack); }
#[no_mangle]
pub unsafe extern "C" fn unsave(mut k: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    while k != 0 {
        if 8192 as libc::c_int + 5 as libc::c_int == Stack {
            fatal(b"stack empty\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
        }
        n = car(Stack);
        Stack = cdr(Stack);
        k -= 1
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn msave(mut n: libc::c_int) {
    Mstack = cons3(n, Mstack, 0x1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn munsave() -> libc::c_int {
    let mut n: libc::c_int = 0;
    if 8192 as libc::c_int + 5 as libc::c_int == Mstack {
        fatal(b"mstack empty\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char);
    }
    n = car(Mstack);
    Mstack = cdr(Mstack);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn strsym(mut s: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    i = 0 as libc::c_int;
    if 0 as libc::c_int == *s.offset(i as isize) as libc::c_int {
        return 8192 as libc::c_int + 5 as libc::c_int
    }
    n =
        cons3(8192 as libc::c_int + 5 as libc::c_int,
              8192 as libc::c_int + 5 as libc::c_int, 0x1 as libc::c_int);
    save(n);
    loop  {
        setcar(n,
               (*s.offset(i as isize) as libc::c_int) << 8 as libc::c_int |
                   *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int);
        if 0 as libc::c_int ==
               *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int ||
               0 as libc::c_int ==
                   *s.offset((i + 2 as libc::c_int) as isize) as libc::c_int {
            break ;
        }
        setcdr(n,
               cons3(8192 as libc::c_int + 5 as libc::c_int,
                     8192 as libc::c_int + 5 as libc::c_int,
                     0x1 as libc::c_int));
        n = cdr(n);
        i += 2 as libc::c_int
    }
    n = unsave(1 as libc::c_int);
    return cons(n, 8192 as libc::c_int + 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn symstr(mut n: libc::c_int) -> *mut libc::c_char {
    static mut b: [libc::c_char; 66] = [0; 66];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    n = car(n);
    while n != 8192 as libc::c_int + 5 as libc::c_int {
        b[i as usize] = (car(n) >> 8 as libc::c_int) as libc::c_char;
        b[(i + 1 as libc::c_int) as usize] =
            (car(n) & 0xff as libc::c_int) as libc::c_char;
        i += 2 as libc::c_int;
        n = cdr(n)
    }
    b[i as usize] = 0 as libc::c_int as libc::c_char;
    return b.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn findsym(mut s: *mut libc::c_char) -> libc::c_int {
    let mut p: libc::c_int = 0;
    p = Symbols;
    while p != 8192 as libc::c_int + 5 as libc::c_int {
        if strcmp(s, symstr(car(p))) == 0 as libc::c_int { return car(p) }
        p = cdr(p)
    }
    return 8192 as libc::c_int + 5 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addsym(mut s: *mut libc::c_char, mut v: libc::c_int)
 -> libc::c_int {
    let mut n: libc::c_int = 0;
    n = findsym(s);
    if n != 8192 as libc::c_int + 5 as libc::c_int { return n }
    n = strsym(s);
    save(n);
    if 8192 as libc::c_int == v {
        setcdr(n, cons(n, 8192 as libc::c_int + 5 as libc::c_int));
    } else { setcdr(n, cons(v, 8192 as libc::c_int + 5 as libc::c_int)); }
    Symbols = cons(n, Symbols);
    unsave(1 as libc::c_int);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn rdch() -> libc::c_int {
    let mut c: libc::c_int = 0;
    if Rejected != 8192 as libc::c_int + 4 as libc::c_int {
        c = Rejected;
        Rejected = 8192 as libc::c_int + 4 as libc::c_int;
        return c
    }
    if Inp >= Ink {
        Ink =
            read(Input, Inbuf as *mut libc::c_void,
                 128 as libc::c_int as size_t) as libc::c_int;
        if Ink < 1 as libc::c_int {
            return 8192 as libc::c_int + 4 as libc::c_int
        }
        Inp = 0 as libc::c_int
    }
    let fresh0 = Inp;
    Inp = Inp + 1;
    return *Inbuf.offset(fresh0 as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdchci() -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = rdch();
    if c >= 8192 as libc::c_int { return c }
    return tolower(c);
}
#[no_mangle]
pub unsafe extern "C" fn rdlist() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut lst: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut badpair: *mut libc::c_char = 0 as *mut libc::c_char;
    badpair =
        b"bad pair\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    Parens += 1;
    lst =
        cons(8192 as libc::c_int + 5 as libc::c_int,
             8192 as libc::c_int + 5 as libc::c_int);
    save(lst);
    a = 8192 as libc::c_int + 5 as libc::c_int;
    count = 0 as libc::c_int;
    loop  {
        if Error != 0 { return 8192 as libc::c_int + 5 as libc::c_int }
        n = xread();
        if 8192 as libc::c_int + 4 as libc::c_int == n {
            error(b"missing \')\'\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  8192 as libc::c_int + 1 as libc::c_int);
        }
        if 8192 as libc::c_int + 3 as libc::c_int == n {
            if count < 1 as libc::c_int {
                error(badpair, 8192 as libc::c_int + 1 as libc::c_int);
            }
            n = xread();
            setcdr(a, n);
            if 8192 as libc::c_int + 2 as libc::c_int == n ||
                   xread() != 8192 as libc::c_int + 2 as libc::c_int {
                error(badpair, 8192 as libc::c_int + 1 as libc::c_int);
            }
            unsave(1 as libc::c_int);
            Parens -= 1;
            return lst
        }
        if 8192 as libc::c_int + 2 as libc::c_int == n { break ; }
        if 8192 as libc::c_int + 5 as libc::c_int == a {
            a = lst
        } else { a = cdr(a) }
        setcar(a, n);
        setcdr(a,
               cons(8192 as libc::c_int + 5 as libc::c_int,
                    8192 as libc::c_int + 5 as libc::c_int));
        count += 1
    }
    Parens -= 1;
    if a != 8192 as libc::c_int + 5 as libc::c_int {
        setcdr(a, 8192 as libc::c_int + 5 as libc::c_int);
    }
    unsave(1 as libc::c_int);
    return if count != 0 {
               lst
           } else { (8192 as libc::c_int) + 5 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn symbolic(mut c: libc::c_int) -> libc::c_int {
    ((c as u8).is_ascii_alphabetic() || 
    (c as u8).is_ascii_digit() ||
    ((c as u8) == b'-') ||
    ((c as u8) == b'/'))
    as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn rdsym(mut c: libc::c_int) -> libc::c_int {
    let mut s: [libc::c_char; 65] = [0; 65];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while symbolic(c) != 0 {
        if '/' as i32 == c { c = rdchci() }
        if 64 as libc::c_int == i {
            error(b"long symbol\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  8192 as libc::c_int + 1 as libc::c_int);
        } else if i < 64 as libc::c_int {
            s[i as usize] = c as libc::c_char;
            i += 1
        }
        c = rdchci()
    }
    s[i as usize] = 0 as libc::c_int as libc::c_char;
    Rejected = c;
    if strcmp(s.as_mut_ptr(), b"nil\x00" as *const u8 as *const libc::c_char)
           == 0 {
        return 8192 as libc::c_int + 5 as libc::c_int
    }
    return addsym(s.as_mut_ptr(), 8192 as libc::c_int + 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn syntax(mut x: libc::c_int) {
    error(b"syntax\x00" as *const u8 as *const libc::c_char as
              *mut libc::c_char, x);
}
#[no_mangle]
pub unsafe extern "C" fn quote(mut q: libc::c_int, mut n: libc::c_int)
 -> libc::c_int {
    return cons(q, cons(n, 8192 as libc::c_int + 5 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn rdword() -> libc::c_int {
    let mut s: [libc::c_char; 2] = [0; 2];
    let mut n: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    s[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    c = rdchci();
    if symbolic(c) == 0 { syntax(8192 as libc::c_int + 1 as libc::c_int); }
    n =
        cons(8192 as libc::c_int + 5 as libc::c_int,
             8192 as libc::c_int + 5 as libc::c_int);
    save(n);
    loop  {
        if '/' as i32 == c { c = rdchci() }
        s[0 as libc::c_int as usize] = c as libc::c_char;
        setcar(n,
               addsym(s.as_mut_ptr(),
                      8192 as libc::c_int + 1 as libc::c_int));
        c = rdchci();
        if !(symbolic(c) != 0) { break ; }
        setcdr(n,
               cons(8192 as libc::c_int + 5 as libc::c_int,
                    8192 as libc::c_int + 5 as libc::c_int));
        n = cdr(n)
    }
    Rejected = c;
    n = unsave(1 as libc::c_int);
    return quote(S_quote, n);
}
#[export_name = "type"]
pub unsafe extern "C" fn type_0(mut x: libc::c_int) {
    error(b"type\x00" as *const u8 as *const libc::c_char as
              *mut libc::c_char, x);
}
#[no_mangle]
pub unsafe extern "C" fn xread() -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = rdchci();
    loop  {
        while ' ' as i32 == c || '\t' as i32 == c || '\n' as i32 == c ||
                  '\r' as i32 == c {
            if Error != 0 { return 8192 as libc::c_int + 5 as libc::c_int }
            c = rdchci()
        }
        if c != ';' as i32 { break ; }
        while c != '\n' as i32 { c = rdchci() }
    }
    if 8192 as libc::c_int + 4 as libc::c_int == c || '%' as i32 == c {
        return 8192 as libc::c_int + 4 as libc::c_int
    }
    if '(' as i32 == c {
        return rdlist()
    } else if '\'' as i32 == c {
        return quote(S_quote, xread())
    } else if '@' as i32 == c {
        return quote(S_qquote, xread())
    } else if ',' as i32 == c {
        c = rdchci();
        if '@' as i32 == c { return quote(S_splice, xread()) }
        Rejected = c;
        return quote(S_unquote, xread())
    } else if ')' as i32 == c {
        if Parens == 0 {
            error(b"extra paren\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  8192 as libc::c_int + 1 as libc::c_int);
        }
        return 8192 as libc::c_int + 2 as libc::c_int
    } else if '.' as i32 == c {
        if Parens == 0 {
            error(b"free dot\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char,
                  8192 as libc::c_int + 1 as libc::c_int);
        }
        return 8192 as libc::c_int + 3 as libc::c_int
    } else if symbolic(c) != 0 {
        return rdsym(c)
    } else if '#' as i32 == c {
        return rdword()
    } else {
        syntax(8192 as libc::c_int + 1 as libc::c_int);
        return 8192 as libc::c_int + 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn print2(mut n: libc::c_int, mut d: libc::c_int) {
    if d > 128 as libc::c_int {
        error(b"print depth\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    }
    if Error != 0 {
        error(b"stop\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    }
    if 8192 as libc::c_int + 5 as libc::c_int == n {
        pr(b"nil\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    } else if 8192 as libc::c_int + 4 as libc::c_int == n {
        pr(b"*eot*\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    } else if n >= 8192 as libc::c_int ||
                  Tag[n as usize] as libc::c_int & 0x1 as libc::c_int != 0 {
        pr(b"*unprintable*\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char);
    } else if symbolp(n) != 0 {
        pr(symstr(n));
    } else {
        /* List */
        if atomp(n) == 0 && S_lamstar == car(n) {
            pr(b"*closure*\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char);
            return
        }
        pr(b"(\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        while n != 8192 as libc::c_int + 5 as libc::c_int {
            print2(car(n), d + 1 as libc::c_int);
            n = cdr(n);
            if symbolp(n) != 0 {
                pr(b" . \x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
                print2(n, d + 1 as libc::c_int);
                n = 8192 as libc::c_int + 5 as libc::c_int
            } else if n != 8192 as libc::c_int + 5 as libc::c_int {
                pr(b" \x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
            }
        }
        pr(b")\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn print(mut n: libc::c_int) {
    print2(n, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookup(mut x: libc::c_int) -> libc::c_int {
    let mut a: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    e = Env;
    while e != 8192 as libc::c_int + 5 as libc::c_int {
        a = car(e);
        while a != 8192 as libc::c_int + 5 as libc::c_int {
            if caar(a) == x { return cdar(a) }
            a = cdr(a)
        }
        e = cdr(e)
    }
    return cdr(x);
}
#[no_mangle]
pub unsafe extern "C" fn specialp(mut n: libc::c_int) -> libc::c_int {
    return (n == S_quote || n == S_if || n == S_prog || n == S_ifnot ||
                n == S_lambda || n == S_lamstar || n == S_apply || n == S_setq
                || n == S_macro) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn check(mut x: libc::c_int, mut k0: libc::c_int,
                               mut kn: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    i = 0 as libc::c_int;
    a = x;
    while atomp(a) == 0 { i += 1; a = cdr(a) }
    if a != 8192 as libc::c_int + 5 as libc::c_int || i < k0 ||
           kn != -(1 as libc::c_int) && i > kn {
        syntax(x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn load(mut s: *mut libc::c_char) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut ib: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ip: libc::c_int = 0;
    let mut ik: libc::c_int = 0;
    let mut in_0: libc::c_int = 0;
    let mut re: libc::c_int = 0;
    if Loads >= 2 as libc::c_int {
        error(b"nested load\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    }
    in_0 = Input;
    ip = Inp;
    ik = Ink;
    ib = Inbuf;
    re = Rejected;
    Inbuf = buf.as_mut_ptr();
    Ink = 0 as libc::c_int;
    Inp = Ink;
    Rejected = 8192 as libc::c_int + 4 as libc::c_int;
    Input = open(s, 0 as libc::c_int);
    if Input < 0 as libc::c_int {
        error(b"load\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, strsym(s));
    }
    Loads += 1;
    loop  {
        Acc = xread();
        if 8192 as libc::c_int + 4 as libc::c_int == Acc { break ; }
        eval(Acc);
    }
    Loads -= 1;
    Rejected = re;
    Inbuf = ib;
    Ink = ik;
    Inp = ip;
    Input = in_0;
}
#[no_mangle]
pub unsafe extern "C" fn dowrite(mut fd: libc::c_int,
                                 mut b: *mut libc::c_void,
                                 mut k: libc::c_int) {
    if write(fd, b, k as size_t) != k as libc::c_long {
        error(b"write error\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn suspend(mut s: *mut libc::c_char) {
    let mut fd: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 128] = [0; 128];
    fd = creat(s, 0o644);
    if fd < 0 as libc::c_int {
        error(b"suspend\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, strsym(s));
    }
    memcpy(buf.as_mut_ptr() as *mut libc::c_void,
           b"KL21\x00" as *const u8 as *const libc::c_char as
               *const libc::c_void,
           strlen(b"KL21\x00" as *const u8 as
                      *const libc::c_char).wrapping_add(1
                                                            ));
    k =
        strlen(b"KL21\x00" as *const u8 as
                   *const libc::c_char).wrapping_add(1
                                                         ) as
            libc::c_int;
    buf[k as usize] =
        (8192 as libc::c_int >> 8 as libc::c_int) as libc::c_uchar;
    buf[(k + 1 as libc::c_int) as usize] =
        8192 as libc::c_int as libc::c_uchar;
    buf[(k + 2 as libc::c_int) as usize] =
        (Freelist >> 8 as libc::c_int) as libc::c_uchar;
    buf[(k + 3 as libc::c_int) as usize] = Freelist as libc::c_uchar;
    buf[(k + 4 as libc::c_int) as usize] =
        (Symbols >> 8 as libc::c_int) as libc::c_uchar;
    buf[(k + 5 as libc::c_int) as usize] = Symbols as libc::c_uchar;
    buf[(k + 6 as libc::c_int) as usize] =
        (Id >> 8 as libc::c_int) as libc::c_uchar;
    buf[(k + 7 as libc::c_int) as usize] = Id as libc::c_uchar;
    dowrite(fd, buf.as_mut_ptr() as *mut libc::c_void, k + 8 as libc::c_int);
    dowrite(fd, Car.as_mut_ptr() as *mut libc::c_void,
            (8192 as libc::c_int as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                 as libc::c_ulong) as
                libc::c_int);
    dowrite(fd, Cdr.as_mut_ptr() as *mut libc::c_void,
            (8192 as libc::c_int as
                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                 as libc::c_ulong) as
                libc::c_int);
    dowrite(fd, Tag.as_mut_ptr() as *mut libc::c_void, 8192 as libc::c_int);
    close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn doread(mut fd: libc::c_int, mut b: *mut libc::c_void,
                                mut k: libc::c_int) {
    if read(fd, b, k as size_t) != k as libc::c_long {
        error(b"read error\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fasload(mut s: *mut libc::c_char) {
    let mut fd: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 128] = [0; 128];
    let mut badimg: *mut libc::c_char = 0 as *mut libc::c_char;
    badimg =
        b"bad image\x00" as *const u8 as *const libc::c_char as
            *mut libc::c_char;
    fd = open(s, 0 as libc::c_int);
    if fd < 0 as libc::c_int { return }
    k =
        strlen(b"KL21\x00" as *const u8 as
                   *const libc::c_char).wrapping_add(1
                                                         ) as
            libc::c_int;
    doread(fd, buf.as_mut_ptr() as *mut libc::c_void, k + 8 as libc::c_int);
    n =
        (buf[k as usize] as libc::c_int) << 8 as libc::c_int |
            buf[(k + 1 as libc::c_int) as usize] as libc::c_int;
    Freelist =
        (buf[(k + 2 as libc::c_int) as usize] as libc::c_int) <<
            8 as libc::c_int |
            buf[(k + 3 as libc::c_int) as usize] as libc::c_int;
    Symbols =
        (buf[(k + 4 as libc::c_int) as usize] as libc::c_int) <<
            8 as libc::c_int |
            buf[(k + 5 as libc::c_int) as usize] as libc::c_int;
    Id =
        (buf[(k + 6 as libc::c_int) as usize] as libc::c_int) <<
            8 as libc::c_int |
            buf[(k + 7 as libc::c_int) as usize] as libc::c_int;
    if n != 8192 as libc::c_int ||
           memcmp(buf.as_mut_ptr() as *const libc::c_void,
                  b"KL21\x00" as *const u8 as *const libc::c_char as
                      *const libc::c_void, k.try_into().unwrap()) !=
               0 as libc::c_int {
        error(badimg, 8192 as libc::c_int + 1 as libc::c_int);
    }
    doread(fd, Car.as_mut_ptr() as *mut libc::c_void,
           (8192 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                as libc::c_ulong) as
               libc::c_int);
    doread(fd, Cdr.as_mut_ptr() as *mut libc::c_void,
           (8192 as libc::c_int as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_short>()
                                                as libc::c_ulong) as
               libc::c_int);
    doread(fd, Tag.as_mut_ptr() as *mut libc::c_void, 8192 as libc::c_int);
    if read(fd, buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t) != 0 as libc::c_int as libc::c_long {
        error(badimg, 8192 as libc::c_int + 1 as libc::c_int);
    }
    close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn builtin(mut x: libc::c_int) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if S_car == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        if atomp(cadr(x)) != 0 { type_0(x); }
        return caadr(x)
    } else if S_cdr == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        if atomp(cadr(x)) != 0 { type_0(x); }
        return cdadr(x)
    } else if S_eq == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        return if cadr(x) == caddr(x) {
                   S_t
               } else { (8192 as libc::c_int) + 5 as libc::c_int }
    } else if S_atom == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        return if atomp(cadr(x)) != 0 {
                   S_t
               } else { (8192 as libc::c_int) + 5 as libc::c_int }
    } else if S_cons == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        return cons(cadr(x), caddr(x))
    } else if S_setcar == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        if atomp(cadr(x)) != 0 { type_0(x); }
        setcar(cadr(x), caddr(x));
        return cadr(x)
    } else if S_setcdr == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        if atomp(cadr(x)) != 0 { type_0(x); }
        setcdr(cadr(x), caddr(x));
        return cadr(x)
    } else if S_gensym == car(x) {
        check(x, 1 as libc::c_int, 1 as libc::c_int);
        Id += 1;
        s = ntoa(Id);
        s = s.offset(-1);
        *s = 'G' as i32 as libc::c_char;
        return addsym(s, 8192 as libc::c_int + 1 as libc::c_int)
    } else if S_eofp == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        return if 8192 as libc::c_int + 4 as libc::c_int == cadr(x) {
                   S_t
               } else { (8192 as libc::c_int) + 5 as libc::c_int }
    } else if S_read == car(x) {
        check(x, 1 as libc::c_int, 1 as libc::c_int);
        return xread()
    } else if S_prin == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        print(cadr(x));
        pr(b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return cadr(x)
    } else if S_prin1 == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        print(cadr(x));
        return cadr(x)
    } else if S_print == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        print(cadr(x));
        nl();
        return cadr(x)
    } else if S_load == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        if symbolp(cadr(x)) == 0 { type_0(x); }
        load(symstr(cadr(x)));
        return S_t
    } else if S_error == car(x) {
        check(x, 2 as libc::c_int, 3 as libc::c_int);
        if symbolp(cadr(x)) == 0 { type_0(x); }
        if 8192 as libc::c_int + 5 as libc::c_int == cddr(x) {
            error(symstr(cadr(x)), 8192 as libc::c_int + 1 as libc::c_int);
        } else { error(symstr(cadr(x)), caddr(x)); }
        return 8192 as libc::c_int + 1 as libc::c_int
    } else if S_gc == car(x) {
        check(x, 1 as libc::c_int, 2 as libc::c_int);
        if cdr(x) != 8192 as libc::c_int + 5 as libc::c_int {
            Verbose_GC =
                (cadr(x) != 8192 as libc::c_int + 5 as libc::c_int) as
                    libc::c_int
        }
        gc(1 as libc::c_int);
        return 8192 as libc::c_int + 5 as libc::c_int
    } else if S_suspend == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        if symbolp(cadr(x)) == 0 { type_0(x); }
        suspend(symstr(cadr(x)));
        return S_t
    } else { syntax(x); return 8192 as libc::c_int + 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn cklam(mut x: libc::c_int) {
    let mut p: libc::c_int = 0;
    check(x, 3 as libc::c_int, -(1 as libc::c_int));
    p = cadr(x);
    while atomp(p) == 0 { if symbolp(car(p)) == 0 { syntax(x); } p = cdr(p) };
}
#[no_mangle]
pub unsafe extern "C" fn special(mut x: libc::c_int, mut pm: *mut libc::c_int)
 -> libc::c_int {
    if S_quote == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        *pm = munsave();
        return cadr(x)
    } else {
        if S_if == car(x) {
            check(x, 4 as libc::c_int, 4 as libc::c_int);
            msave(MPRED as libc::c_int);
            *pm = MEXPR as libc::c_int;
            save(cddr(x));
            return cadr(x)
        } else {
            if S_prog == car(x) {
                *pm = MEXPR as libc::c_int;
                if 8192 as libc::c_int + 5 as libc::c_int == cdr(x) {
                    return 8192 as libc::c_int + 5 as libc::c_int
                }
                if 8192 as libc::c_int + 5 as libc::c_int == cddr(x) {
                    return cadr(x)
                }
                msave(MPROG as libc::c_int);
                save(cddr(x));
                return cadr(x)
            }
        }
    }
    if S_ifnot == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        msave(MNOTP as libc::c_int);
        *pm = MEXPR as libc::c_int;
        save(caddr(x));
        return cadr(x)
    } else if S_lambda == car(x) {
        cklam(x);
        *pm = munsave();
        return cons(S_lamstar, cons(Env, cdr(x)))
    } else if S_lamstar == car(x) {
        check(x, 3 as libc::c_int, -(1 as libc::c_int));
        *pm = munsave();
        return x
    } else if S_apply == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        msave(MAPPL as libc::c_int);
        *pm = MEXPR as libc::c_int;
        save(caddr(x));
        save(8192 as libc::c_int + 5 as libc::c_int);
        return cadr(x)
    } else if S_macro == car(x) {
        check(x, 2 as libc::c_int, 2 as libc::c_int);
        if atomp(cadr(x)) != 0 || caadr(x) != S_lambda { syntax(x); }
        cklam(cadr(x));
        *pm = munsave();
        return cons(S_macro,
                    cons(cons(S_lamstar, cons(Env, cdadr(x))),
                         8192 as libc::c_int + 5 as libc::c_int))
    } else if S_setq == car(x) {
        check(x, 3 as libc::c_int, 3 as libc::c_int);
        if symbolp(cadr(x)) == 0 { syntax(x); }
        msave(MSETQ as libc::c_int);
        *pm = MEXPR as libc::c_int;
        save(cadr(x));
        return caddr(x)
    } else { syntax(x); return 8192 as libc::c_int + 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn bindargs(mut v: libc::c_int, mut a: libc::c_int) {
    let mut e: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    e = 8192 as libc::c_int + 5 as libc::c_int;
    save(e);
    while atomp(v) == 0 {
        if 8192 as libc::c_int + 5 as libc::c_int == a {
            error(b"too few args\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, Acc);
        }
        n =
            cons(car(v),
                 cons(car(a), 8192 as libc::c_int + 5 as libc::c_int));
        e = cons(n, e);
        setcar(Stack, e);
        v = cdr(v);
        a = cdr(a)
    }
    if symbolp(v) != 0 {
        n = cons(v, cons(a, 8192 as libc::c_int + 5 as libc::c_int));
        e = cons(n, e);
        setcar(Stack, e);
    } else if a != 8192 as libc::c_int + 5 as libc::c_int {
        error(b"extra args\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, Acc);
    }
    Env = cons(e, Env);
    unsave(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn funapp(mut x: libc::c_int) -> libc::c_int {
    Acc = x;
    if atomp(car(x)) != 0 || caar(x) != S_lamstar { syntax(x); }
    if Mstack != 8192 as libc::c_int + 5 as libc::c_int &&
           MRETN as libc::c_int == car(Mstack) {
        Env = cadar(Acc);
        bindargs(caddar(Acc), cdr(Acc));
    } else {
        save(Env);
        Env = cadar(Acc);
        bindargs(caddar(Acc), cdr(Acc));
        msave(MRETN as libc::c_int);
    }
    return cons(S_prog, cdddar(x));
}
#[no_mangle]
pub unsafe extern "C" fn expand(mut x: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    if atomp(x) != 0 { return x }
    if S_quote == car(x) { return x }
    n =
        if symbolp(car(x)) != 0 {
            car(lookup(car(x)))
        } else { (8192 as libc::c_int) + 1 as libc::c_int };
    if atomp(n) == 0 && car(n) == S_macro {
        m = cons(cdr(x), 8192 as libc::c_int + 5 as libc::c_int);
        m = cons(S_quote, m);
        m = cons(m, 8192 as libc::c_int + 5 as libc::c_int);
        m = cons(cadr(n), m);
        m = cons(S_apply, m);
        save(m);
        n = eval(m);
        setcar(Stack, n);
        n = expand(n);
        unsave(1 as libc::c_int);
        return n
    }
    p = x;
    while atomp(p) == 0 { p = cdr(p) }
    if symbolp(p) != 0 { return x }
    save(x);
    n = 8192 as libc::c_int + 5 as libc::c_int;
    save(n);
    p = x;
    while p != 8192 as libc::c_int + 5 as libc::c_int {
        m = expand(car(p));
        n = cons(m, n);
        setcar(Stack, n);
        p = cdr(p)
    }
    n = nrev(unsave(1 as libc::c_int));
    unsave(1 as libc::c_int);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn eval(mut x: libc::c_int) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    Acc = expand(x);
    msave(MHALT as libc::c_int);
    m = MEXPR as libc::c_int;
    while 0 as libc::c_int == Error {
        match m {
            1 => {
                if symbolp(Acc) != 0 {
                    n = car(lookup(Acc));
                    if 8192 as libc::c_int + 1 as libc::c_int == n {
                        error(b"undefined\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              Acc);
                    }
                    Acc = n;
                    m = munsave()
                } else if atomp(Acc) != 0 {
                    m = munsave()
                } else if specialp(car(Acc)) != 0 {
                    m = MBETA as libc::c_int
                } else {
                    save(cdr(Acc));
                    Acc = car(Acc);
                    save(8192 as libc::c_int + 5 as libc::c_int);
                    msave(MLIST as libc::c_int);
                }
            }
            2 => {
                if 8192 as libc::c_int + 5 as libc::c_int == cadr(Stack) {
                    Acc = nrev(cons(Acc, unsave(1 as libc::c_int)));
                    unsave(1 as libc::c_int);
                    m = MBETA as libc::c_int
                } else {
                    setcar(Stack, cons(Acc, car(Stack)));
                    Acc = caadr(Stack);
                    setcar(cdr(Stack), cdadr(Stack));
                    msave(m);
                    m = MEXPR as libc::c_int
                }
            }
            5 => {
                if 8192 as libc::c_int + 5 as libc::c_int == car(Stack) {
                    setcar(Stack, Acc);
                    Acc = cadr(Stack);
                    msave(MAPPL as libc::c_int);
                    m = MEXPR as libc::c_int
                } else {
                    n = unsave(1 as libc::c_int);
                    unsave(1 as libc::c_int);
                    Acc = cons(n, Acc);
                    m = MBETA as libc::c_int
                }
            }
            6 => {
                n = unsave(1 as libc::c_int);
                if 8192 as libc::c_int + 5 as libc::c_int == Acc {
                    Acc = cadr(n)
                } else { Acc = car(n) }
                m = MEXPR as libc::c_int
            }
            7 => {
                n = unsave(1 as libc::c_int);
                if 8192 as libc::c_int + 5 as libc::c_int == Acc {
                    Acc = n;
                    m = MEXPR as libc::c_int
                } else { m = munsave() }
            }
            3 => {
                if specialp(car(Acc)) != 0 {
                    Acc = special(Acc, &mut m)
                } else if symbolp(car(Acc)) != 0 {
                    Acc = builtin(Acc);
                    m = munsave()
                } else { Acc = funapp(Acc); m = MEXPR as libc::c_int }
            }
            4 => { Env = unsave(1 as libc::c_int); m = munsave() }
            8 => {
                n = unsave(1 as libc::c_int);
                setcar(lookup(n), Acc);
                Acc = n;
                m = munsave()
            }
            9 => {
                if 8192 as libc::c_int + 5 as libc::c_int == cdar(Stack) {
                    Acc = car(unsave(1 as libc::c_int));
                    m = MEXPR as libc::c_int
                } else {
                    Acc = caar(Stack);
                    setcar(Stack, cdar(Stack));
                    msave(MPROG as libc::c_int);
                    m = MEXPR as libc::c_int
                }
            }
            0 => { return Acc }
            _ => { }
        }
    }
    return 8192 as libc::c_int + 5 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn init() {
    Verbose_GC = 0 as libc::c_int;
    Symbols = 8192 as libc::c_int + 5 as libc::c_int;
    Tmpcar = 8192 as libc::c_int + 5 as libc::c_int;
    Tmpcdr = 8192 as libc::c_int + 5 as libc::c_int;
    Tmp = 8192 as libc::c_int + 5 as libc::c_int;
    Id = 0 as libc::c_int;
    Freelist = 8192 as libc::c_int + 5 as libc::c_int;
    Input = 0 as libc::c_int;
    Inbuf = Buffer.as_mut_ptr();
    Ink = 0 as libc::c_int;
    Inp = Ink;
    Output = 1 as libc::c_int;
    Outp = 0 as libc::c_int;
    Rejected = 8192 as libc::c_int + 4 as libc::c_int;
    Parens = 0 as libc::c_int;
    Stack = 8192 as libc::c_int + 5 as libc::c_int;
    Mstack = 8192 as libc::c_int + 5 as libc::c_int;
    ::std::ptr::write_volatile(&mut Error as *mut libc::c_int,
                               0 as libc::c_int);
    Env = 8192 as libc::c_int + 5 as libc::c_int;
    S_t =
        addsym(b"t\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_apply =
        addsym(b"apply\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_if =
        addsym(b"if\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_ifnot =
        addsym(b"ifnot\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_lambda =
        addsym(b"lambda\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_lamstar =
        addsym(b"lambda*\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_macro =
        addsym(b"macro\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_prog =
        addsym(b"prog\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_quote =
        addsym(b"quote\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_qquote =
        addsym(b"qquote\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_unquote =
        addsym(b"unquote\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_splice =
        addsym(b"splice\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_setq =
        addsym(b"setq\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_it =
        addsym(b"it\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int + 1 as libc::c_int);
    S_cons =
        addsym(b"cons\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_car =
        addsym(b"car\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_cdr =
        addsym(b"cdr\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_atom =
        addsym(b"atom\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_eq =
        addsym(b"eq\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_eofp =
        addsym(b"eofp\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_setcar =
        addsym(b"setcar\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_setcdr =
        addsym(b"setcdr\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_gensym =
        addsym(b"gensym\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_read =
        addsym(b"read\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_prin =
        addsym(b"prin\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_prin1 =
        addsym(b"prin1\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_print =
        addsym(b"print\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_error =
        addsym(b"error\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_load =
        addsym(b"load\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_gc =
        addsym(b"gc\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
    S_suspend =
        addsym(b"suspend\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char, 8192 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kbint(_: libc::c_int) {
    ::std::ptr::write_volatile(&mut Error as *mut libc::c_int,
                               1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn kbbrk() -> libc::c_int {
    ::std::ptr::write_volatile(&mut Error as *mut libc::c_int,
                               1 as libc::c_int);
    return ::std::ptr::read_volatile::<libc::c_int>(&Error as
                                                        *const libc::c_int);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char)
 -> libc::c_int {
    init();

    let result = std::panic::catch_unwind(|| {
        fasload(if argc > 1 as libc::c_int {
                    *argv.offset(1 as libc::c_int as isize)
                } else { b"klisp\x00" as *const u8 as *const libc::c_char } as
                    *mut libc::c_char);
    });

    if result.is_err() {
        std::process::exit(1);
    }

    loop {
        let result = std::panic::catch_unwind(|| {

            signal(2 as libc::c_int,
                kbint as usize);
            loop  {
                Parens = 0 as libc::c_int;
                Loads = 0 as libc::c_int;
                ::std::ptr::write_volatile(&mut Error as *mut libc::c_int,
                                        0 as libc::c_int);
                Env = 8192 as libc::c_int + 5 as libc::c_int;
                Acc = Env;
                Mstack = 8192 as libc::c_int + 5 as libc::c_int;
                Stack = Mstack;
                pr(b"* \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char);
                flush();
                Acc = xread();
                if Error != 0 { continue ; }
                if 8192 as libc::c_int + 4 as libc::c_int == Acc { break; }
                print(eval(Acc));
                setcar(cdr(S_it), Acc);
                nl();
            }

        });

        if result.is_ok() {
            break;
        }
    }
    nl();
    return 0 as libc::c_int;
}

pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as libc::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut libc::c_char) as i32)
    }
}
