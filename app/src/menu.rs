use crate::translation::Translation;

pub fn menu_logo() -> String {
    r#"
  _____        _       _    _    _   
 |_   _|__  __| |___  | |  (_)__| |_ 
   | |/ _ \/ _` / _ \ | |__| (_-<  _|
   |_|\___/\__,_\___/ |____|_/__/\__|
                                         
    
"#
    .to_string()
}

pub fn menu_show(translation: &Translation) -> String {
    format!(
        "{}{}{}{}{}{}{}",
        translation.get_message("menu.add"),
        translation.get_message("menu.remove"),
        translation.get_message("menu.update"),
        translation.get_message("menu.display"),
        translation.get_message("menu.complete"),
        translation.get_message("menu.save"),
        translation.get_message("menu.exit")
    )
}
