pub static OPTIONS: Opt = Opt {
    
    colors:  ["Red", "Blue", "Black", "White", "Green", "Yellow", "Brown", "Purple", "Pink", "Orange"],
    numbers: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"],
    shapes:  ["Circle", "Rectangle", "Square", "Octagon", "Triangle", "Hectagon", "Line", "Hexagon", "Pentagon", "Diamond"],

};


pub struct Opt<'a>
{
    pub colors: [&'a str; 10],
    pub numbers: [&'a str; 10],
    pub shapes: [&'a str; 10],
}

impl Opt<'_> {
    pub fn get_choices (&self, tof: TypeOfChoices, N: i32) -> Vec<String>
    {
        let mut choices: Vec<String> = vec![];
        match tof {
            TypeOfChoices::Numbers => 
            { 
                for i in 0..N as usize
                {
                    choices.push(self.numbers[i].to_string());
                }
            },
            TypeOfChoices::Colors => 
            { 
                for i in 0..N as usize
                {
                    choices.push(self.colors[i].to_string());
                }
            },
            TypeOfChoices::Shapes =>
            {
                for i in 0..N as usize
                {
                    choices.push(self.shapes[i].to_string());
                }
            },

        }
        choices
    }
}


#[derive(Clone, Debug)]
pub enum TypeOfChoices {
    Colors, Numbers, Shapes
}

impl TypeOfChoices
{
    pub fn convert_type (N: i32) -> TypeOfChoices
    {
        match N
        {
            0 => TypeOfChoices::Colors,
            1 => TypeOfChoices::Numbers,
            2 => TypeOfChoices::Shapes,

            _ => TypeOfChoices::Numbers
        }
    }
}
