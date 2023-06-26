use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    println!("Hello, world!");
}

// Network packets of intergers are always encoded as BigEndian.
#[derive(Default, Debug, PartialEq)]
struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

// Use Vec to represent an array of bytes.
struct DNSQuestion {
    name: Vec<u8>,
    dns_type: u16,
    dns_class: u16,
}

fn header_to_bytes(header: &DNSHeader) -> Vec<u8> {
    let mut result = Vec::new();

    result.write_u16::<BigEndian>(header.id).unwrap();
    result.write_u16::<BigEndian>(header.flags).unwrap();
    result.write_u16::<BigEndian>(header.num_questions).unwrap();
    result.write_u16::<BigEndian>(header.num_answers).unwrap();
    result
        .write_u16::<BigEndian>(header.num_authorities)
        .unwrap();
    result
        .write_u16::<BigEndian>(header.num_additionals)
        .unwrap();

    result
}

fn question_to_bytes(question: &DNSQuestion) -> Vec<u8> {
    let mut result = question.name.clone();
    result.write_u16::<BigEndian>(question.dns_type).unwrap();
    result.write_u16::<BigEndian>(question.dns_class).unwrap();
    result
}

fn encode_dns_name(domain_name: &str) -> Vec<u8> {
    let mut encoded = Vec::new();

    for part in domain_name.as_bytes().split(|&byte| byte == b'.') {
        encoded.push(part.len() as u8);
        encoded.extend_from_slice(part)
    }
    encoded.push(0x00);
    encoded
}
