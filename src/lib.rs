use std::fmt;

#[derive(Debug)]
pub struct Clock{
    mm: u32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m= convert_hm(hours,minutes);

        Self{mm: m}

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let my =self.mm;
        let mut m=minutes+my as i32;
        if m<0 {
            while m<0 { m+=1440;}
        }

        return Self{  mm: m as u32}
    }
}
impl PartialEq for Clock{
    fn eq(&self, other: &Self) -> bool {
        let h1=(self.mm/60)%24;
        let h2=(other.mm/60)%24;
        let m1=self.mm%60;
        let m2=other.mm%60;
        if m1==m2 && h1==h2{
        return true;
        }else{
            return false;
        }
    }
    }


impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", (self.mm / 60) %24, self.mm % 60)
    }
}

pub fn convert_hm(hours:i32, minutes:i32) -> u32{

    let mut tot= hours*60+minutes;
    if tot<0 {
        while tot<0 { tot+=1440;}
    }

   return tot as u32;
}
