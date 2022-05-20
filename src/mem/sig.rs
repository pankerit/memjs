pub struct Signature {
    pub value: Vec<u8>,
    pub mask: Vec<char>,
}

impl Signature {
    pub fn new(signature: &str) -> Self {
        let mut value: Vec<u8> = Vec::new();
        let mut mask: Vec<char> = Vec::new();
        signature.split(" ").for_each(|x| {
            let m = if x == "?" { '?' } else { 'x' };
            let v = if m == 'x' {
                i32::from_str_radix(x, 16).unwrap() as u8
            } else {
                0
            };
            value.push(v);
            mask.push(m);
        });
        Self { value, mask }
    }

    pub fn scan(&self, buffer: &[u8]) -> Option<u64> {
        let value_size = self.value.len();
        let buffer_size = buffer.len();
        for i in 0..buffer_size {
            if (self.mask[0] == '?' || self.value[0] == buffer[i])
                && value_size <= (buffer_size - i)
            {
                let mut j = 1;
                while j < value_size {
                    if self.mask[j] == '?' || self.value[j] == buffer[i + j] {
                        j += 1;
                    } else {
                        break;
                    }
                }
                if j == value_size {
                    return Some(i as u64);
                }
            }
        }
        return None;
    }
}
