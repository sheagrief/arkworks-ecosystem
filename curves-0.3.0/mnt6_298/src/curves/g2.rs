#![allow(warnings)]
use ark_ec::{
    mnt6,
    mnt6::MNT6Parameters,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::field_new;

use crate::{g1, Fq, Fq3, Fr, FQ_ZERO};

pub type G2Affine = mnt6::G2Affine<crate::Parameters>;
pub type G2Projective = mnt6::G2Projective<crate::Parameters>;
pub type G2Prepared = mnt6::G2Prepared<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq3;
    type ScalarField = Fr;
}

/// MUL_BY_A_C0 = NONRESIDUE * COEFF_A = 5 * 11
    #[rustfmt::skip]
pub const MUL_BY_A_C0: Fq = field_new!(Fq, "55");

/// MUL_BY_A_C1 = NONRESIDUE * COEFF_A
    #[rustfmt::skip]
pub const MUL_BY_A_C1: Fq = field_new!(Fq, "55");

/// MUL_BY_A_C2 = COEFF_A
pub const MUL_BY_A_C2: Fq = g1::Parameters::COEFF_A;

impl SWModelParameters for Parameters {
    const COEFF_A: Fq3 = crate::Parameters::TWIST_COEFF_A;
    #[rustfmt::skip]
    const COEFF_B: Fq3 = field_new!(Fq3,
        // 5 * G1::COEFF_B
        field_new!(Fq, "57578116384997352636487348509878309737146377454014423897662211075515354005624851787652233"),
        FQ_ZERO,
        FQ_ZERO,
    );

    /// COFACTOR =
    /// 226502022472576270196498690498308461791828762732602586162207535351960270082712694977333372361549082214519252261735048131889018501404377856786623430385820659037970876666767495659520
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        15308190245346869248,
        10669098443577192943,
        4561413759929581409,
        3680089780298582849,
        17336300687782721465,
        10745756320947240891,
        17479264233688728128,
        16828697388537672097,
        4184034152442024798,
        915787,
    ];

    /// COFACTOR^(-1) mod r =
    /// 79320381028210220958891541608841408590854146655427655872973753568875979721417185067925504
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "79320381028210220958891541608841408590854146655427655872973753568875979721417185067925504");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elt: &Fq3) -> Fq3 {
        field_new!(
            Fq3,
            MUL_BY_A_C0 * &elt.c1,
            MUL_BY_A_C1 * &elt.c2,
            MUL_BY_A_C2 * &elt.c0,
        )
    }
}

const G2_GENERATOR_X: Fq3 =
    field_new!(Fq3, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_X_C2);
const G2_GENERATOR_Y: Fq3 =
    field_new!(Fq3, G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1, G2_GENERATOR_Y_C2);

pub const G2_GENERATOR_X_C0: Fq = field_new!(
    Fq,
    "421456435772811846256826561593908322288509115489119907560382401870203318738334702321297427"
);
pub const G2_GENERATOR_X_C1: Fq = field_new!(
    Fq,
    "103072927438548502463527009961344915021167584706439945404959058962657261178393635706405114"
);
pub const G2_GENERATOR_X_C2: Fq = field_new!(
    Fq,
    "143029172143731852627002926324735183809768363301149009204849580478324784395590388826052558"
);

pub const G2_GENERATOR_Y_C0: Fq = field_new!(
    Fq,
    "464673596668689463130099227575639512541218133445388869383893594087634649237515554342751377"
);
pub const G2_GENERATOR_Y_C1: Fq = field_new!(
    Fq,
    "100642907501977375184575075967118071807821117960152743335603284583254620685343989304941678"
);
pub const G2_GENERATOR_Y_C2: Fq = field_new!(
    Fq,
    "123019855502969896026940545715841181300275180157288044663051565390506010149881373807142903"
);
