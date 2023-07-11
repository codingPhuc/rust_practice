
use std::fmt;
pub struct Clock
{
        hours: i32 , 
        minutes : i32  , 
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut change_hour = hours ;  
     
        let mut change_minutes   =  minutes  ; 
        if change_minutes < 0 
        {
            
            change_hour= change_hour + (change_minutes/60) -1 ; 
            change_minutes  =  60  +  (change_minutes% 60 );
        }
        else if minutes >= 60  
        {
            change_hour= change_hour + (change_minutes/60);
            change_minutes  =  change_minutes % 60 ;
        }
        // if change_minutes.abs() >= 60  
        // {   
            
        //     if   change_minutes  >0  {change_hour  =  change_hour + (minutes/60);   }
        //     change_minutes =  (change_minutes - (change_minutes/60)*60).abs() ;   
        // }
        // change_hour = change_hour  -  (24*(change_hour/24)) ; 
        
        if change_hour >=  0  
        {
            change_hour   =  change_hour  %24 ; 
        }
        else 
        {
            change_hour   = 24 + (change_hour  %24 );
        }

        Clock{
            hours : change_hour , 
            minutes: change_minutes
        }


    }

   
    
      pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes) 
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes )
    }
}
fn main ( )
{
    let clock = Clock::new(0, 45).add_minutes(160);
    assert_eq!(clock.to_string(), "03:25");
}