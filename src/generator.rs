use std::num::Wrapping;

pub struct Generator{
    table: [Wrapping<u8>; 256],
    i: Wrapping<u8>,
    j: Wrapping<u8>,
}

impl Generator{
    pub fn new(key: &[u8]) -> Generator{
        let table = gen_table(key);
        Generator {
            table,
            i: Wrapping(0),
            j: Wrapping(0),
        }
    }
}

impl Iterator for Generator{
    type Item = u8;
    fn next(&mut self) -> Option<u8>{
        self.i += Wrapping(1);
        let i = self.i.0 as usize;
        self.j += self.table[i];
        let j = self.j.0 as usize;
        // swap
        let tmp = self.table[i];
        self.table[i] = self.table[j];
        self.table[j] = tmp;

        let s = self.table[i] + self.table[j];
        Some(self.table[s.0 as usize].0)
    }
}

fn gen_table(key: &[u8]) -> [Wrapping<u8>; 256]{
    let mut table: [Wrapping<u8>; 256] = [Wrapping(0); 256];
    let len = key.len();

    // initialize table
    let r = 0..256;
    for i in r {
        table[i as usize] = Wrapping(i as u8);
    }

    let mut j = Wrapping(0u8);
    for i in 0..256 {
        j += table[i] + Wrapping(key[i % len]);

        let ju = j.0 as usize;

        let tmp = table[i];
        table[i] = table[ju];
        table[ju] = tmp;
    }

    table
}
