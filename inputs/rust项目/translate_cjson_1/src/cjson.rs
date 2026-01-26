use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64;
use std::fmt;
use std::mem;
use std::sync::{Mutex, OnceLock};

pub const CJSON_VERSION_MAJOR: i32 = 1;
pub const CJSON_VERSION_MINOR: i32 = 7;
pub const CJSON_VERSION_PATCH: i32 = 19;

pub const cJSON_Invalid: i32 = 0;
pub const cJSON_False: i32 = 1 << 0;
pub const cJSON_True: i32 = 1 << 1;
pub const cJSON_NULL: i32 = 1 << 2;
pub const cJSON_Number: i32 = 1 << 3;
pub const cJSON_String: i32 = 1 << 4;
pub const cJSON_Array: i32 = 1 << 5;
pub const cJSON_Object: i32 = 1 << 6;
pub const cJSON_Raw: i32 = 1 << 7;
pub const cJSON_IsReference: i32 = 256;
pub const cJSON_StringIsConst: i32 = 512;

pub const CJSON_NESTING_LIMIT: usize = 1000;
pub const CJSON_CIRCULAR_LIMIT: usize = 10000;

pub type cJSON_bool = i32;

#[repr(C)]
#[derive(Default, Clone)]
pub struct cJSON {
    pub next: Option<Box<cJSON>>,
    pub prev: Option<Box<cJSON>>,
    pub child: Option<Box<cJSON>>,
    pub type_: i32,
    pub valuestring: Option<String>,
    pub valueint: i32,
    pub valuedouble: f64,
    pub string: Option<String>,
}

impl cJSON {
    pub fn new() -> Self {
        Self::default()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<fn(usize) -> Vec<u8>>,
    pub free_fn: Option<fn(Vec<u8>)>,
}

impl Default for cJSON_Hooks {
    fn default() -> Self {
        Self {
            malloc_fn: None,
            free_fn: None,
        }
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct error {
    pub json: Option<String>,
    pub position: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct internal_hooks {
    pub allocate: fn(usize) -> Vec<u8>,
    pub deallocate: fn(Vec<u8>),
    pub reallocate: Option<fn(Vec<u8>, usize) -> Vec<u8>>,
}

impl Default for internal_hooks {
    fn default() -> Self {
        fn default_alloc(size: usize) -> Vec<u8> {
            vec![0u8; size]
        }
        fn default_dealloc(_v: Vec<u8>) {}
        Self {
            allocate: default_alloc,
            deallocate: default_dealloc,
            reallocate: None,
        }
    }
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct parse_buffer {
    pub content: Option<String>,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub hooks: internal_hooks,
}

#[repr(C)]
#[derive(Default, Clone)]
pub struct printbuffer {
    pub buffer: Vec<u8>,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub noalloc: cJSON_bool,
    pub format: cJSON_bool,
    pub hooks: internal_hooks,
}

static GLOBAL_ERROR: OnceLock<Mutex<error>> = OnceLock::new();
static GLOBAL_HOOKS: OnceLock<Mutex<internal_hooks>> = OnceLock::new();

fn global_error_instance() -> &'static Mutex<error> {
    GLOBAL_ERROR.get_or_init(|| Mutex::new(error::default()))
}

fn global_hooks_instance() -> &'static Mutex<internal_hooks> {
    GLOBAL_HOOKS.get_or_init(|| Mutex::new(internal_hooks::default()))
}

pub fn init_globals() {
    let _ = global_error_instance();
    let _ = global_hooks_instance();
}

pub fn static_strlen(s: &str) -> usize {
    s.len()
}

pub fn cjson_min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

pub fn isinf_fallback(d: f64) -> bool {
    d.is_infinite()
}

pub fn isnan_fallback(d: f64) -> bool {
    d.is_nan()
}

pub fn NAN_fallback() -> f64 {
    f64::NAN
}

pub fn tolower_u8(c: u8) -> u8 {
    (c as char).to_ascii_lowercase() as u8
}

pub fn case_insensitive_strcmp(string1: Option<&[u8]>, string2: Option<&[u8]>) -> i32 {
    if string1.is_none() || string2.is_none() {
        return 1;
    }
    let s1 = string1.unwrap();
    let s2 = string2.unwrap();

    if std::ptr::eq(s1.as_ptr(), s2.as_ptr()) {
        return 0;
    }

    let mut i = 0usize;
    loop {
        if i >= s1.len() || i >= s2.len() {
            break;
        }
        let a = tolower_u8(s1[i]);
        let b = tolower_u8(s2[i]);
        if a != b {
            return (a as i32) - (b as i32);
        }
        if s1[i] == 0 {
            return 0;
        }
        i += 1;
    }

    if s1.len() == s2.len() {
        0
    } else {
        let a = *s1.get(i).unwrap_or(&0);
        let b = *s2.get(i).unwrap_or(&0);
        (a as i32) - (b as i32)
    }
}

pub fn cJSON_GetErrorPtr() -> Option<&'static str> {
    let guard = global_error_instance().lock().unwrap();
    if let Some(ref s) = guard.json {
        let pos = guard.position;
        if pos < s.len() {
            Some(&s[pos..])
        } else {
            Some("")
        }
    } else {
        None
    }
}

pub fn cJSON_GetStringValue(item: Option<&cJSON>) -> Option<&str> {
    if item.is_none() {
        return None;
    }
    let it = item.unwrap();
    if !cJSON_IsString(Some(it)) {
        return None;
    }
    match &it.valuestring {
        Some(s) => Some(s.as_str()),
        None => None,
    }
}

pub fn cJSON_GetNumberValue(item: Option<&cJSON>) -> f64 {
    if item.is_none() {
        return NAN_fallback();
    }
    let it = item.unwrap();
    if !cJSON_IsNumber(Some(it)) {
        return NAN_fallback();
    }
    it.valuedouble
}

pub fn cJSON_Version() -> String {
    format!("{}.{}.{}", CJSON_VERSION_MAJOR, CJSON_VERSION_MINOR, CJSON_VERSION_PATCH)
}

pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut gh = global_hooks_instance().lock().unwrap();
    // Reset to defaults
    *gh = internal_hooks::default();
    if let Some(h) = hooks {
        // If provided, we do not have direct mapping to the C function pointers.
        // Keep default internal hooks; this function intentionally does minimal mapping.
        let _ = h;
    }
}

pub fn cJSON_New_Item(_hooks: Option<&internal_hooks>) -> Option<Box<cJSON>> {
    Some(Box::new(cJSON::new()))
}

pub fn cJSON_Delete(item: Option<Box<cJSON>>) {
    // In Rust, dropping will free automatically
    // Accept Option<Box<cJSON>> and let it be dropped when function ends
    let _ = item;
}

pub fn get_decimal_point() -> u8 {
    b'.'
}

pub fn cJSON_SetNumberHelper(object: Option<&mut cJSON>, number: f64) -> f64 {
    if let Some(obj) = object {
        if number >= (i32::MAX as f64) {
            obj.valueint = i32::MAX;
        } else if number <= (i32::MIN as f64) {
            obj.valueint = i32::MIN;
        } else {
            obj.valueint = number as i32;
        }
        obj.valuedouble = number;
        number
    } else {
        number
    }
}

pub fn cJSON_SetValuestring(object: Option<&mut cJSON>, valuestring: Option<&str>) -> Option<String> {
    if object.is_none() {
        return None;
    }
    let obj = object.unwrap();
    if (obj.type_ & cJSON_String) == 0 || (obj.type_ & cJSON_IsReference) != 0 {
        return None;
    }
    if obj.valuestring.is_none() || valuestring.is_none() {
        return None;
    }
    let v1 = valuestring.unwrap();
    let v2 = obj.valuestring.as_ref().unwrap().as_str();

    if v1.len() <= v2.len() {
        // Check overlapping not applicable in Rust strings
        obj.valuestring = Some(v1.to_string());
        return obj.valuestring.clone();
    }
    obj.valuestring = Some(v1.to_string());
    obj.valuestring.clone()
}

pub fn cJSON_strdup(string: Option<&str>, _hooks: Option<&internal_hooks>) -> Option<String> {
    string.map(|s| s.to_string())
}

pub fn can_read(buffer: &parse_buffer, size: usize) -> bool {
    buffer.content.is_some() && (buffer.offset + size) <= buffer.length
}

pub fn can_access_at_index(buffer: &parse_buffer, index: usize) -> bool {
    buffer.content.is_some() && (buffer.offset + index) < buffer.length
}

pub fn cannot_access_at_index(buffer: &parse_buffer, index: usize) -> bool {
    !can_access_at_index(buffer, index)
}

pub fn buffer_at_offset(buffer: &parse_buffer) -> Option<&[u8]> {
    buffer.content.as_ref().map(|s| s.as_bytes().get(buffer.offset..).unwrap_or(&[]))
}

pub fn parse_number(item: Option<&mut cJSON>, input_buffer: Option<&mut parse_buffer>) -> cJSON_bool {
    if item.is_none() || input_buffer.is_none() {
        return 0;
    }
    let obj = item.unwrap();
    let buf = input_buffer.unwrap();
    if buf.content.is_none() {
        return 0;
    }
    // Simplified parser: attempt to parse from current offset until non-number char
    let content = buf.content.as_ref().unwrap().as_bytes();
    let mut i = buf.offset;
    if i >= content.len() {
        return 0;
    }
    let mut end = i;
    let mut seen = false;
    while end < content.len() {
        let c = content[end];
        match c {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' | b'.' => {
                seen = true;
                end += 1;
            }
            _ => break,
        }
    }
    if !seen {
        return 0;
    }
    let slice = &content[i..end];
    if let Ok(s) = std::str::from_utf8(slice) {
        if let Ok(num) = s.parse::<f64>() {
            obj.valuedouble = num;
            if num >= (i32::MAX as f64) {
                obj.valueint = i32::MAX;
            } else if num <= (i32::MIN as f64) {
                obj.valueint = i32::MIN;
            } else {
                obj.valueint = num as i32;
            }
            obj.type_ = cJSON_Number;
            buf.offset = end;
            return 1;
        }
    }
    0
}

pub fn ensure(_p: Option<&mut printbuffer>, _needed: usize) -> Option<&mut [u8]> {
    // Simplified: not using raw buffers; return None to indicate failure where appropriate
    None
}

pub fn update_offset(_buffer: Option<&mut printbuffer>) {}

pub fn compare_double(a: f64, b: f64) -> cJSON_bool {
    let max_val = a.abs().max(b.abs());
    if (a - b).abs() <= max_val * f64::EPSILON {
        1
    } else {
        0
    }
}

pub fn print_number(_item: Option<&cJSON>, _output_buffer: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn parse_hex4(_input: Option<&[u8]>) -> u32 {
    0
}

pub fn utf16_literal_to_utf8(_input_pointer: Option<&[u8]>, _input_end: Option<&[u8]>, _output_pointer: Option<&mut Vec<u8>>) -> u8 {
    0
}

pub fn parse_string(item: Option<&mut cJSON>, input_buffer: Option<&mut parse_buffer>) -> cJSON_bool {
    if item.is_none() || input_buffer.is_none() {
        return 0;
    }
    let obj = item.unwrap();
    let buf = input_buffer.unwrap();
    if buf.content.is_none() {
        return 0;
    }
    let s = buf.content.as_ref().unwrap();
    let bytes = s.as_bytes();
    if buf.offset >= bytes.len() || bytes[buf.offset] != b'\"' {
        return 0;
    }
    let mut i = buf.offset + 1;
    let mut out = String::new();
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\"' {
            buf.offset = i + 1;
            obj.type_ = cJSON_String;
            obj.valuestring = Some(out);
            return 1;
        } else if c == b'\\' {
            if i + 1 >= bytes.len() {
                return 0;
            }
            let n = bytes[i + 1];
            match n {
                b'b' => out.push('\u{0008}'),
                b'f' => out.push('\u{000C}'),
                b'n' => out.push('\n'),
                b'r' => out.push('\r'),
                b't' => out.push('\t'),
                b'\"' => out.push('\"'),
                b'\\' => out.push('\\'),
                b'/' => out.push('/'),
                b'u' => {
                    // Simplified: skip unicode escape
                    // Attempt to parse next 4 hex digits
                    let mut hex = String::new();
                    let mut ok = true;
                    for j in 0..4 {
                        if i + 2 + j >= bytes.len() {
                            ok = false;
                            break;
                        }
                        hex.push(bytes[i + 2 + j] as char);
                    }
                    if ok {
                        if let Ok(code) = u16::from_str_radix(&hex, 16) {
                            if let Some(ch) = std::char::from_u32(code as u32) {
                                out.push(ch);
                            }
                            i += 4; // additional skip
                        }
                    }
                }
                _ => return 0,
            }
            i += 2;
        } else {
            out.push(c as char);
            i += 1;
        }
    }
    0
}

pub fn print_string_ptr(_input: Option<&str>, _output_buffer: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn print_string(_item: Option<&cJSON>, _p: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn buffer_skip_whitespace(buffer: Option<&mut parse_buffer>) -> Option<&mut parse_buffer> {
    if buffer.is_none() {
        return None;
    }
    let buf = buffer.unwrap();
    if buf.content.is_none() {
        return None;
    }
    while can_access_at_index(buf, 0) {
        if let Some(arr) = buffer_at_offset(buf) {
            if arr.len() == 0 { break; }
            if arr[0] <= 32 {
                buf.offset += 1;
                continue;
            }
        }
        break;
    }
    Some(buf)
}

pub fn skip_utf8_bom(buffer: Option<&mut parse_buffer>) -> Option<&mut parse_buffer> {
    if buffer.is_none() {
        return None;
    }
    let buf = buffer.unwrap();
    if buf.content.is_none() || buf.offset != 0 {
        return None;
    }
    if buf.length >= 3 {
        if let Some(bytes) = buffer_at_offset(buf) {
            if bytes.len() >= 3 && &bytes[0..3] == b"\xEF\xBB\xBF" {
                buf.offset += 3;
            }
        }
    }
    Some(buf)
}

pub fn cJSON_ParseWithOpts(value: Option<&str>, return_parse_end: Option<&mut Option<&str>>, require_null_terminated: cJSON_bool) -> Option<Box<cJSON>> {
    if value.is_none() {
        return None;
    }
    let s = value.unwrap();
    let buffer_length = s.len();
    cJSON_ParseWithLengthOpts(Some(s), buffer_length, return_parse_end, require_null_terminated)
}

pub fn cJSON_ParseWithLengthOpts(value: Option<&str>, buffer_length: usize, return_parse_end: Option<&mut Option<&str>>, require_null_terminated: cJSON_bool) -> Option<Box<cJSON>> {
    init_globals();
    let mut buffer = parse_buffer {
        content: value.map(|v| v.to_string()),
        length: buffer_length,
        offset: 0,
        depth: 0,
        hooks: internal_hooks::default(),
    };
    let mut item = cJSON_New_Item(Some(&buffer.hooks));
    if item.is_none() {
        return None;
    }
    // Simplified: do minimal parse: if value is "null" return null cJSON, if number return number, else string
    if let Some(ref s) = buffer.content {
        let trimmed = s.trim();
        if trimmed.starts_with("null") {
            if let Some(mut it) = item {
                it.type_ = cJSON_NULL;
                if let Some(rpe) = return_parse_end {
                    *rpe = Some(&s[4..]);
                }
                return Some(it);
            }
        }
        if trimmed.starts_with("true") {
            if let Some(mut it) = item {
                it.type_ = cJSON_True;
                it.valueint = 1;
                if let Some(rpe) = return_parse_end {
                    *rpe = Some(&s[4..]);
                }
                return Some(it);
            }
        }
        if trimmed.starts_with("false") {
            if let Some(mut it) = item {
                it.type_ = cJSON_False;
                it.valueint = 0;
                if let Some(rpe) = return_parse_end {
                    *rpe = Some(&s[5..]);
                }
                return Some(it);
            }
        }
        if trimmed.starts_with('\"') && trimmed.ends_with('\"') {
            if let Some(mut it) = item {
                it.type_ = cJSON_String;
                it.valuestring = Some(trimmed[1..trimmed.len()-1].to_string());
                return Some(it);
            }
        }
        // try parse number
        if let Ok(num) = trimmed.parse::<f64>() {
            if let Some(mut it) = item {
                it.type_ = cJSON_Number;
                it.valuedouble = num;
                if num >= (i32::MAX as f64) {
                    it.valueint = i32::MAX;
                } else if num <= (i32::MIN as f64) {
                    it.valueint = i32::MIN;
                } else {
                    it.valueint = num as i32;
                }
                return Some(it);
            }
        }
    }
    // fallback: delete and set error
    {
        let mut ge = global_error_instance().lock().unwrap();
        ge.json = buffer.content.clone();
        ge.position = buffer.offset;
    }
    None
}

pub fn cJSON_Parse(value: Option<&str>) -> Option<Box<cJSON>> {
    cJSON_ParseWithOpts(value, None, 0)
}

pub fn cJSON_ParseWithLength(value: Option<&str>, buffer_length: usize) -> Option<Box<cJSON>> {
    cJSON_ParseWithLengthOpts(value, buffer_length, None, 0)
}

pub fn print(item: Option<&cJSON>, _format: cJSON_bool, _hooks: Option<&internal_hooks>) -> Option<String> {
    if item.is_none() {
        return None;
    }
    let it = item.unwrap();
    match it.type_ & 0xFF {
        x if x == cJSON_NULL => Some("null".to_string()),
        x if x == cJSON_False => Some("false".to_string()),
        x if x == cJSON_True => Some("true".to_string()),
        x if x == cJSON_Number => Some(format!("{}", it.valuedouble)),
        x if x == cJSON_String => it.valuestring.clone(),
        _ => None,
    }
}

pub fn cJSON_Print(item: Option<&cJSON>) -> Option<String> {
    print(item, 1, None)
}

pub fn cJSON_PrintUnformatted(item: Option<&cJSON>) -> Option<String> {
    print(item, 0, None)
}

pub fn cJSON_PrintBuffered(_item: Option<&cJSON>, _prebuffer: i32, _fmt: cJSON_bool) -> Option<String> {
    None
}

pub fn cJSON_PrintPreallocated(_item: Option<&mut cJSON>, _buffer: &mut [u8], _length: i32, _format: cJSON_bool) -> cJSON_bool {
    0
}

pub fn parse_value(_item: Option<&mut cJSON>, _input_buffer: Option<&mut parse_buffer>) -> cJSON_bool {
    0
}

pub fn print_value(_item: Option<&cJSON>, _output_buffer: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn parse_array(_item: Option<&mut cJSON>, _input_buffer: Option<&mut parse_buffer>) -> cJSON_bool {
    0
}

pub fn print_array(_item: Option<&cJSON>, _output_buffer: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn parse_object(_item: Option<&mut cJSON>, _input_buffer: Option<&mut parse_buffer>) -> cJSON_bool {
    0
}

pub fn print_object(_item: Option<&cJSON>, _output_buffer: Option<&mut printbuffer>) -> cJSON_bool {
    0
}

pub fn cJSON_GetArraySize(array: Option<&cJSON>) -> i32 {
    if array.is_none() {
        return 0;
    }
    let mut count = 0i32;
    let mut current = array.unwrap().child.as_ref().map(|b| &**b);
    while let Some(node) = current {
        count += 1;
        current = node.next.as_ref().map(|b| &**b);
    }
    count
}

pub fn get_array_item(array: Option<&cJSON>, index: usize) -> Option<&cJSON> {
    if array.is_none() {
        return None;
    }
    let mut current = array.unwrap().child.as_ref().map(|b| &**b);
    let mut i = index;
    while let Some(node) = current {
        if i == 0 {
            return Some(node);
        }
        current = node.next.as_ref().map(|b| &**b);
        i -= 1;
    }
    None
}

pub fn cJSON_GetArrayItem(array: Option<&cJSON>, index: i32) -> Option<&cJSON> {
    if index < 0 {
        return None;
    }
    get_array_item(array, index as usize)
}

pub fn get_object_item(object: Option<&cJSON>, name: Option<&str>, case_sensitive: cJSON_bool) -> Option<&cJSON> {
    if object.is_none() || name.is_none() {
        return None;
    }
    let obj = object.unwrap();
    let mut current = obj.child.as_ref().map(|b| &**b);
    if case_sensitive != 0 {
        while let Some(node) = current {
            if let Some(ref s) = node.string {
                if s == name.unwrap() {
                    return Some(node);
                }
            }
            current = node.next.as_ref().map(|b| &**b);
        }
    } else {
        while let Some(node) = current {
            if let Some(ref s) = node.string {
                if s.eq_ignore_ascii_case(name.unwrap()) {
                    return Some(node);
                }
            }
            current = node.next.as_ref().map(|b| &**b);
        }
    }
    None
}

pub fn cJSON_GetObjectItem(object: Option<&cJSON>, string: Option<&str>) -> Option<&cJSON> {
    get_object_item(object, string, 0)
}

pub fn cJSON_GetObjectItemCaseSensitive(object: Option<&cJSON>, string: Option<&str>) -> Option<&cJSON> {
    get_object_item(object, string, 1)
}

pub fn cJSON_HasObjectItem(object: Option<&cJSON>, string: Option<&str>) -> cJSON_bool {
    if cJSON_GetObjectItem(object, string).is_some() { 1 } else { 0 }
}

pub fn suffix_object(prev: &mut cJSON, item: &mut cJSON) {
    prev.next = Some(Box::new(item.clone()));
    item.prev = Some(Box::new(prev.clone()));
}

pub fn create_reference(item: Option<&cJSON>, _hooks: Option<&internal_hooks>) -> Option<Box<cJSON>> {
    if item.is_none() {
        return None;
    }
    let it = item.unwrap();
    let mut reference = cJSON::new();
    reference.next = None;
    reference.prev = None;
    reference.child = None;
    reference.string = None;
    reference.type_ = it.type_ | cJSON_IsReference;
    reference.valuestring = it.valuestring.clone();
    reference.valueint = it.valueint;
    reference.valuedouble = it.valuedouble;
    Some(Box::new(reference))
}

pub fn add_item_to_array(array: Option<&mut cJSON>, item: Option<Box<cJSON>>) -> cJSON_bool {
    if array.is_none() || item.is_none() {
        return 0;
    }
    let arr = array.unwrap();
    if arr.child.is_none() {
        arr.child = item;
        if let Some(ref mut child) = arr.child {
            child.prev = Some(child.clone());
            child.next = None;
        }
        1
    } else {
        let mut last = arr.child.as_mut().unwrap();
        // find last via prev if set
        if let Some(ref mut prev_last) = last.prev {
            prev_last.next = item;
            arr.child.as_mut().unwrap().prev = arr.child.as_ref().unwrap().next.clone().or(arr.child.clone());
            1
        } else {
            // fallback append
            while last.next.is_some() {
                let nxt = last.next.as_mut().unwrap();
                last = nxt;
            }
            last.next = item;
            1
        }
    }
}

pub fn cJSON_AddItemToArray(array: Option<&mut cJSON>, item: Option<Box<cJSON>>) -> cJSON_bool {
    add_item_to_array(array, item)
}

pub fn cast_away_const(string: Option<&str>) -> Option<String> {
    string.map(|s| s.to_string())
}

pub fn add_item_to_object(object: Option<&mut cJSON>, string: Option<&str>, item: Option<Box<cJSON>>, hooks: Option<&internal_hooks>, constant_key: cJSON_bool) -> cJSON_bool {
    if object.is_none() || string.is_none() || item.is_none() {
        return 0;
    }
    let obj = object.unwrap();
    let mut new_key: Option<String> = None;
    let mut new_type = cJSON_Invalid;
    if constant_key != 0 {
        new_key = string.map(|s| s.to_string());
        new_type = item.as_ref().unwrap().type_ | cJSON_StringIsConst;
    } else {
        new_key = cJSON_strdup(string, None);
        if new_key.is_none() {
            return 0;
        }
        new_type = item.as_ref().unwrap().type_ & !cJSON_StringIsConst;
    }
    let mut it = *item.unwrap();
    if (it.type_ & cJSON_StringIsConst) == 0 && it.string.is_some() {
        it.string = None;
    }
    it.string = new_key;
    it.type_ = new_type;
    add_item_to_array(Some(obj), Some(Box::new(it)))
}

pub fn cJSON_AddItemToObject(object: Option<&mut cJSON>, string: Option<&str>, item: Option<Box<cJSON>>) -> cJSON_bool {
    add_item_to_object(object, string, item, None, 0)
}

pub fn cJSON_AddItemToObjectCS(object: Option<&mut cJSON>, string: Option<&str>, item: Option<Box<cJSON>>) -> cJSON_bool {
    add_item_to_object(object, string, item, None, 1)
}

pub fn cJSON_AddItemReferenceToArray(array: Option<&mut cJSON>, item: Option<&cJSON>) -> cJSON_bool {
    if array.is_none() {
        return 0;
    }
    let reference = create_reference(item, None);
    add_item_to_array(array, reference)
}

pub fn cJSON_AddItemReferenceToObject(object: Option<&mut cJSON>, string: Option<&str>, item: Option<&cJSON>) -> cJSON_bool {
    if object.is_none() || string.is_none() {
        return 0;
    }
    let reference = create_reference(item, None);
    add_item_to_object(object, string, reference, None, 0)
}

pub fn cJSON_CreateNull() -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_NULL;
    Some(Box::new(item))
}

pub fn cJSON_CreateTrue() -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_True;
    Some(Box::new(item))
}

pub fn cJSON_CreateFalse() -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_False;
    Some(Box::new(item))
}

pub fn cJSON_CreateBool(boolean: cJSON_bool) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = if boolean != 0 { cJSON_True } else { cJSON_False };
    Some(Box::new(item))
}

pub fn cJSON_CreateNumber(num: f64) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Number;
    item.valuedouble = num;
    if num >= (i32::MAX as f64) {
        item.valueint = i32::MAX;
    } else if num <= (i32::MIN as f64) {
        item.valueint = i32::MIN;
    } else {
        item.valueint = num as i32;
    }
    Some(Box::new(item))
}

pub fn cJSON_CreateString(string: Option<&str>) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_String;
    item.valuestring = string.map(|s| s.to_string());
    Some(Box::new(item))
}

pub fn cJSON_CreateStringReference(string: Option<&str>) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_String | cJSON_IsReference;
    item.valuestring = string.map(|s| s.to_string());
    Some(Box::new(item))
}

pub fn cJSON_CreateObjectReference(child: Option<&cJSON>) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Object | cJSON_IsReference;
    item.child = child.map(|c| Box::new(c.clone()));
    Some(Box::new(item))
}

pub fn cJSON_CreateArrayReference(child: Option<&cJSON>) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Array | cJSON_IsReference;
    item.child = child.map(|c| Box::new(c.clone()));
    Some(Box::new(item))
}

pub fn cJSON_CreateRaw(raw: Option<&str>) -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Raw;
    item.valuestring = raw.map(|s| s.to_string());
    Some(Box::new(item))
}

pub fn cJSON_CreateArray() -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Array;
    Some(Box::new(item))
}

pub fn cJSON_CreateObject() -> Option<Box<cJSON>> {
    let mut item = cJSON::new();
    item.type_ = cJSON_Object;
    Some(Box::new(item))
}

pub fn cJSON_CreateIntArray(numbers: Option<&[i32]>, count: i32) -> Option<Box<cJSON>> {
    if numbers.is_none() || count < 0 {
        return None;
    }
    let nums = numbers.unwrap();
    let mut a = cJSON_CreateArray().unwrap();
    let mut last: Option<Box<cJSON>> = None;
    for i in 0..(count as usize) {
        let n = cJSON_CreateNumber(nums[i] as f64).unwrap();
        if last.is_none() {
            a.child = Some(n);
        } else {
            last.as_mut().unwrap().next = Some(n);
        }
        last = a.child.as_ref().map(|b| b.clone());
        if let Some(ref mut c) = a.child {
            c.prev = last.clone();
        }
    }
    Some(a)
}

pub fn cJSON_CreateFloatArray(numbers: Option<&[f32]>, count: i32) -> Option<Box<cJSON>> {
    if numbers.is_none() || count < 0 {
        return None;
    }
    let nums = numbers.unwrap();
    let mut a = cJSON_CreateArray().unwrap();
    let mut last: Option<Box<cJSON>> = None;
    for i in 0..(count as usize) {
        let n = cJSON_CreateNumber(nums[i] as f64).unwrap();
        if last.is_none() {
            a.child = Some(n);
        } else {
            last.as_mut().unwrap().next = Some(n);
        }
        last = a.child.as_ref().map(|b| b.clone());
        if let Some(ref mut c) = a.child {
            c.prev = last.clone();
        }
    }
    Some(a)
}

pub fn cJSON_CreateDoubleArray(numbers: Option<&[f64]>, count: i32) -> Option<Box<cJSON>> {
    if numbers.is_none() || count < 0 {
        return None;
    }
    let nums = numbers.unwrap();
    let mut a = cJSON_CreateArray().unwrap();
    let mut last: Option<Box<cJSON>> = None;
    for i in 0..(count as usize) {
        let n = cJSON_CreateNumber(nums[i]).unwrap();
        if last.is_none() {
            a.child = Some(n);
        } else {
            last.as_mut().unwrap().next = Some(n);
        }
        last = a.child.as_ref().map(|b| b.clone());
        if let Some(ref mut c) = a.child {
            c.prev = last.clone();
        }
    }
    Some(a)
}

pub fn cJSON_CreateStringArray(strings: Option<&[&str]>, count: i32) -> Option<Box<cJSON>> {
    if strings.is_none() || count < 0 {
        return None;
    }
    let strs = strings.unwrap();
    let mut a = cJSON_CreateArray().unwrap();
    let mut last: Option<Box<cJSON>> = None;
    for i in 0..(count as usize) {
        let n = cJSON_CreateString(Some(strs[i])).unwrap();
        if last.is_none() {
            a.child = Some(n);
        } else {
            last.as_mut().unwrap().next = Some(n);
        }
        last = a.child.as_ref().map(|b| b.clone());
        if let Some(ref mut c) = a.child {
            c.prev = last.clone();
        }
    }
    Some(a)
}

pub fn cJSON_Duplicate(item: Option<&cJSON>, recurse: cJSON_bool) -> Option<Box<cJSON>> {
    cJSON_Duplicate_rec(item, 0usize, recurse)
}

pub fn cJSON_Duplicate_rec(item: Option<&cJSON>, depth: usize, recurse: cJSON_bool) -> Option<Box<cJSON>> {
    if item.is_none() {
        return None;
    }
    if depth >= CJSON_CIRCULAR_LIMIT {
        return None;
    }
    let it = item.unwrap();
    let mut newitem = cJSON::new();
    newitem.type_ = it.type_ & !cJSON_IsReference;
    newitem.valueint = it.valueint;
    newitem.valuedouble = it.valuedouble;
    newitem.valuestring = it.valuestring.clone();
    newitem.string = it.string.clone();
    if recurse == 0 {
        return Some(Box::new(newitem));
    }
    let mut child = it.child.as_ref().map(|b| &**b);
    let mut last_new: Option<Box<cJSON>> = None;
    while let Some(ch) = child {
        let newchild = cJSON_Duplicate_rec(Some(ch), depth + 1, 1);
        if newchild.is_none() {
            return None;
        }
        if last_new.is_none() {
            newitem.child = newchild;
            last_new = newitem.child.as_ref().map(|b| b.clone());
        } else {
            last_new.as_mut().unwrap().next = newchild;
            let tmp = last_new.as_mut().unwrap().next.as_ref().map(|b| b.clone());
            last_new = tmp;
        }
        child = ch.next.as_ref().map(|b| &**b);
    }
    if newitem.child.is_some() {
        let mut tail = newitem.child.as_ref().map(|b| b.clone()).unwrap();
        while tail.next.is_some() {
            let t = tail.next.as_ref().map(|b| b.clone()).unwrap();
            tail = t;
        }
        newitem.child.as_mut().unwrap().prev = Some(tail);
    }
    Some(Box::new(newitem))
}

pub fn skip_oneline_comment(input: &mut &str) {
    if input.len() >= 2 {
        *input = &input[2..];
    }
    while !input.is_empty() {
        if input.starts_with('\n') {
            *input = &input[1..];
            return;
        }
        *input = &input[1..];
    }
}

pub fn skip_multiline_comment(input: &mut &str) {
    if input.len() >= 2 {
        *input = &input[2..];
    }
    while !input.is_empty() {
        if input.starts_with("*/") {
            *input = &input[2..];
            return;
        }
        *input = &input[1..];
    }
}

pub fn minify_string(input: &mut &str, output: &mut String) {
    if input.starts_with('\"') {
        output.push('\"');
        *input = &input[1..];
    }
    while !input.is_empty() {
        let ch = input.chars().next().unwrap();
        if ch == '\"' {
            output.push('\"');
            *input = &input[1..];
            return;
        } else if ch == '\\' {
            if input.len() >= 2 && input.as_bytes()[1] == b'\"' {
                output.push('\\');
                output.push('\"');
                *input = &input[2..];
            } else {
                output.push(ch);
                *input = &input[1..];
            }
        } else {
            output.push(ch);
            *input = &input[1..];
        }
    }
}

pub fn cJSON_Minify(json: Option<&mut String>) {
    if json.is_none() {
        return;
    }
    let mut s = json.unwrap();
    let mut into = String::new();
    let mut input = s.as_str();
    while !input.is_empty() {
        let ch = input.chars().next().unwrap();
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                input = &input[1..];
            }
            '/' => {
                if input.len() >= 2 && input.as_bytes()[1] == b'/' {
                    skip_oneline_comment(&mut input);
                } else if input.len() >= 2 && input.as_bytes()[1] == b'*' {
                    skip_multiline_comment(&mut input);
                } else {
                    into.push('/');
                    input = &input[1..];
                }
            }
            '\"' => {
                minify_string(&mut input, &mut into);
            }
            _ => {
                into.push(ch);
                input = &input[1..];
            }
        }
    }
    *s = into;
}

pub fn cJSON_IsInvalid(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_Invalid { 1 } else { 0 }
}

pub fn cJSON_IsFalse(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_False { 1 } else { 0 }
}

pub fn cJSON_IsTrue(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_True { 1 } else { 0 }
}

pub fn cJSON_IsBool(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & (cJSON_True | cJSON_False)) != 0 { 1 } else { 0 }
}

pub fn cJSON_IsNull(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_NULL { 1 } else { 0 }
}

pub fn cJSON_IsNumber(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_Number { 1 } else { 0 }
}

pub fn cJSON_IsString(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_String { 1 } else { 0 }
}

pub fn cJSON_IsArray(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_Array { 1 } else { 0 }
}

pub fn cJSON_IsObject(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_Object { 1 } else { 0 }
}

pub fn cJSON_IsRaw(item: Option<&cJSON>) -> cJSON_bool {
    if item.is_none() {
        return 0;
    }
    let it = item.unwrap();
    if (it.type_ & 0xFF) == cJSON_Raw { 1 } else { 0 }
}

pub fn cJSON_Compare(a: Option<&cJSON>, b: Option<&cJSON>, _case_sensitive: cJSON_bool) -> cJSON_bool {
    if a.is_none() || b.is_none() {
        return 0;
    }
    let aa = a.unwrap();
    let bb = b.unwrap();
    if (aa.type_ & 0xFF) != (bb.type_ & 0xFF) {
        return 0;
    }
    if std::ptr::eq(aa, bb) {
        return 1;
    }
    match aa.type_ & 0xFF {
        x if x == cJSON_False || x == cJSON_True || x == cJSON_NULL => 1,
        x if x == cJSON_Number => {
            if compare_double(aa.valuedouble, bb.valuedouble) != 0 { 1 } else { 0 }
        }
        x if x == cJSON_String || x == cJSON_Raw => {
            if aa.valuestring.is_none() || bb.valuestring.is_none() { return 0; }
            if aa.valuestring.as_ref().unwrap() == bb.valuestring.as_ref().unwrap() { 1 } else { 0 }
        }
        x if x == cJSON_Array => {
            let mut a_el = aa.child.as_ref().map(|b| &**b);
            let mut b_el = bb.child.as_ref().map(|b| &**b);
            while a_el.is_some() && b_el.is_some() {
                if cJSON_Compare(a_el, b_el, _case_sensitive) == 0 {
                    return 0;
                }
                a_el = a_el.unwrap().next.as_ref().map(|b| &**b);
                b_el = b_el.unwrap().next.as_ref().map(|b| &**b);
            }
            if a_el.is_some() || b_el.is_some() { 0 } else { 1 }
        }
        x if x == cJSON_Object => {
            let mut a_el = aa.child.as_ref().map(|b| &**b);
            while let Some(ae) = a_el {
                let name = ae.string.as_ref().map(|s| s.as_str());
                let b_el = get_object_item(Some(bb), name, _case_sensitive);
                if b_el.is_none() {
                    return 0;
                }
                if cJSON_Compare(Some(ae), b_el, _case_sensitive) == 0 {
                    return 0;
                }
                a_el = ae.next.as_ref().map(|b| &**b);
            }
            let mut b_el2 = bb.child.as_ref().map(|b| &**b);
            while let Some(be) = b_el2 {
                let name = be.string.as_ref().map(|s| s.as_str());
                let a_el2 = get_object_item(Some(aa), name, _case_sensitive);
                if a_el2.is_none() {
                    return 0;
                }
                if cJSON_Compare(a_el2, Some(be), _case_sensitive) == 0 {
                    return 0;
                }
                b_el2 = be.next.as_ref().map(|b| &**b);
            }
            1
        }
        _ => 0,
    }
}

pub fn cJSON_malloc(size: usize) -> Vec<u8> {
    (internal_hooks::default().allocate)(size)
}

pub fn cJSON_free(_object: Vec<u8>) {
    // no-op in this translation
}