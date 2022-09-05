use std::iter::{Iterator, IntoIterator};
use std::convert::TryInto;

const MOLD_HEADER_LEN : usize = 20;

pub struct MoldReader<'a> {
  data: &'a [u8],
}

fn as_u64(bytes: &[u8]) -> u64 {
  (bytes[7] as u64) +
  ((bytes[6] as u64) << 8) +
  ((bytes[5] as u64) << 16) +
  ((bytes[4] as u64) << 24) +
  ((bytes[3] as u64) << 32) +
  ((bytes[2] as u64) << 40) +
  ((bytes[1] as u64) << 48) +
  ((bytes[0] as u64) << 56)
}

impl<'a> MoldReader<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    Self{data: data}
  }
  pub fn iter(&self) -> MoldIter {
    let iter = MoldIter{data: self.data, bytes_eaten: 0, msg_count: 0};
    iter
  }
  pub fn len(&self) -> usize {
    (self.data[19] as u16 + ((self.data[18] as u16) << 8)) as usize
  }
  pub fn seqno(&self) -> u64 {
    as_u64(&self.data[10..])
  }
  pub fn session(&self) -> &'a [u8] {
    &self.data[0..10]
  }
}

impl<'a> IntoIterator for &'a MoldReader<'a> {
  type Item = &'a [u8];
  type IntoIter = MoldIter<'a>;
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

pub struct MoldIter<'a> {
  data: &'a [u8],
  bytes_eaten: usize,
  msg_count: u16,
}

impl<'a> Iterator for MoldIter<'a> {
  type Item = &'a [u8];
  fn next(&mut self) -> Option<&'a [u8]> {
    if self.bytes_eaten == 0 {
      self.bytes_eaten = 10 + 8 + 2;
      self.msg_count = self.data[19] as u16 + ((self.data[18] as u16) << 8);
    }
    let bytes_remaining = self.data.len() - self.bytes_eaten;
    if bytes_remaining <= 0 || self.msg_count <= 0 {
      return None;
    }
    let offset = self.bytes_eaten;
    let msglen = (((self.data[offset] as u16) << 8) + self.data[offset+1] as u16) as usize;
    if bytes_remaining < msglen + 2 {
      return None;
    }
    self.msg_count -= 1;
    self.bytes_eaten += 2 + msglen;
    return Some(&self.data[offset+2..offset+2+msglen]);
  }
}

pub struct MoldWriter {
  buf: [u8; 1400],
  bytes_written: usize,
}

impl MoldWriter {
  pub fn new(session: &str, seqno: u64) -> Self {
    let mut ans = Self{buf: [0u8; 1400], bytes_written: 0};
    ans.set_session(session).set_seqno(seqno).set_message_count(0);
    ans
  }

  pub fn set_session(&mut self, what: &str) -> &mut Self {
    self.buf[0..10].copy_from_slice(what.as_bytes());
    self
  }

  pub fn set_seqno(&mut self, seqno: u64) -> &mut Self {
    self.buf[10..18].copy_from_slice(&seqno.to_be_bytes()[..]);
    self
  }

  fn set_message_count(&mut self, msg_count: u16) -> &mut Self {
    self.buf[18..20].copy_from_slice(&msg_count.to_be_bytes()[..]);
    self
  }

  fn increment_count(&mut self) {
    let msg_count_bytes : [u8;2] = self.buf[18..20].try_into().unwrap();
    let mut msg_count = u16::from_be_bytes(msg_count_bytes);
    msg_count += 1;
    self.set_message_count(msg_count);
    println!("current message count: {}", msg_count);
  }
  pub fn add_message(&mut self, what: &[u8]) -> &mut Self {
    if ! self.can_fit(what.len()) {
      return self;
    }
    self.increment_count();
    // let msg_count_bytes : [u8;2] = self.buf[18..20].try_into().unwrap();
    // let mut msg_count = u16::from_be_bytes(msg_count_bytes);
    // msg_count += 1;
    // self.buf[18..20].copy_from_slice(&msg_count.to_be_bytes()[..]);
    let msg_size = what.len() as u16;
    let offsets = (MOLD_HEADER_LEN+self.bytes_written, MOLD_HEADER_LEN+self.bytes_written+2);
    self.buf[offsets.0..offsets.1].copy_from_slice(&msg_size.to_be_bytes()[..]);
    self.buf[offsets.0+2..offsets.0+2+what.len()].copy_from_slice(what);
    self.bytes_written += what.len() + 2;
    return self;
  }
  pub fn write_message<F : FnOnce(&mut [u8])>(&mut self, msg_size: u16, writer: F) -> &mut Self {
    if ! self.can_fit(msg_size.into()) {
      return self;
    }
    self.increment_count();
    let offsets = (MOLD_HEADER_LEN+self.bytes_written, MOLD_HEADER_LEN+self.bytes_written+2);
    self.buf[offsets.0..offsets.1].copy_from_slice(&msg_size.to_be_bytes()[..]);
    writer(&mut self.buf[offsets.0+2..offsets.0+2+msg_size as usize]);
    self.bytes_written += (msg_size + 2) as usize;
    return self;
  }
  pub fn size_remaining(&self) -> usize {
    self.buf.len() - (self.bytes_written + MOLD_HEADER_LEN)
  }
  pub fn can_fit(&self, msg_size: usize) -> bool {
    (msg_size + 2) < self.size_remaining()
  }
  pub fn data(&self) -> &[u8] {
    return &self.buf[..self.bytes_written+MOLD_HEADER_LEN];
  }
  pub fn reset(&mut self) {
    self.set_message_count(0);
    self.bytes_written = 0;
  }
}

#[test]
fn mold_iterate() {
  use std::str;
  let msgbuf = b"1234567890\x00\x00\x00\x00\x00\x00\x00\x01\x00\x03\x00\x01X\x00\x04ASDF\x00\x0AABCDEFGHIJ";
  let reader = MoldReader::new(&msgbuf[..]);
  for (i, msg) in reader.iter().enumerate() {
    println!("msg: session[{}] seqno[{}] msg#{}: \"{}\"", str::from_utf8(reader.session()).unwrap(), reader.seqno(), i, str::from_utf8(msg).unwrap());
  }
  println!("seqno was: {}", reader.seqno());
}


#[test]
fn mold_writer() {
  use std::str;
  let mut writer = MoldWriter::new("1234567890", 666);
  writer
    .add_message(b"HELLO")
    .add_message(b"GOODBYE")
    .add_message(b"BOOGADEEBOO")
    .write_message(6, |loc| {
      loc.clone_from_slice(b"FOOBAR");
    })
    .write_message(12, |loc| {
      loc.clone_from_slice(b"BAZQUXFOOBAR");
    });
  let buf = writer.data();
  assert_eq!(&buf[0..10], b"1234567890");
  println!("{:?}", buf);

  let reader = MoldReader::new(&buf);
  assert_eq!(reader.len(), 5);
  assert_eq!(reader.seqno(), 666);
  let expected = vec!["HELLO","GOODBYE","BOOGADEEBOO","FOOBAR","BAZQUXFOOBAR"];
  for (i, msg) in reader.iter().enumerate() {
    println!("msg: session[{}] seqno[{}] msg#{}: \"{}\"", str::from_utf8(reader.session()).unwrap(), reader.seqno() + i as u64, i, str::from_utf8(msg).unwrap());
    assert_eq!(std::str::from_utf8(msg).unwrap(), expected[i]);
  }
}
