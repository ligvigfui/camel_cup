#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Place {
    TopRemaining,
    Top(u8),
    MiddleRemaining,
    Bottom(u8),
    BottomRemaining,
    Wrong,
}