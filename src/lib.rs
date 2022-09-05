pub mod itch;
pub mod moldudp;

pub use crate::itch::*;

#[cfg(test)]
mod tests {

// use std::io;
// use std::collections::HashMap;
use super::*;

#[test]
fn process_packet() {
  let msgbuf = b"SUCKONTHIS\x00\x00\x00\x00\x00\x00\x00\x02\
    \x00\x03\
    \x00\x24\
    A\x00\x10\x00\x00\
    \x16\xce\xd3\xc5\xb0\xc8\x10\x20\
    \x30\x40\x50\x60\x70\x7F\
    B\
    \x00\x00\x00\x64\
    AMZN    \
    \x01\xe8\x6e\x48\
    \x00\x1f\
    E\x00\x10\x00\x00\x16\xce\xd3\xc5\xb0\xc8\x10\x20\x30\x40\x50\x60\x70\x7F\x00\x00\x00\x0A\x00\x00\x00\x00\x00\x00\x10\x00\
    \x00\x13\
    D\x00\x10\x00\x00\x00\x00\x00\x00\x00\xFF\x10\x20\x30\x40\x50\x60\x70\x7F";
  let mut handler = itch::Dumper{};
  let reader = moldudp::MoldReader::new(&msgbuf[..]);
  for (i, msg) in reader.iter().enumerate() {
    println!("processing session:{} seqno:{} msgno:{}", std::str::from_utf8(reader.session()).unwrap(), reader.seqno(), i);
    itch::crack_message(msg, &mut handler);
  }
}

#[test]
fn test_write() {
  let mut data = vec![0u8;512];
  let mut cursor = std::io::Cursor::new(&mut data[..]);
  itch::write_system_event(&mut cursor, 1234, 0, 0, itch::eSystemEvent(itch::eSystemEvent::Start_of_Messages)).unwrap();
  let expected = b"S\
    \x04\xd2\
    \x00\x00\
    \x00\x00\x00\x00\x00\x00\
    O\
    A\
    \x02\x9a\
    \x00\x00\
    \x00\x00\x00\x00\xff\xff\
    \x00\x00\x00\x00\x00\x00\xbe\xef\
    S\
    \x00\x00\x00\x64\
    AMZN    \
    \x00\x12\xd6\x44"
    ;

  let mut add_order = itch::AddOrder::default();
  add_order.stock_locate = 666;
  add_order.timestamp = 65535;
  add_order.order_reference_number = 0xbeef;
  add_order.buy_sell_indicator = itch::eBuySellIndicator(itch::eBuySellIndicator::Sell_Order);
  add_order.shares = 100;
  add_order.stock = b"AMZN    ".clone();
  add_order.price = 1234500;
  itch::write_add_order_struct(&mut cursor, add_order).unwrap();

  assert_eq!(data[..12+36], expected[..12+36]);
}

} // tests
