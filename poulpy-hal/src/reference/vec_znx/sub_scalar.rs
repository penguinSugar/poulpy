use crate::layouts::{ScalarZnxRef, ScalarZnxToRef, VecZnxMut, VecZnxRef, VecZnxToMut, VecZnxToRef};
use crate::{
    layouts::{ZnxInfos, ZnxView, ZnxViewMut},
    reference::znx::{ZnxSub, ZnxSubInplace, ZnxZero},
};

pub fn vec_znx_sub_scalar<R, A, B, ZNXARI>(res: &mut R, res_col: usize, a: &A, a_col: usize, b: &B, b_col: usize, b_limb: usize)
where
    R: VecZnxToMut,
    A: ScalarZnxToRef,
    B: VecZnxToRef,
    ZNXARI: ZnxSub + ZnxZero,
{
    let a: ScalarZnxRef<'_> = a.to_ref();
    let b: VecZnxRef<'_> = b.to_ref();
    let mut res: VecZnxMut<'_> = res.to_mut();

    let min_size: usize = b.size().min(res.size());

    #[cfg(debug_assertions)]
    {
        assert!(b_limb < min_size, "b_limb: {b_limb} > min_size: {min_size}");
    }

    for j in 0..min_size {
        if j == b_limb {
            ZNXARI::znx_sub(res.at_mut(res_col, j), b.at(b_col, j), a.at(a_col, 0));
        } else {
            res.at_mut(res_col, j).copy_from_slice(b.at(b_col, j));
        }
    }

    for j in min_size..res.size() {
        ZNXARI::znx_zero(res.at_mut(res_col, j));
    }
}

pub fn vec_znx_sub_scalar_inplace<R, A, ZNXARI>(res: &mut R, res_col: usize, res_limb: usize, a: &A, a_col: usize)
where
    R: VecZnxToMut,
    A: ScalarZnxToRef,
    ZNXARI: ZnxSubInplace,
{
    let a: ScalarZnxRef<'_> = a.to_ref();
    let mut res: VecZnxMut<'_> = res.to_mut();

    #[cfg(debug_assertions)]
    {
        assert!(res_limb < res.size());
    }

    ZNXARI::znx_sub_inplace(res.at_mut(res_col, res_limb), a.at(a_col, 0));
}
