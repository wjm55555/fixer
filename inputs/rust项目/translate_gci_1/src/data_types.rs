#[repr(C)]
#[derive(Clone, Debug)]
pub enum VALUE_TYPE {
    VALUE_TYPE_INTEGER = 0,
    VALUE_TYPE_DOUBLE = 1,
    VALUE_TYPE_OBJ = 2,
    VALUE_TYPE_ARR = 3,
}

impl Default for VALUE_TYPE {
    fn default() -> Self {
        VALUE_TYPE::VALUE_TYPE_INTEGER
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct VALUE {
    pub mark: i8,
    pub int_val: i64,
    pub double_val: f64,
    pub obj_val: Option<Box<OBJECT>>,
    pub arr_val: Option<Box<ARRAY>>,
    pub r#type: VALUE_TYPE,
}

impl Default for VALUE {
    fn default() -> Self {
        VALUE {
            mark: 0,
            int_val: 0,
            double_val: 0.0,
            obj_val: None,
            arr_val: None,
            r#type: VALUE_TYPE::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PROPERTY {
    pub key: usize,
    pub val: VALUE,
}

impl Default for PROPERTY {
    fn default() -> Self {
        PROPERTY {
            key: 0,
            val: VALUE::default(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct OBJECT {
    pub properties_len: usize,
    pub properties_cap: usize,
    pub properties: Vec<PROPERTY>,
}

impl Default for OBJECT {
    fn default() -> Self {
        OBJECT {
            properties_len: 0,
            properties_cap: 0,
            properties: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ARRAY {
    pub len: usize,
    pub cap: usize,
    pub values: Vec<VALUE>,
}

impl Default for ARRAY {
    fn default() -> Self {
        ARRAY {
            len: 0,
            cap: 0,
            values: Vec::new(),
        }
    }
}