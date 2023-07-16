


//start of use commands 


//end of use commands 


//start of structs 

pub struct CompiledScripts{

    pub internal_script:String,
    pub path:String,

}


//end of structs


//start of modules

pub mod whiskey_python_file_paths {

    pub const WHISKEY_PYTHON_FOLDER_NAME:&'static str="WHISKEY_PYTHON_DONOTEDIT";
    pub const WHISKEY_PYTHON_EXTERNAL_SCRIPT_NAME:&'static str="external_whiskey_python_script.py";
    pub const WHISKEY_PYTHON_INTERNAL_SCRIPT_NAME:&'static str="internal_whiskey_python_script.py";
    

}

pub mod general_functions {
    
    macro_rules!  add_forward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"/"
            }
            
        };
    }
    macro_rules!  add_backward_slash_to_path{
        ($a:expr) => {
            {
                String::from($a)+"\\"
            }
            
        };
    }

    pub fn check_then_add_slash_to_path(path:&str,forward_slash:bool)->String{
        let mut new_path:String=String::from(path);
        match new_path.as_bytes()[new_path.len()-1] as char{
            '/'=>return new_path,
            '\\'=>return new_path,
            _=>{}
        }
        new_path=match forward_slash{
            true=>add_forward_slash_to_path!(new_path),
            false=>add_backward_slash_to_path!(new_path)
        };
        
        return new_path

    }
    /*
    this function checks what kind of slashes a string that contains a path uses, returns true for a forward slash
    returns false for a back slash
    */
    pub fn check_for_slash_type(path:&str)->bool{
        return path.contains("/");
    }
    
    //spawns a file, the file path should be in the first parameter, and the second parameter
    //should be the data to write to the file
    pub fn create_then_write_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        use std::io::Write;
        use std::fs::File;
        

        File::create(path)?.write_all(contents.as_bytes())?;
        return Ok(());
    
    }
    
    // this function creates the directory meant to contain the compiled whiskey python 
    pub fn create_whiskey_python_directory(path:&str)->Result<(),std::io::Error>{
        use super::whiskey_python_file_paths::*;
        let whiskey_python_directory:String=check_then_add_slash_to_path(
            path, 
            check_for_slash_type(path))+WHISKEY_PYTHON_FOLDER_NAME;

        

        
        std::fs::create_dir(whiskey_python_directory)?;
        
        return Ok(());    

    }
    //this function creates the compiled internal python file inside the main whiskey_python_files
    //directory then writes the contents to it 
    pub fn create_write_whiskey_python_internal_file(path:&str,contents:&str)->Result<(),std::io::Error>{
        
        use super::whiskey_python_file_paths::*;

        let mut current_path:String = add_whiskey_files_dir_to_path(path);
            
        
        

       current_path = add_whiskey_internal_script_name_to_path(&current_path);       

        create_then_write_file(
            &current_path                              
            , 
            contents)?;
        return Ok(());

    }
    
    //creates and writes the necessary compiled files for whiskey python 
    pub fn create_write_whiskey_python_files(path:&str,internal_script_contents:&str)->Result<(),std::io::Error>{
        
        create_whiskey_python_directory(path)?;

        create_write_whiskey_python_internal_file(path, internal_script_contents)?;

        return Ok(());

    }
    //adds the whiskey python directory name to the provided path str
    pub fn add_whiskey_files_dir_to_path(path:&str)->String{
        
        use super::whiskey_python_file_paths::*;

        let current_path:String = 
            check_then_add_slash_to_path(
                path,
                check_for_slash_type(path)) + WHISKEY_PYTHON_FOLDER_NAME;

        return current_path;

    }

    //adds the whiskey internal script name to the provided path str
    pub fn add_whiskey_internal_script_name_to_path(path:&str)->String{

        use super::whiskey_python_file_paths::*;

        let current_path:String =
           check_then_add_slash_to_path(
            path,
            check_for_slash_type(path))
            +WHISKEY_PYTHON_INTERNAL_SCRIPT_NAME;

        return current_path;
    }

}

pub mod whiskey_python_parsing {
    
    const WHISKEY_COMMAND_IDENTIFIER:char='$';


    pub mod command_types{
        const EXTERNAL_PYTHON_SCRIPT:&'static str="EXTERNAL";  
    }

    //this function parses a single wiskey command, returns a list of 3 objects instead of a box.
    //refer to the commets before the parse_whiskey_commands function for more details
    pub fn parse_single_whiskey_command(text:&str)->Result<[String;3],std::io::Error>{

        use std::io::ErrorKind::InvalidData;

        let mut parsed:[String;3]=[String::new(),String::new(),String::new()];

        let mut command_type:String=String::new();
        let mut command_name:String=String::new();
        let mut command_parameters:String=String::new();

        let mut read_command_type:bool=true;
        let mut read_command_name:bool=false;
        let mut read_command_parameters:bool=false;
        
        let mut command_type_done:bool=false;
        let mut command_name_done:bool=false;

        for character in text.chars(){
            
            if read_command_type{
                
                if character != ' '{ 

                    command_type.push(character);
                    
                }

                else{
                    read_command_type=false;
                    command_type_done=true;
                    read_command_name=true;
                }
            }

            else if read_command_name{
                
                if character != ' '{ 

                    command_name.push(character);
                    
                }

                else{
                    read_command_name=false;
                    read_command_type=false;
                    command_name_done=true;
                    read_command_parameters=true;
                }

            }
            else if read_command_parameters {
                
                command_parameters.push(character);
    
            }
        }

        if command_parameters == String::from("") || command_parameters == String::new() {
            command_name_done=true;

        }

        

        parsed[0] = command_type;
        parsed[1] = command_name;
        parsed[2] = command_parameters;
        
        

        if command_name_done != true{
            return Err(
                std::io::Error::new(
                    InvalidData,
                    "whiskey command not properly structured (no command type)"
                    )
                );
        }

        if command_type_done != true{
            return Err(
                std::io::Error::new(
                    InvalidData,
                    "whiskey command not properly structured (no command name)"
                    )
                );
        }
        
        return Ok(parsed);
    }

    //this function parses the whiskey commands out of the given text and returns it in the vector
    /*
    the format at which the whiskey commands are returned to is of the following 

    a parsed whiskey command is arranged in three objects inside the vector, first object is the command type, the second object
    is the command name. The third is the parameters passed into the whiskey command.
    
    
    */
    pub fn parse_whiskey_commands(text:&str)->Result<Vec<String>,std::io::Error>{

        let mut parsed:Vec<String> = Vec::new();
        
        let mut current_whiskey_command:String = String::new();
        let mut script:String = String::new();

        let mut read_until_new_line:bool = false;
        let mut finished_with_new_line:bool = false;
        let mut read_script:bool = false;
        let mut read_for_end_bracket = true;


        for character in text.chars(){
            
            if read_until_new_line == false && read_script == false {

                if character == WHISKEY_COMMAND_IDENTIFIER{
                    read_until_new_line=true;
                }
            }
            
            else if read_until_new_line && read_script == false{

                
                if character == '{'{ 
                    read_until_new_line=false;
                    read_script=true;
                    

                }
                else if character != '\n' {
                    finished_with_new_line=false;
                    current_whiskey_command.push(character);
                }
                else {

                    read_until_new_line=false;
                    
                    let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
                    parsed.push(single_command_buffer[0].clone());
                    parsed.push(single_command_buffer[1].clone());
                    parsed.push(single_command_buffer[2].clone());
                    finished_with_new_line=true;
                    current_whiskey_command=String::new();
                }

            }

            else if read_script {
            
                if character == '}' && read_for_end_bracket{
                    
                    read_script = false;

                    let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
                    parsed.push(single_command_buffer[0].clone());
                    parsed.push(single_command_buffer[1].clone());
                    parsed.push(script.clone());
                    finished_with_new_line = true;
                }
                else{
                    
                    script.push(character);
                    if character == '"' || character == '\'' {
                        
                        read_for_end_bracket = !read_for_end_bracket; //flips the bool value
                        
                    }

                }
    
            }



        }
        
        if finished_with_new_line != true{
            
            let single_command_buffer=parse_single_whiskey_command(&current_whiskey_command)?;
            parsed.push(single_command_buffer[0].clone());
            parsed.push(single_command_buffer[1].clone());
            parsed.push(single_command_buffer[2].clone());

        }
        

        return Ok(parsed);

    }


}

pub mod init_whiskey_python{
    
    use super::*;

    //constructs the compiled files struct from the function parameters 
    pub fn define_compiled_files_struct(path:&str,internal_script_contents:&str)->CompiledScripts{
            

        return CompiledScripts { internal_script:internal_script_contents.to_string(), path:path.to_string()}

    }

    //creates all the neccessary files for whiskey_python
    pub fn spawn_whiskey_python_files(compiled_scripts:&CompiledScripts)->Result<(),std::io::Error>{
        
        
        use std::io::Error;
        use std::io::ErrorKind::*;
        use std::path::Path;    
        use general_functions::*;

        if Path::new(&compiled_scripts.path).exists() == false {
            
            return Err(
                Error::new(
                    NotFound,
                    "directory does not exist"
                    )

                );

        }
        
        create_write_whiskey_python_files(&compiled_scripts.path, &compiled_scripts.internal_script)?;


        return Ok(());
    }
    //this function removes all of the whiskey python compiled files
    pub fn wipe_whiskey_python_files(compiled_scripts:&CompiledScripts)->Result<(),std::io::Error> {
        
        use general_functions::*;

        std::fs::remove_dir_all(
            add_whiskey_files_dir_to_path(&compiled_scripts.path)
                                )?;
        return Ok(());
        
    }

    //this function returns a bool if the whiskey python compiled files exist 
    pub fn check_if_whiskey_python_files_exist(compiled_scripts:&CompiledScripts)->bool{
        
        use super::general_functions::*;
        


        return std::path::Path::new(
            &add_whiskey_files_dir_to_path(
                &compiled_scripts.path)).exists();
        
    }
    
    //this functions wipes the whiskey python compiled files if they exist
    pub fn check_wipe_whiskey_python_files(compiled_scripts:&CompiledScripts){
        
        if check_if_whiskey_python_files_exist(&compiled_scripts) {
            
            wipe_whiskey_python_files(&compiled_scripts).unwrap();
        
        }
    }
}



//end of modules
