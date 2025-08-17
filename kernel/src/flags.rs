use bitflags::bitflags;
use libc::c_int;

bitflags! {
/*
 * Flags for ioflag. (high 16 bits used to ask for read-ahead and
 * help with write clustering)
 * NB: IO_NDELAY and IO_DIRECT are linked to fcntl.h
#define	IO_UNIT		0x0001		/* do I/O as atomic unit */
#define	IO_APPEND	0x0002		/* append write to end */
#define	IO_NDELAY	0x0004		/* FNDELAY flag set in file table */
#define	IO_NODELOCKED	0x0008		/* underlying node already locked */
#define	IO_ASYNC	0x0010		/* bawrite rather then bdwrite */
#define	IO_VMIO		0x0020		/* data already in VMIO space */
#define	IO_INVAL	0x0040		/* invalidate after I/O */
#define	IO_SYNC		0x0080		/* do I/O synchronously */
#define	IO_DIRECT	0x0100		/* attempt to bypass buffer cache */
#define	IO_NOREUSE	0x0200		/* VMIO data won't be reused */
#define	IO_EXT		0x0400		/* operate on external attributes */
#define	IO_NORMAL	0x0800		/* operate on regular data */
#define	IO_NOMACCHECK	0x1000		/* MAC checks unnecessary */
#define	IO_BUFLOCKED	0x2000		/* ffs flag; indir buf is locked */
#define	IO_RANGELOCKED	0x4000		/* range locked */
#define	IO_DATASYNC	0x8000		/* do only data I/O synchronously */

#define IO_SEQMAX	0x7F		/* seq heuristic max value */
#define IO_SEQSHIFT	16		/* seq heuristic in upper 16 bits */
*/
    pub struct Ioflag: u32 {
        const IO_UNIT = 0x0001;
        const IO_APPEND = 0x0002;
        const IO_NDELAY = 0x0004;
        const IO_NODELOCKED = 0x0008;
        const IO_ASYNC = 0x0010;
        const IO_VMIO = 0x0020;
        const IO_INVAL = 0x0040;
        const IO_SYNC = 0x0080;
        const IO_DIRECT = 0x0100;
        const IO_NOREUSE = 0x0200;
        const IO_EXT = 0x0400;
        const IO_NORMAL = 0x0800;
        const IO_NOMACCHECK = 0x1000;
        const IO_BUFLOCKED = 0x2000;
        const IO_RANGELOCKED = 0x4000;
        const IO_DATASYNC = 0x8000;

        const IO_SEQMAX = 0x7F;
        const IO_SEQSHIFT = 16;
    }

/*
 * File status flags: these are used by open(2), fcntl(2).
 * They are also used (indirectly) in the kernel file structure f_flags,
 * which is a superset of the open/fcntl flags.  Open flags and f_flags
 * are inter-convertible using OFLAGS(fflags) and FFLAGS(oflags).
 * Open/fcntl flags begin with O_; kernel-internal flags begin with F.
/* open-only flags */
#define	O_RDONLY	0x0000		/* open for reading only */
#define	O_WRONLY	0x0001		/* open for writing only */
#define	O_RDWR		0x0002		/* open for reading and writing */
#define	O_ACCMODE	0x0003		/* mask for above modes */
*/
    pub struct Oflags: u32 {
        const O_RDONLY = 0x0000;
        const O_WRONLY = 0x0001;
        const O_RDWR = 0x0002;
        const O_ACCMODE = 0x0003;
    }
}

impl Ioflag {
    pub fn convert(c_ioflag: c_int) -> Self {
        Self::from_bits_truncate(c_ioflag as u32)
    }
}

impl Oflags {
    pub fn convert(c_oflags: c_int) -> Self {
        Self::from_bits_truncate(c_oflags as u32)
    }   
}
