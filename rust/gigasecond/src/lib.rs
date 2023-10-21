use time::PrimitiveDateTime as DateTime;
use std::time::Duration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
  //  todo!("What time is a gigasecond later than {start}");
    let gigasecond = Duration::new(1_000_000_000,0);
    let mut ret: DateTime = start;
    ret += gigasecond;
    return ret;
}
