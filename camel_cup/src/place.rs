#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Place {
    TopRemaining,
    Top(u8),
    MiddleRemaining,
    Bottom(u8),
    BottomRemaining,
    Wrong,
}