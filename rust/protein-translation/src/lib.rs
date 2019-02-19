use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    codon_names: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.codon_names.get(codon).map(|&s| s)
    }

    // Return a list of protein names that correspond to the 'rna' RNA string or
    // None if the RNA string is invalid"
    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        if rna.len() % 3 != 0 {
            // All valid codons are length 3
            return None;
        }
        // Push characters onto a stack until we get enough chars to decide
        // if its a valid codon or not
        let mut names = vec![];
        let mut stack = String::with_capacity(3);
        for ch in rna.chars() {
            stack.push(ch);
            if stack.len() == 3 {
                match self.name_for(&stack) {
                    Some("stop codon") => {
                        stack.clear();
                        break;
                    }
                    Some(name) => {
                        stack.clear();
                        names.push(name);
                    }
                    None => return None,
                }
            }
        }
        if stack.is_empty() {
            Some(names)
        } else {
            None
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut codon_names = HashMap::new();
    for (codon, name) in pairs.into_iter() {
        codon_names.insert(codon, name);
    }
    CodonsInfo { codon_names }
}
