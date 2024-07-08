#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]

use derive_more::IsVariantAnd;

#[derive(IsVariantAnd)]
enum Either<TLeft, TRight> {
    Left(TLeft),
    Right(TRight),
}

const _: () = {
    let either: Either<u8, i16> = Either::Right(7);
    assert!(either.is_right());
    assert!(!either.is_left());

    let either: Either<u8, i16> = Either::Left(7);
    assert!(!either.is_right());
    assert!(either.is_left());
};