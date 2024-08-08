pub fn menu_logo () -> String {

    let logo = r#"  _____        _       _    _    _   
 |_   _|__  __| |___  | |  (_)__| |_ 
   | |/ _ \/ _` / _ \ | |__| (_-<  _|
   |_|\___/\__,_\___/ |____|_/__/\__|
                                         
    
    "#.to_string();
    logo
}

pub fn menu_show () -> String {

let options = r#"
Add      To add a new item
Remove   To remove a  item
Update   To update a  item
Display  To display items
Complete To mark item as complete
Save     To save the modifications
Exit     To quit application

"#.to_string();

    options
}