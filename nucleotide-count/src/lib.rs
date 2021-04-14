use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    } else if dna.len() == 0 { 
        return Ok(0);
    } else {
        let mut count = 0;
        for curr_char in dna.chars() {
            match curr_char  {
                'A' => count += 1,
                'C' => count += 1,
                'G' => count += 1,
                'T' => count += 1,
                _ => { return Err(curr_char); }
            }
        }

        return Ok(count);
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map: HashMap<char, usize> = HashMap::new();

    map.insert('A', count('A', dna)?);
    map.insert('C', count('C', dna)?);
    map.insert('G', count('G', dna)?);
    map.insert('T', count('T', dna)?);

    return Ok(map);
}
