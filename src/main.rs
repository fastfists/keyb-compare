use std::fmt::Display;

struct Swap {
    from: usize,
    to: usize,
    character: char
}

impl Display for Swap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} <=> {}", self.character, self.from, self.to)
    }
}

struct Keymap {
    keys : [&'static str; 3],
    swaps: Vec<Swap>,
    name: &'static str
}

impl Display for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        write!(f, "{} Keymap\n", self.name)?;
        for item in &self.swaps {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}


fn main() {
    let qwerty = Keymap { 
        keys: [ "qwertyuiop", "asdfghjkl;'", "zxcvbnm,./"],
        swaps: Vec::new(),
        name: "qwerty"
    };
    let colemak_dh = Keymap {
        keys: ["qwfpbjluy;", "arstgmneio'", "zxcdvkh,./"],
        swaps: Vec::new(),
        name: "colemak"
    };

    let dvorak = Keymap {
        keys: ["',.pyfgcrl", "aoeuidhfjs", ";qjkxbmwvz"],
        swaps: Vec::new(),
        name: "dvorak"
    };

    let all_key_maps = [colemak_dh, dvorak];

    for mut map in all_key_maps {
        for n in 0..3 {
            let qwerty_line = qwerty.keys[n];
            // Compare to colemmakDH 
            let map_line = map.keys[n];

            for char in qwerty_line.chars() {
                if map_line.find(char).is_some() {
                    continue;
                }
                // Find what line this does belong on
                for m in 0..3 {
                    if map.keys[m].find(char).is_some() {
                        map.swaps.push(Swap {character: char, from: n, to: m});
                    }
                }
            }
        }
    }
    println!("{:?}", all_key_maps);
}
