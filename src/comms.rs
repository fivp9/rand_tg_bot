// struct of available commands

use teloxide::utils::html;

#[derive(Debug)]
pub struct GenPref {
    pub length: usize,
    pub times: usize,
    pub special_characters: SetSpecial,
}

impl GenPref {
    pub fn simple_example() -> &'static str {
        "l15 n4 sc-"
    }

    pub fn about_me() -> String {
        format!("{} : lenght of gen.\
        \n{} : number of gens.\
        \n{} : + to include special symbols;\n\
               - to exlude special symbols;\n\
               +@_ to add '@' and '_' to symbol set;\n\
               -@_ to exlude '@' and '_' from special symbols.",
        html::bold("l"),
        html::bold("n"),
        html::bold("sc"))
    }
}

#[derive(Debug)]
pub enum SetSpecial {
    AllMinus(Option<Vec<u8>>),
    NonePlus(Option<Vec<u8>>),
}


impl Default for GenPref{
    fn default() -> Self {
        Self { length: 15, times: 4, special_characters: SetSpecial::NonePlus(None) }
    }
}

pub struct ParseGenPrefError;

impl std::str::FromStr for GenPref {
    type Err = ParseGenPrefError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prf = GenPref::default();
        for cm in s.split(' ') {
            if let Some(numb) = cm.strip_prefix('l') {
                prf.length = numb.parse::<usize>().map_err(|_| ParseGenPrefError)?;
            } else if let Some(numb) = cm.strip_prefix('n') {
                prf.times = numb.parse::<usize>().map_err(|_| ParseGenPrefError)?;
            } else if cm == "sc+" {
                prf.special_characters = SetSpecial::AllMinus(None);
            } else if let Some(chs) = cm.strip_prefix("sc+"){
                prf.special_characters = SetSpecial::NonePlus(Some(Vec::from(chs)));
            } else if cm == "sc-" {
                prf.special_characters = SetSpecial::NonePlus(None);
            } else if let Some(chs) = cm.strip_prefix("sc-"){
                prf.special_characters = SetSpecial::AllMinus(Some(Vec::from(chs)));
            }
            else {
              return Err(ParseGenPrefError);  
            }
        }

        Ok(prf)
    }
}
