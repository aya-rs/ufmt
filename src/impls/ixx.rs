use core::str;

use crate::{uDebug, uDisplay, uWrite, Formatter};

macro_rules! ixx {
    ($uxx:ty, $n:expr, $buf:expr) => {{
        let n = $n;
        let negative = n.is_negative();
        let mut n = if negative {
            match n.checked_abs() {
                Some(n) => n as $uxx,
                None => <$uxx>::max_value() / 2 + 1,
            }
        } else {
            n as $uxx
        };
        let len = $buf.len();
        let mut i = len - 1;
        while i > 0 {
            unsafe { *$buf.get_unchecked_mut(i) = (n % 10) as u8 + b'0' };
            n = n / 10;

            if n == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        if negative {
            i -= 1;
            unsafe { *$buf.get_unchecked_mut(i) = b'-' };
        }

        i
    }};
}

#[inline(never)]
fn isize<W>(n: isize, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
where
    W: uWrite + ?Sized,
{
    let mut buf: [u8; 20] = unsafe { crate::uninitialized() };
    let i = ixx!(usize, n, &mut buf);
    f.write_str(unsafe { str::from_utf8_unchecked(buf.get_unchecked(i..)) })
}

impl uDebug for i8 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        isize(*self as isize, f)
    }
}

impl uDisplay for i8 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i8 as uDebug>::fmt(self, f)
    }
}

impl uDebug for i16 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        isize(*self as isize, f)
    }
}

impl uDisplay for i16 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i16 as uDebug>::fmt(self, f)
    }
}

impl uDebug for i32 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        isize(*self as isize, f)
    }
}

impl uDisplay for i32 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i32 as uDebug>::fmt(self, f)
    }
}

impl uDebug for i64 {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        isize(*self as isize, f)
    }
}

impl uDisplay for i64 {
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i64 as uDebug>::fmt(self, f)
    }
}

impl uDebug for isize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i16 as uDebug>::fmt(&(*self as i16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i32 as uDebug>::fmt(&(*self as i32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i64 as uDebug>::fmt(&(*self as i64), f)
    }
}

impl uDisplay for isize {
    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i16 as uDisplay>::fmt(&(*self as i16), f)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i32 as uDisplay>::fmt(&(*self as i32), f)
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        <i64 as uDisplay>::fmt(&(*self as i64), f)
    }
}
