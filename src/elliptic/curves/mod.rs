//! # General purpose elliptic curve cryptography
//!
//! Here we define generic elliptic curve cryptography and provide implementation of several curves.
//!
//! ## Usage
//!
//! Elliptic curve cryptography operates on points and scalars. We provide according structures
//! [Point\<E\>](Point), [Scalar\<E\>](Scalar) (and [PointZ\<E\>](PointZ), [ScalarZ\<E\>](ScalarZ)
//! — for points and scalars that might be zero), where generic `E` stands for choice of elliptic
//! curve, e.g. [Secp256k1] (`Point<Secp256k1>`, `Scalar<Secp256k1>`, etc.).
//!
//! Various methods and traits are defined for points and scalars which basically empowers you to do
//! anything you can do in elliptic curve cryptography.
//!
//! ## Examples
//!
//! ### Public point/private scalar generation
//! ```rust
//! use curv::elliptic::curves::{Point, Scalar, Secp256k1};
//!
//! // Samples a random nonzero scalar (mod group order)
//! let secret = Scalar::<Secp256k1>::random();
//! // Multiplies generator at secret, retrieving a public point
//! let public = Point::generator() * secret;    
//! ```
//!
//! ### Diffie-Hellman
//!
//! Function below is a final step of the Diffie-Hellman key exchange protocol when both parties
//! have exchanged their ephemeral public keys. Giving it here just for an example, the `curv` library
//! includes implementation of this protocol (see [dh_key_exchange], or its more involved version:
//! [dh_key_exchange_variant_with_pok_comm]).
//!
//! [dh_key_exchange]: crate::cryptographic_primitives::twoparty::dh_key_exchange
//! [dh_key_exchange_variant_with_pok_comm]: crate::cryptographic_primitives::twoparty::dh_key_exchange_variant_with_pok_comm
//!
//! ```rust
//! use curv::elliptic::curves::{Point, Scalar, Secp256k1};
//!
//! fn diffie_hellman(
//!     my_secret: &Scalar<Secp256k1>,
//!     counterparty_point: &Point<Secp256k1>
//! ) -> Point<Secp256k1> {
//!     my_secret * counterparty_point
//! }
//! ```
//!
//! You may have noticed that this function lacks of subgroup check (whether counterparty
//! point has order=group_order), and is vulnerable to [small subgroup attack][subgroup-attack].
//! Actually, **it isn't**! Any `Point<E>` instance is guaranteed to have large prime order,
//! so you can be sure that subgroup check was performed. See [guarantees section] to learn more.
//!
//! [subgroup-attack]: http://safecurves.cr.yp.to/twist.html
//! [Guarantees section]: Point#guarantees
//!
//! The function above performs DH only on secp256k1 curve, which is disappointing as the code will
//! look the same for any curve. Luckily we can make it generic over choice of curve:
//!
//! ```rust
//! use curv::elliptic::curves::{Curve, Point, Scalar};
//!
//! fn diffie_hellman<E: Curve>(
//!     my_secret: &Scalar<E>,
//!     counterparty_point: &Point<E>
//! ) -> Point<E> {
//!     my_secret * counterparty_point
//! }
//! ```
//!
//! `Point<E>` (for generic `E: Curve`) implements many traits you might need (e.g. Serialize, PartialEq,
//! Debug, etc.) without specifying additional bounds. The same apllies to other structures (PointZ, Scalar, ...).
//!
//! ## Implementing your own curve
//!
//! Downstream crates can define their own curves just by implementing [Curve], [ECPoint], [ECScalar]
//! traits, no additional work is required. Note that these traits are intended not to be used directly.
//! Point, Scalar structures wrap ECPoint / ECScalar implementation, and provide a lot of convenient
//! methods, implement arithmetic traits, etc.

pub mod bls12_381;
pub mod curve_ristretto;
pub mod ed25519;
pub mod p256;
pub mod secp256_k1;

#[cfg(test)]
mod test;
mod traits;
mod wrappers;

pub use self::{
    bls12_381::{Bls12_381_1, Bls12_381_2},
    curve_ristretto::Ristretto,
    ed25519::Ed25519,
    p256::Secp256r1,
    secp256_k1::Secp256k1,
};
pub use self::{
    traits::{Curve, ECPoint, ECScalar, PointCoords},
    wrappers::{Generator, Point, PointRef, PointZ, Scalar, ScalarZ},
};

pub mod error {
    pub use super::{
        traits::{DeserializationError, NotOnCurve},
        wrappers::error::*,
    };
}

#[doc(no_inline)]
pub use self::error::*;
