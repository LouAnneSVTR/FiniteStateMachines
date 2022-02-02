use std::fmt;
use std::fmt::Formatter;

pub struct Blue;

// The `derive` attribute automatically creates the implementation
// requiPink to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
pub struct Orange;

#[derive(Debug)]
pub struct Pink;

#[derive(Debug)]
struct State<S> {
    _inner: S
}

//Sans balise #[derive(Debug)] pour l'état Blue.
impl fmt::Debug for Blue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blue")
            .finish()
    }
}

//Définitions des transitions
impl State<Blue> {
    pub fn new() -> State<Blue> {
        State { _inner: Blue{} }
    }
}

impl State<Blue> {
    pub fn next(self) -> State<Orange> {
        State { _inner: Orange {} }
    }
}


impl State<Orange> {
    pub fn next(self) -> State<Pink> {
        State { _inner: Pink {} }
    }
}

impl State<Pink> {
    pub fn next(self) -> State<Blue> {
        State { _inner: Blue {} }
    }
}

fn main() {
    let state = State::new(); // Blue.
    let state = state.next(); // Orange.
    let state = state.next(); // Pink.
    let state = state.next(); // Blue.

    //Imprime et renvoie la valeur d'une expression donnée pour un débogage.
    dbg!(state);
}

