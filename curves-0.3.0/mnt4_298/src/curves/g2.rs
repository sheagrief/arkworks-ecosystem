#![allow(clippy::op_ref)]
use crate::{Fq, Fq2, Fr, FQ_ZERO, G1_COEFF_A_NON_RESIDUE};
use ark_ec::{
    mnt4,
    mnt4::MNT4Parameters,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::field_new;

pub type G2Affine = mnt4::G2Affine<crate::Parameters>;
pub type G2Projective = mnt4::G2Projective<crate::Parameters>;
pub type G2Prepared = mnt4::G2Prepared<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;
}

/// MUL_BY_A_C0 = NONRESIDUE * COEFF_A
#[rustfmt::skip]
pub const MUL_BY_A_C0: Fq = G1_COEFF_A_NON_RESIDUE;

/// MUL_BY_A_C1 = NONRESIDUE * COEFF_A
#[rustfmt::skip]
pub const MUL_BY_A_C1: Fq = G1_COEFF_A_NON_RESIDUE;

impl SWModelParameters for Parameters {
    const COEFF_A: Fq2 = crate::Parameters::TWIST_COEFF_A;
    // B coefficient of MNT4-298 G2 =
    // ```
    // mnt4298_twist_coeff_b = mnt4298_Fq2(mnt4298_Fq::zero(),
    //                                  mnt4298_G1::coeff_b * mnt4298_Fq2::non_residue);
    // non_residue = mnt4298_Fq2::non_residue = mnt4298_Fq("13");
    //  = (ZERO, G1_B_COEFF * NON_RESIDUE);
    //  =
    //  (0, 67372828414711144619833451280373307321534573815811166723479321465776723059456513877937430)
    // ```
    #[rustfmt::skip]
    const COEFF_B: Fq2 = field_new!(Fq2,
        FQ_ZERO,
        field_new!(Fq, "67372828414711144619833451280373307321534573815811166723479321465776723059456513877937430"),
    );

    /// COFACTOR =
    /// 475922286169261325753349249653048451545124879932565935237842521413255878328503110407553025
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        15480692783052488705,
        9802782456999489873,
        14622846468721090623,
        11702080941310629006,
        4110145082483,
    ];

    /// COFACTOR^(-1) mod r =
    /// 475922286169261325753349249653048451545124878207887910632124039320641839552134835598065665
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "475922286169261325753349249653048451545124878207887910632124039320641839552134835598065665");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elt: &Fq2) -> Fq2 {
        field_new!(Fq2, MUL_BY_A_C0 * &elt.c0, MUL_BY_A_C1 * &elt.c1,)
    }
}

const G2_GENERATOR_X: Fq2 = field_new!(Fq2, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
const G2_GENERATOR_Y: Fq2 = field_new!(Fq2, G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

// Generator of G2
// These are two Fq elements each because X and Y (and Z) are elements of Fq^2
// X = 438374926219350099854919100077809681842783509163790991847867546339851681564223481322252708,
//     37620953615500480110935514360923278605464476459712393277679280819942849043649216370485641,
// Y = 37437409008528968268352521034936931842973546441370663118543015118291998305624025037512482,
//     424621479598893882672393190337420680597584695892317197646113820787463109735345923009077489,
#[rustfmt::skip]
pub const G2_GENERATOR_X_C0: Fq = field_new!(Fq, "438374926219350099854919100077809681842783509163790991847867546339851681564223481322252708");

#[rustfmt::skip]
pub const G2_GENERATOR_X_C1: Fq = field_new!(Fq, "37620953615500480110935514360923278605464476459712393277679280819942849043649216370485641");

#[rustfmt::skip]
pub const G2_GENERATOR_Y_C0: Fq = field_new!(Fq, "37437409008528968268352521034936931842973546441370663118543015118291998305624025037512482");

#[rustfmt::skip]
pub const G2_GENERATOR_Y_C1: Fq = field_new!(Fq, "424621479598893882672393190337420680597584695892317197646113820787463109735345923009077489");
