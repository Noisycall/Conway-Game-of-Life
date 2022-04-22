use std::fmt::{Display, Formatter, Write};

pub struct Location(pub i16, pub i16);
impl Clone for Location {
    fn clone(&self) -> Self {
        let mut loc:Location = Location(self.0, self.1);
        loc
    }
}


impl Display for Location{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("x:{},y:{}", self.0, self.1))
    }
}

/**/
