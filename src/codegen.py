import sys
import xml.etree.ElementTree as ET

type_map = {'char_t': 'u8'
        , 'char_2_t': '[u8;2]'
        , 'char_4_t': '[u8;4]'
        , 'char_8_t': '[u8;8]'
        , 'char_10_t': '[u8;10]'
        , 'char_14_t': '[u8;14]'
        , 'char_20_t': '[u8;20]'
        , 'u64_t': 'u64'
        #, 'u48_t': '[u8;6]'
        , 'u48_t': 'u64'
        , 'u32_t': 'u32'
        , 'u16_t': 'u16'
        , 'price_4_t': 'u32'
        , 'price_8_t': 'u64'
        }

def do_header(xml):
    print('use std::fmt;')
    print('use std::io::{Cursor, Read, Write};')
    print('use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};')
    print('')
    print('pub fn u48_to_u64(num: &[u8;6]) -> u64 {')
    print('  ((num[0] as u64) << 40) +')
    print('  ((num[1] as u64) << 32) +')
    print('  ((num[2] as u64) << 24) +')
    print('  ((num[3] as u64) << 16) +')
    print('  ((num[4] as u64) << 8) +')
    print('  ((num[5] as u64) << 0)')
    print('}')
    print('')

def do_constants(xml):
    pass

def do_enums(xml):
    print('// Enums')
    for item in xml.find('Enums'):
        print('#[derive(PartialEq, Default)]')
        print('#[allow(non_camel_case_types)]')
        print('pub struct {}(pub {});'.format(item.get('name'), type_map[item.get('type')]))
        print('#[allow(non_upper_case_globals)]')
        print('impl {} {{'.format(item.get('name')))
        for value in item:
            print('  pub const {} : {} = \'{}\' as {};'.format(value.get('name'), type_map[item.get('type')], value.get('value'), type_map[item.get('type')]))
        # print('  fn new(code : u8) -> Self {{ {}(code) }}'.format(item.get('name')))
        print('}} // {}'.format(item.get('name')))
        print('')
        type_map[item.get('name')] = item.get('name')

        stringize = lambda x: x.replace('_', ' ')

        print('impl fmt::Display for {} {{'.format(item.get('name')))
        print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
        print('    let s = match self.0 {')
        for value in item:
            print('      {}::{} => "\'{}\' ({})".to_string(),'.format(item.get('name'), value.get('name'), value.get('value'), stringize(value.get('name'))))
        print('      _ => "Unknown".to_string(),')
        print('    };')
        print('    write!(f, "{}", s)')
        print('  }')
        print('}')
        print('')



def do_structs(xml):
    struct_name = lambda x: ''.join(map(lambda n: n[0].upper() + n[1:], x.split('_')))
    print('// Structs')
    for item in xml.find('Structs'):
        len = item.get('len')
        # print('#[repr(C, packed)]')
        print('#[derive(Default)]')
        print('pub struct {} {{'.format(struct_name(item.get('name'))))
        for field in item:
            print('  pub {}: {},'.format(field.get('name'), type_map[field.get('type')]))
        print('}} // {}'.format(struct_name(item.get('name'))))
        print('pub const {}_SIZE : usize = {};'.format(item.get('name').upper(), item.get('len')))
        # size check
        # print('const _{}_SIZE_CHECK : usize = (std::mem::size_of::<{}>() == {}) as usize - 1;'.format(struct_name(item.get('name')).upper(), struct_name(item.get('name')), item.get('len')))
        # print('assert!({}_SIZE_CHECK != 0);'.format(struct_name(item.get('name')).upper()))
        print('')

        print('impl {} {{'.format(struct_name(item.get('name'))))
        if item.get('id') is not None:
            print('  pub const TYPE : u8 = \'{}\' as u8;'.format(item.get('id')))

        def do_struct_parser(error_str):
            print('    let mut obj = Self::default();')
            for field in item:
                if field.get('type') == 'char_t':
                    print('    rdr.read_exact(std::slice::from_mut(&mut obj.{})){};'.format(field.get('name'), error_str));
                elif field.get('type').startswith('char_'):
                    print('    rdr.read_exact(&mut obj.{}[..{}]){};'.format(field.get('name'), field.get('type').split('_')[1], error_str));
                elif field.get('type') == 'u48_t':
                    print('    let mut the_u48 = [0u8; 6];')
                    print('    rdr.read_exact(&mut the_u48[..6]){};'.format(error_str))
                    print('    obj.{} = u48_to_u64(&the_u48);'.format(field.get('name')));
                elif type_map[field.get('type')] in ['u16', 'u32', 'u64']:
                    print('    obj.{} = rdr.read_{}::<BigEndian>(){};'.format(field.get('name'), type_map[field.get('type')], error_str))
                elif field.get('type')[0] == 'e':
                    print('    rdr.read_exact(std::slice::from_mut(&mut obj.{}.0)){};'.format(field.get('name'), error_str))
                else:
                    print('    // TODO: parse {} of type {}'.format(field.get('name'), field.get('type')))

        print('  pub fn from_bytes(bytes: &[u8]) -> Option<({},usize)> {{'.format(struct_name(item.get('name'))))
        print('    if bytes.len() < {}_SIZE {{'.format(item.get('name').upper()))
        print('      return None;')
        print('    }')
        print('    let mut rdr = Cursor::new(bytes);')
        do_struct_parser('.unwrap()')
        print('    Some((obj, {}_SIZE))'.format(item.get('name').upper()))
        print('  }')

        print('  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<{}> {{'.format(struct_name(item.get('name'))))
        do_struct_parser('?')
        print('    Ok(obj)')
        print('  }')
        print('}')

        def maybe_transform(field):
            if field.get('type').startswith('char') and field.get('type') != 'char_t':
                return 'String::from_utf8_lossy(&self.{}[..])'.format(field.get('name'))
            # elif field.get('type') == 'u48_t':
                # return 'u64::from_be(self.' + field.get('name') + ')'
            # elif type_map[field.get('type')] in ['u16', 'u32', 'u64']:
                # return type_map[field.get('type')] + '::from_be(self.' + field.get('name') + ')'
            return 'self.' + field.get('name')

        print('impl fmt::Display for {} {{'.format(struct_name(item.get('name'))))
        print('  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {')
        print('    write!(f, "{}('.format(struct_name(item.get('name'))), end='')
        for i, field in enumerate(item):
            if i != 0: 
                print(',', end='')
            print('{}:{{}}'.format(field.get('name')), end='')
        print(')"', end='')
        for field in item:
            print(', {}'.format(maybe_transform(field)), end='')
        print(')')
        print('  }')
        print('}')


def do_functions(xml):
    struct_name = lambda x: ''.join(map(lambda n: n[0].upper() + n[1:], x.split('_')))
    print('pub trait ItchHandler {')
    for item in xml.find('Structs'):
        if item.get('id') is None:
            continue
        print('  fn on_{}(&mut self, _msg: {}) {{}}'.format(item.get('name'), struct_name(item.get('name'))))
    print('}')
    print('')

    print('pub fn crack_message<T: ItchHandler>(msg: &[u8], handler: &mut T) {')
    print('  let tipe = msg[0];')
    print('  let mut rdr = Cursor::new(msg);')
    print('  match tipe {')
    for item in xml.find('Structs'):
        if item.get('id') is None:
            continue
        print('    {}::TYPE => {{'.format(struct_name(item.get('name'))))
        print('      //let msg = unsafe {{ std::ptr::read(msg[..].as_ptr() as *const {}) }};'.format(struct_name(item.get('name'))))
        print('      if let Ok(msg) = {}::from_cursor(&mut rdr) {{'.format(struct_name(item.get('name'))))
        print('        handler.on_{}(msg);'.format(item.get('name')))
        print('      }')
        print('      else { unreachable!() }') # FIXME
        print('    },')
    print('    _ => { panic!("unknown type!") },')
    print('  }')
    print('}')

    for item in xml.find('Structs'):
        if item.get('id') is None:
            continue
        print('pub fn write_{}(wrt: &mut Cursor<&mut [u8]>, '.format(item.get('name')), end='')
        print(', '.join(map(lambda f: '{}: {}'.format(f.get('name'), type_map[f.get('type')]), filter(lambda f: f.get('name') != 'message_type', item))), end='')
        print(') -> std::io::Result<()> {')
        print('  let start_pos = wrt.position();');
        for field in item:
            if field.get('name') == 'message_type':
                print('  let tipe = {}::TYPE;'.format(struct_name(item.get('name'))))
                print('  wrt.write_all(std::slice::from_ref(&tipe))?;')
            elif field.get('type') == 'u48_t':
                print('  wrt.write_all(&u64::to_be_bytes({})[2..])?;'.format(field.get('name')))
            elif type_map[field.get('type')] in ['u16','u32','u64']:
                print('  wrt.write_{}::<BigEndian>({})?;'.format(type_map[field.get('type')], field.get('name')))
            elif field.get('type') == 'char_t':
                print('  wrt.write_all(std::slice::from_ref(&{}))?;'.format(field.get('name')))
            elif field.get('type').startswith('char_'):
                print('  wrt.write_all(&{}[..{}])?;'.format(field.get('name'), field.get('type').split('_')[1]))
            elif field.get('type')[0] == 'e':
                print('  wrt.write_all(std::slice::from_ref(&{}.0))?;'.format(field.get('name')))
            else:
                print('  // TODO write({}) type {}'.format(field.get('name'), field.get('type')))

        print('  assert_eq!(wrt.position() - start_pos, {}_SIZE as u64);'.format(item.get('name').upper()));
        print('  Ok(())')
        print('}')
        print('')
        print('pub fn write_{}_struct(wrt: &mut Cursor<&mut [u8]>, msg: {}) -> std::io::Result<()> {{'.format(item.get('name'), struct_name(item.get('name'))))
        print('  let start_pos = wrt.position();');
        for field in item:
            if field.get('name') == 'message_type':
                print('  let tipe = {}::TYPE;'.format(struct_name(item.get('name'))))
                print('  wrt.write_all(std::slice::from_ref(&tipe))?;')
            elif field.get('type') == 'u48_t':
                print('  wrt.write_all(&u64::to_be_bytes(msg.{})[2..])?;'.format(field.get('name')))
            elif type_map[field.get('type')] in ['u16','u32','u64']:
                print('  wrt.write_{}::<BigEndian>(msg.{})?;'.format(type_map[field.get('type')], field.get('name')))
            elif field.get('type') == 'char_t':
                print('  wrt.write_all(std::slice::from_ref(&msg.{}))?;'.format(field.get('name')))
            elif field.get('type').startswith('char_'):
                print('  wrt.write_all(&msg.{}[..{}])?;'.format(field.get('name'), field.get('type').split('_')[1]))
            elif field.get('type')[0] == 'e':
                print('  wrt.write_all(std::slice::from_ref(&msg.{}.0))?;'.format(field.get('name')))
            else:
                print('  // TODO write({}) type {}'.format(field.get('name'), field.get('type')))
        print('  assert_eq!(wrt.position() - start_pos, {}_SIZE as u64);'.format(item.get('name').upper()));
        print('  Ok(())')
        print('}')

def do_utils(xml):
    struct_name = lambda x: ''.join(map(lambda n: n[0].upper() + n[1:], x.split('_')))
    print('')
    print('pub struct Dumper {}')
    print('impl ItchHandler for Dumper {')
    for item in xml.find('Structs'):
        if item.get('id') is None:
            continue
        print('  fn on_{}(&mut self, msg: {}) {{ println!("{{}}", msg); }}'.format(item.get('name'), struct_name(item.get('name'))))
    print('}')

    # print('')
    # print('//fn main() {')
    # print('//  let msgbuf = "A\\x00\\x00\\x00\\x00\\x01\\x02\\x03\\x04\\x05\\x06\\x07\\x08\\x10\\x20\\x30\\x40\\x50\\x60\\x70\\x7FB\\x64\\x00\\x00\\x00AMZN    \\x10\\x20\\x30\\x40";')
    # print('//  let mut handler = Dumper{};')
    # print('//  crack_message(msgbuf.as_bytes(), &mut handler);')
    # print('//}')
    # print('')

def main():
    tree = ET.parse(sys.argv[1])
    root = tree.getroot()

    do_header(root)
    do_constants(root)
    do_enums(root)
    do_structs(root)
    do_functions(root)
    do_utils(root)
    
    # print('fn main() {}')

main()


