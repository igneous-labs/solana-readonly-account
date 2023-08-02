use core::ops::Deref;
use solana_program::{pubkey::Pubkey, stake_history::Epoch};

pub mod program;
pub mod sdk;

/// Until [SlicePattern](https://doc.rust-lang.org/core/slice/trait.SlicePattern.html) is merged into rust stable
/// We will need to do this double Deref bullshit hack for data because
/// Rc<RefCell<&mut [u8]>>::borrow() returns Ref<&mut [u8]>
/// and there's no common trait in stable with method `.as_slice()` that both &mut [u8] and &[u8] impls
/// (that would be SlicePattern)
pub trait ReadonlyAccount {
    type SliceDeref<'s>: Deref<Target = [u8]>
    where
        Self: 's;
    type DataDeref<'d>: Deref<Target = Self::SliceDeref<'d>>
    where
        Self: 'd;

    fn lamports(&self) -> u64;
    fn data(&self) -> Self::DataDeref<'_>;
    fn owner(&self) -> &Pubkey;
    fn executable(&self) -> bool;
    fn rent_epoch(&self) -> Epoch;
}
