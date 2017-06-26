use std::ops::Range;

pub struct Generator{
    table: [u8; 256],
    i: u8,
    j: u8,
}

impl Generator{
    pub fn new(key: &[u8]) -> Generator{
        let table = gen_table(key);
        Generator {
            table,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for Generator{
    type Item = u8;
    fn next(&mut self) -> Option<u8>{
        self.i += 1;
        let i = self.i as usize;
        self.j += self.table[i];
        let j = self.j as usize;
        // swap
        let tmp = self.table[i];
        self.table[i] = self.table[j];
        self.table[j] = tmp;

        let s = self.table[i] + self.table[j];
        Some(self.table[s as usize])
    }
}

fn gen_table(key: &[u8]) -> [u8; 256]{
    let mut table: [u8; 256] = [0; 256];
    let len = key.len();

    // initialize table
    let r: Range<u8> = 0..256;
    for i in r {
        table[i as usize] = i;
    }

    let mut j = 0u8;
    for i in 0..256 {
        j += table[i] + key[i % len];

        let ju = j as usize;

        let tmp = table[i];
        table[i] = table[ju];
        table[ju] = tmp;
    }

    table
}
