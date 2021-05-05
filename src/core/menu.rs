pub struct Menu {
    pub items: Vec<MenuItem>,
    pub cursor: MenuCursor,
    config: MenuConfig,
}

pub struct MenuConfig {
    n_selectable: usize,
    moveable: bool,
}

impl MenuConfig { 
    pub fn new(n_selectable: usize, moveable: bool) -> MenuConfig {
        MenuConfig {
            n_selectable,
            moveable,
        }
    }
}

pub struct MenuCursor<'m> {
    pub current_selections: &'m [MenuItem], 
    pub pointing_at_index: usize,
}

impl MenuCursor <'m>{
    // `m should be alive as long as the menu is open? 
    // or as the game is running? 
    // or as long as its parent is in game?
    pub fn new(items: &'m Vec<MenuItem>) -> MenuCursor {
        MenuCursor { 
            current_selections: &[items[0]],
            pointing_at_index: 0,
        }
    } 
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, config: MenuConfig) -> Menu {
        let cursor = MenuCursor::new(items);
        Menu {
            items, 
            config,
            cursor
        }
    }

    fn title_menu_start() -> Result<(), String> {
        Ok(())
    }

    fn title_menu_quit() -> Result<(), String> {
        Ok(())
    }

    pub fn default_title() -> Menu {
        let items: Vec<MenuItem> = Vec::new();
        items.push(MenuItem::new(String::from("START"), Menu::title_menu_start));
        items.push(MenuItem::new(String::from("QUIT"), Menu::title_menu_start));
        let config: MenuConfig = MenuConfig::new();
        Menu::new(items, )
    }

    fn cursor_shift_down() -> Result<(), String> {
        Ok(())
    }

    fn cursor_shift_up() -> Result<(), String> {
        Ok(())
    }

    pub fn cursor_select_nth(n: usize) -> Result<(), String> {
        Ok(())
    }
}

struct MenuItem {
    text: String,
    action: fn() -> Result<(), String>,
}