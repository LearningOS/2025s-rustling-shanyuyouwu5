// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // This problem is how to deal with the whitespace in the between two worlds.
    let mut isspace = 0;
    let mut lastchar = 0;
    let mut result = String::new();
    for i in input.chars() {
        lastchar = isspace;
        if i == ' ' {
            isspace = 1;
        }
        else {
            isspace = 0;
        }
        
        match (isspace, lastchar) {
            (0,1) =>{
                if result == String::new(){
                    result.push(i);
                }
                else {
                    result.push(' ');
                    result.push(i);
                }
            }
            (1,0) =>{}
            (1,1) =>{}
            (0,0) => {result.push(i);}
            _ => {} 
        }
    }
    return result;
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let mut result = String::from(input);
    result = result + " world!";
    result as String
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // let mut interal = trim_me(input);
    let mut interal = input;
    let mut num = 0;
    let mut result = String::new();
    let mut t = 0;
    let mut world:&str = "";
    for i in interal.chars() {
        if i == ' ' {
            world=(&input[t..num]);
            t = num;
            if world == "cars" {
                world = "balloons";
            }
            else if world == "Cars" {
                world = "Balloons";
            }
            else if world == " cars"  {
                    world = " balloons";
            }
            result.push_str(world);
        }
        else if num == interal.len()-1 {
            world=(&input[t..num+1]);
            if world == "cars" {
                world = "balloons";
            }
            else if world == "Cars" {
                world = "Balloons";
            }
            else if world == " cars"  {
                    world = " balloons";
            }
            result.push_str(world);
        }
        num+=1;
    }
    
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
