/*
    I've chosen to use this little utility because of its performance in benchmarks being the best, and because it makes it the easiest to specialize to the needs of this project (in terms of both optimizations and code structure.)
    In this case, those needs being a way to have bigger integer sizes that are compatible with bit operations at high speeds.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitSet<const T : usize> {
    pub data: [ u128; T ]
}

impl<const T: usize> BitSet<T> {
    pub fn from_data(data: [ u128; T ]) -> BitSet<T> {
        BitSet {
            data
        }
    }

    pub fn from_element(el: u128) -> BitSet<T> {
        let mut arr = [ 0; T ];
        arr[0] = el;
        BitSet {
            data: arr
        }
    }

    pub fn is_empty(&self) -> bool {
        if T == 1 {
            self.data[0] == 0   
        } else {
            self.data.iter().all(|el| *el == 0)
        }
    }

    pub fn max() -> BitSet<T> {
        BitSet::<T>::from_data([ u128::MAX; T ])
    }

    pub fn new() -> BitSet<T> {
        BitSet::<T>::from_data([ 0; T ])
    }

    pub fn apply(self, rhs: &BitSet<T>, apply: impl Fn((&u128, &u128)) -> u128) -> Self {
        BitSet {
            data: self.data.iter().zip(&rhs.data).map(apply).collect::<Vec<_>>().try_into().unwrap()
        }
    }

    pub fn effect(&mut self, rhs: &BitSet<T>, apply: impl Fn((&u128, &u128)) -> u128) {
        self.data = self.data.iter().zip(&rhs.data).map(apply).collect::<Vec<_>>().try_into().unwrap()
    }

    /*
        Not a well optimized method; avoid using in hot loops.
    */
    pub fn get_bits(&self) -> Vec<u128> {
        let mut bits: Vec<u128> = Vec::with_capacity(128 * T);
        for container in self.data {
            for i in 0..128 {
                bits.push((container >> i) & 1); // Get `i`th bit of `container` and check if it is toggled on (equal to 1)
            }
        }
        bits
    }

    pub fn display(&self, cols: usize, rows: usize) -> String {
        let mut chunks = Vec::<String>::with_capacity(rows);
        for (ind, row) in self.get_bits().chunks(cols).enumerate() {
            if ind == rows {
                break;
            }

            let chunk = row.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
            chunks.push(chunk);
        }
        
        chunks.join("\n")
    }
}