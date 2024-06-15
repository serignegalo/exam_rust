
pub struct One {
    pub first_layer: Option<Two>
}

#[derive(Debug, Clone)]
pub struct Two {
    pub second_layer: Option<Three>
}

#[derive(Debug, Clone)]
pub struct Three {
    pub third_layer: Option<Four>
}

#[derive(Debug, Clone)]
pub struct Four {
    pub fourth_layer: Option<u16>
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        <Option<Two> as Clone>::clone(&self.first_layer)?.second_layer?.third_layer?.fourth_layer
    }
}