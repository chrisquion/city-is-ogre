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

pub struct MenuCursor{
    pub current_selections: Vec<usize>, 
    pub pointing_at_index: usize,
}

impl MenuCursor {
    pub fn new() -> MenuCursor {
        MenuCursor { 
            current_selections: Vec::new(),
            pointing_at_index: 0,
        }
    } 
}

impl Menu {
    pub fn new(items: Vec<MenuItem>, config: MenuConfig) -> Menu {
        let cursor = MenuCursor::new();
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
        let mut items: Vec<MenuItem> = Vec::new();
        items.push(MenuItem::new(String::from("START"), Menu::title_menu_start));
        items.push(MenuItem::new(String::from("QUIT"), Menu::title_menu_start));
        let config: MenuConfig = MenuConfig::new(1, false);
        Menu::new(items, config)
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

#[derive(Clone)]
pub struct MenuItem {
    text: String,
    action: fn() -> Result<(), String>,
}

impl MenuItem {
    pub fn new(text: String, action: fn() -> Result<(), String>) -> MenuItem {
        MenuItem { 
            text,
            action
        }
    }
}