#[derive(Clone)]
pub enum UiEvent {
    Done,
    Wait(u64),
    Fd(f32),
    Bk(f32),
    Lt(f32),
    Rt(f32),
    Setpos(f32, f32),
    Seth(f32),
    Setc(f32),
    Pd,
    Pu,
    St,
    Ht,
    Clean,
    Newturtle(String),
    Addr(String),
}
