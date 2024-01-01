extern crate superlu_sys as raw;

use std::mem::MaybeUninit;

// https://github.com/copies/superlu/blob/master/EXAMPLE/superlu.c
#[allow(non_snake_case)]
#[test]
fn superlu() {
    use std::mem::MaybeUninit;
    use std::slice::from_raw_parts_mut;

    use raw::*;
    use raw::Dtype_t::*;
    use raw::Mtype_t::*;
    use raw::Stype_t::*;
    use raw::colperm_t::*;

    unsafe {
        let (m, n, nnz) = (5, 5, 12);

        let a = doubleMalloc(nnz);
        assert!(!a.is_null());
        {
            let (s, u, p, e, r, l) = (19.0, 21.0, 16.0, 5.0, 18.0, 12.0);
            let a = from_raw_parts_mut(a, nnz as usize);
            a[0] = s;
            a[1] = l;
            a[2] = l;
            a[3] = u;
            a[4] = l;
            a[5] = l;
            a[6] = u;
            a[7] = p;
            a[8] = u;
            a[9] = e;
            a[10] = u;
            a[11] = r;
        }

        let asub = intMalloc(nnz);
        assert!(!asub.is_null());
        {
            let asub = from_raw_parts_mut(asub, nnz as usize);
            asub[0] = 0;
            asub[1] = 1;
            asub[2] = 4;
            asub[3] = 1;
            asub[4] = 2;
            asub[5] = 4;
            asub[6] = 0;
            asub[7] = 2;
            asub[8] = 0;
            asub[9] = 3;
            asub[10] = 3;
            asub[11] = 4;
        }

        let xa = intMalloc(n + 1);
        assert!(!xa.is_null());
        {
            let xa = from_raw_parts_mut(xa, (n + 1) as usize);
            xa[0] = 0;
            xa[1] = 3;
            xa[2] = 6;
            xa[3] = 8;
            xa[4] = 10;
            xa[5] = 12;
        }

        let mut A: SuperMatrix = unsafe { MaybeUninit::zeroed().assume_init() };

        dCreate_CompCol_Matrix(&mut A, m, n, nnz, a, asub, xa, SLU_NC, SLU_D, SLU_GE);

        let nrhs = 1;
        let rhs = doubleMalloc(m * nrhs);
        assert!(!rhs.is_null());
        {
            let rhs = from_raw_parts_mut(rhs, (m * nrhs) as usize);
            for i in 0..((m * nrhs) as usize) {
                rhs[i] = 1.0;
            }
        }

        let mut B: SuperMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
        dCreate_Dense_Matrix(&mut B, m, nrhs, rhs, m, SLU_DN, SLU_D, SLU_GE);

        let perm_r = intMalloc(m);
        assert!(!perm_r.is_null());

        let perm_c = intMalloc(n);
        assert!(!perm_c.is_null());

        let mut options: superlu_options_t = unsafe { MaybeUninit::zeroed().assume_init() };
        set_default_options(&mut options);
        options.ColPerm = NATURAL;

        let mut stat: SuperLUStat_t = unsafe { MaybeUninit::zeroed().assume_init() };
        StatInit(&mut stat);

        let mut L: SuperMatrix = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut U: SuperMatrix = unsafe { MaybeUninit::zeroed().assume_init() };

        let mut info = 0;
        dgssv(
            &mut options,
            &mut A,
            perm_c,
            perm_r,
            &mut L,
            &mut U,
            &mut B,
            &mut stat,
            &mut info,
        );

        SUPERLU_FREE(rhs as *mut _);
        SUPERLU_FREE(perm_r as *mut _);
        SUPERLU_FREE(perm_c as *mut _);
        Destroy_CompCol_Matrix(&mut A);
        Destroy_SuperMatrix_Store(&mut B);
        Destroy_SuperNode_Matrix(&mut L);
        Destroy_CompCol_Matrix(&mut U);
        StatFree(&mut stat);
    }
}
