#[derive(Debug)]
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

        Self{
            hours : change_hour , 
            minutes: change_minutes
        }


    }

    pub fn add_minutes(&self, minutes: i32) -> Self { 
        Clock::new(self.hours , self.minutes + minutes)  
        
    }
    fn to_string(&self) -> String
    {   
        let mut str_hours =  self.hours.to_string()   ; 
        let mut str_minutes=  self.minutes.to_string() ; 
        if self.hours < 10   
        { 
            str_hours = "0".to_string() + &str_hours ; 
        }
        if  self.minutes <10  
        {
            str_minutes = "0".to_string() + &str_minutes  ; 
        }
        format!("{}:{}", str_hours , str_minutes )   
    }
}

fn main() 
{
    let  clock   =  Clock::new(1, 2);
    println!("rect1 is {:?}", clock.to_string());

    
}