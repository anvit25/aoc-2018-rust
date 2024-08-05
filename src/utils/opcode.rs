use self::OpCode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpCode {
    AddI,
    AddR,
    MulI,
    MulR,
    BanI,
    BanR,
    BorI,
    BorR,
    SetI,
    SetR,
    GtIR,
    GtRI,
    GtRR,
    EqIR,
    EqRI,
    EqRR,
}

impl OpCode {
    pub const ALL: [OpCode; 16] = [
        AddI, AddR, MulI, MulR, BanI, BanR, BorI, BorR, SetI, SetR, GtIR, GtRI, GtRR, EqIR, EqRI,
        EqRR,
    ];

    pub fn apply(&self, register: &mut [usize], a: usize, b: usize, c: usize) {
        register[c] = match self {
            AddI => register[a] + b,
            AddR => register[a] + register[b],
            MulI => register[a] * b,
            MulR => register[a] * register[b],
            BanI => register[a] & b,
            BanR => register[a] & register[b],
            BorI => register[a] | b,
            BorR => register[a] | register[b],
            SetI => a,
            SetR => register[a],
            GtIR => (a > register[b]).into(),
            GtRI => (register[a] > b).into(),
            GtRR => (register[a] > register[b]).into(),
            EqIR => (a == register[b]).into(),
            EqRI => (register[a] == b).into(),
            EqRR => (register[a] == register[b]).into(),
        };
    }
}
