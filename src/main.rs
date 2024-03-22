use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::{
    short_weierstrass::curves::bls12_381::curve::BLS12381Curve, traits::IsEllipticCurve,
};
use lambdaworks_math::field::fields::u64_goldilocks_field::Goldilocks64Field;
use lambdaworks_math::field::traits::IsField;

fn main() {
    let g1 = BLS12381Curve::generator();
    let sk = 0x6C616D6264617370_u64;
    let pk = g1.operate_with_self(sk);
    println!("Secret key: {:?}", sk);
    println!("Public key: {:?}", pk.to_affine().x().to_hex());
    let inv_2 = Goldilocks64Field::inv(&2_u64).unwrap();
    println!("Goldilocks inverse of 2: {:?}", inv_2)
}
