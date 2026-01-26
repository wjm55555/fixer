#![allow(non_snake_case, non_upper_case_globals, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::sync::Mutex;
use std::cmp;

const CJSON_VERSION_MAJOR: i32 = 1;
const CJSON_VERSION_MINOR: i32 = 7;
const CJSON_VERSION_PATCH: i32 = 19;

const CJSON_NESTING_LIMIT: usize = 1000;
const CJSON_CIRCULAR_LIMIT: usize = 10000;

const cJSON_Invalid: i32 = 0;
const cJSON_False: i32 = 1 << 0;
const cJSON_True: i32 = 1 << 1;
const cJSON_NULL: i32 = 1 << 2;
const cJSON_Number: i32 = 1 << 3;
const cJSON_String: i32 = 1 << 4;
const cJSON_Array: i32 = 1 << 5;
const cJSON_Object: i32 = 1 << 6;
const cJSON_Raw: i32 = 1 << 7;
const cJSON_IsReference: i32 = 256;
const cJSON_StringIsConst: i32 = 512;

const true: i32 = 1;
const false: i32 = 0;

pub type cJSON_bool = i32;

#[derive(Clone, Debug)]
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

impl Default for cJSON {
    fn default() -> Self {
        cJSON {
            next: None,
            prev: None,
            child: None,
            type_: cJSON_Invalid,
            valuestring: None,
            valueint: 0,
            valuedouble: 0.0,
            string: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<fn(usize) -> *mut u8>,
    pub free_fn: Option<fn(*mut u8)>,
}

impl Default for cJSON_Hooks {
    fn default() -> Self {
        cJSON_Hooks {
            malloc_fn: None,
            free_fn: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct internal_hooks {
    pub allocate: fn(usize) -> *mut u8,
    pub deallocate: fn(*mut u8),
    pub reallocate: fn(*mut u8, usize) -> *mut u8,
}

impl Default for internal_hooks {
    fn default() -> Self {
        internal_hooks {
            allocate: default_malloc,
            deallocate: default_free,
            reallocate: default_realloc,
        }
    }
}

#[derive(Clone, Debug)]
pub struct error {
    pub json: Option<Vec<u8>>,
    pub position: usize,
}

impl Default for error {
    fn default() -> Self {
        error {
            json: None,
            position: 0,
        }
    }
}

fn default_malloc(size: usize) -> *mut u8 {
    std::alloc::alloc(std::alloc::Layout::from_size_align(size, 1).unwrap())
}

fn default_free(ptr: *mut u8) {
    std::alloc::dealloc(ptr, std::alloc::Layout::from_size_align(1, 1).unwrap());
}

fn default_realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    std::alloc::realloc(ptr, std::alloc::Layout::from_size_align(1, 1).unwrap(), size)
}

lazy_static::lazy_static! {
    static ref GLOBAL_HOOKS: Mutex<internal_hooks> = Mutex::new(internal_hooks::default());
    static ref GLOBAL_ERROR: Mutex<error> = Mutex::new(error::default());
}

#[derive(Clone, Debug)]
pub struct parse_buffer {
    pub content: Vec<u8>,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub hooks: internal_hooks,
}

impl Default for parse_buffer {
    fn default() -> Self {
        parse_buffer {
            content: Vec::new(),
            length: 0,
            offset: 0,
            depth: 0,
            hooks: internal_hooks::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct printbuffer {
    pub buffer: Vec<u8>,
    pub length: usize,
    pub offset: usize,
    pub depth: usize,
    pub noalloc: bool,
    pub format: bool,
    pub hooks: internal_hooks,
}

impl Default for printbuffer {
    fn default() -> Self {
        printbuffer {
            buffer: Vec::new(),
            length: 0,
            offset: 0,
            depth: 0,
            noalloc: false,
            format: false,
            hooks: internal_hooks::default(),
        }
    }
}

pub fn cJSON_GetErrorPtr() -> Option<String> {
    let error = GLOBAL_ERROR.lock().unwrap();
    match &error.json {
        Some(json) => {
            let pos = error.position;
            if pos < json.len() {
                String::from_utf8(json[pos..].to_vec()).ok()
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn cJSON_GetStringValue(item: &cJSON) -> Option<String> {
    if cJSON_IsString(item) != 0 {
        item.valuestring.clone()
    } else {
        None
    }
}

pub fn cJSON_GetNumberValue(item: &cJSON) -> f64 {
    if cJSON_IsNumber(item) != 0 {
        item.valuedouble
    } else {
        f64::NAN
    }
}

pub fn cJSON_Version() -> String {
    format!("{}.{}.{}", CJSON_VERSION_MAJOR, CJSON_VERSION_MINOR, CJSON_VERSION_PATCH)
}

fn case_insensitive_strcmp(string1: &[u8], string2: &[u8]) -> i32 {
    if string1.is_empty() || string2.is_empty() {
        return 1;
    }

    for (a, b) in string1.iter().zip(string2.iter()) {
        let a_lower = (*a as char).to_lowercase().next().unwrap_or('\0');
        let b_lower = (*b as char).to_lowercase().next().unwrap_or('\0');
        if a_lower != b_lower {
            return (a_lower as i32) - (b_lower as i32);
        }
    }

    if string1.len() != string2.len() {
        return (string1.len() as i32) - (string2.len() as i32);
    }

    0
}

fn cJSON_strdup(string: &str) -> String {
    string.to_string()
}

pub fn cJSON_InitHooks(hooks: Option<&cJSON_Hooks>) {
    let mut global = GLOBAL_HOOKS.lock().unwrap();
    if let Some(h) = hooks {
        if h.malloc_fn.is_some() {
            // Custom allocator setup would go here
        }
        if h.free_fn.is_some() {
            // Custom deallocator setup would go here
        }
    }
}

fn cJSON_New_Item() -> cJSON {
    cJSON::default()
}

pub fn cJSON_Delete(item: Option<Box<cJSON>>) {
    if let Some(mut current) = item {
        loop {
            let next = current.next.take();
            if !(current.type_ & cJSON_IsReference != 0) && current.child.is_some() {
                cJSON_Delete(current.child.take());
            }
            if !(current.type_ & cJSON_IsReference != 0) {
                current.valuestring = None;
            }
            if !(current.type_ & cJSON_StringIsConst != 0) {
                current.string = None;
            }
            if let Some(n) = next {
                current = n;
            } else {
                break;
            }
        }
    }
}

fn get_decimal_point() -> u8 {
    b'.'
}

fn can_read(buffer: &parse_buffer, size: usize) -> bool {
    buffer.offset + size <= buffer.length
}

fn can_access_at_index(buffer: &parse_buffer, index: usize) -> bool {
    buffer.offset + index < buffer.length
}

fn cannot_access_at_index(buffer: &parse_buffer, index: usize) -> bool {
    !can_access_at_index(buffer, index)
}

fn buffer_at_offset(buffer: &parse_buffer) -> u8 {
    if buffer.offset < buffer.content.len() {
        buffer.content[buffer.offset]
    } else {
        0
    }
}

fn parse_number(item: &mut cJSON, input_buffer: &mut parse_buffer) -> bool {
    let decimal_point = get_decimal_point();
    let mut number_string_length = 0;
    let mut has_decimal_point = false;

    if input_buffer.content.is_empty() {
        return false;
    }

    let mut i = 0;
    while input_buffer.offset + i < input_buffer.length {
        match input_buffer.content[input_buffer.offset + i] {
            b'0'..=b'9' | b'+' | b'-' | b'e' | b'E' => {
                number_string_length += 1;
                i += 1;
            }
            b'.' => {
                number_string_length += 1;
                has_decimal_point = true;
                i += 1;
            }
            _ => break,
        }
    }

    if number_string_length == 0 {
        return false;
    }

    let mut number_c_string = String::new();
    for j in 0..number_string_length {
        let ch = input_buffer.content[input_buffer.offset + j] as char;
        if ch == '.' {
            number_c_string.push(decimal_point as char);
        } else {
            number_c_string.push(ch);
        }
    }

    match number_c_string.parse::<f64>() {
        Ok(number) => {
            item.valuedouble = number;
            if number >= i32::MAX as f64 {
                item.valueint = i32::MAX;
            } else if number <= i32::MIN as f64 {
                item.valueint = i32::MIN;
            } else {
                item.valueint = number as i32;
            }
            item.type_ = cJSON_Number;
            input_buffer.offset += number_string_length;
            true
        }
        Err(_) => false,
    }
}

pub fn cJSON_SetNumberHelper(object: &mut cJSON, number: f64) -> f64 {
    if number >= i32::MAX as f64 {
        object.valueint = i32::MAX;
    } else if number <= i32::MIN as f64 {
        object.valueint = i32::MIN;
    } else {
        object.valueint = number as i32;
    }
    object.valuedouble = number;
    number
}

pub fn cJSON_SetValuestring(object: &mut cJSON, valuestring: &str) -> Option<String> {
    if cJSON_IsString(object) == 0 || (object.type_ & cJSON_IsReference != 0) {
        return None;
    }

    object.valuestring = Some(valuestring.to_string());
    object.valuestring.clone()
}

fn ensure(p: &mut printbuffer, needed: usize) -> bool {
    if p.buffer.is_empty() {
        return false;
    }

    if p.length > 0 && p.offset >= p.length {
        return false;
    }

    if needed > i32::MAX as usize {
        return false;
    }

    let total_needed = needed + p.offset + 1;
    if total_needed <= p.length {
        return true;
    }

    if p.noalloc {
        return false;
    }

    let new_size = if total_needed > (i32::MAX as usize / 2) {
        if total_needed <= i32::MAX as usize {
            i32::MAX as usize
        } else {
            return false;
        }
    } else {
        total_needed * 2
    };

    p.buffer.resize(new_size, 0);
    p.length = new_size;
    true
}

fn update_offset(buffer: &mut printbuffer) {
    if buffer.offset < buffer.buffer.len() {
        let mut len = 0;
        for i in buffer.offset..buffer.buffer.len() {
            if buffer.buffer[i] == 0 {
                break;
            }
            len += 1;
        }
        buffer.offset += len;
    }
}

fn compare_double(a: f64, b: f64) -> bool {
    let max_val = a.abs().max(b.abs());
    (a - b).abs() <= max_val * f64::EPSILON
}

fn print_number(item: &cJSON, output_buffer: &mut printbuffer) -> bool {
    let d = item.valuedouble;

    if d.is_nan() || d.is_infinite() {
        let number_buffer = "null";
        if !ensure(output_buffer, number_buffer.len()) {
            return false;
        }
        let bytes = number_buffer.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if output_buffer.offset + i < output_buffer.buffer.len() {
                output_buffer.buffer[output_buffer.offset + i] = b;
            }
        }
        output_buffer.offset += bytes.len();
        return true;
    }

    let decimal_point = get_decimal_point();
    let number_buffer = if d == item.valueint as f64 {
        format!("{}", item.valueint)
    } else {
        let formatted = format!("{:.15}", d);
        match formatted.parse::<f64>() {
            Ok(test) if compare_double(test, d) => formatted,
            _ => format!("{:.17}", d),
        }
    };

    if !ensure(output_buffer, number_buffer.len()) {
        return false;
    }

    let bytes = number_buffer.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let byte_to_write = if b == b'.' { decimal_point } else { b };
        if output_buffer.offset + i < output_buffer.buffer.len() {
            output_buffer.buffer[output_buffer.offset + i] = byte_to_write;
        }
    }
    output_buffer.offset += bytes.len();
    true
}

fn parse_hex4(input: &[u8]) -> u32 {
    let mut h: u32 = 0;
    for i in 0..4 {
        if i >= input.len() {
            return 0;
        }
        match input[i] {
            b'0'..=b'9' => {
                h += (input[i] - b'0') as u32;
            }
            b'A'..=b'F' => {
                h += (10 + input[i] - b'A') as u32;
            }
            b'a'..=b'f' => {
                h += (10 + input[i] - b'a') as u32;
            }
            _ => return 0,
        }
        if i < 3 {
            h = h << 4;
        }
    }
    h
}

fn utf16_literal_to_utf8(
    input_pointer: &[u8],
    input_end: &[u8],
    output_pointer: &mut Vec<u8>,
) -> u8 {
    let first_sequence = input_pointer;

    if (input_end.len() as i32 - first_sequence.len() as i32) < 6 {
        return 0;
    }

    let first_code = parse_hex4(&first_sequence[2..]);
    if (first_code >= 0xDC00) && (first_code <= 0xDFFF) {
        return 0;
    }

    let codepoint = if (first_code >= 0xD800) && (first_code <= 0xDBFF) {
        let second_sequence = &first_sequence[6..];
        if (input_end.len() as i32 - second_sequence.len() as i32) < 6 {
            return 0;
        }
        if second_sequence.len() < 2 || second_sequence[0] != b'\\' || second_sequence[1] != b'u' {
            return 0;
        }
        let second_code = parse_hex4(&second_sequence[2..]);
        if (second_code < 0xDC00) || (second_code > 0xDFFF) {
            return 0;
        }
        0x10000 + (((first_code & 0x3FF) << 10) | (second_code & 0x3FF))
    } else {
        first_code
    };

    let (utf8_length, first_byte_mark) = if codepoint < 0x80 {
        (1, 0u8)
    } else if codepoint < 0x800 {
        (2, 0xC0)
    } else if codepoint < 0x10000 {
        (3, 0xE0)
    } else if codepoint <= 0x10FFFF {
        (4, 0xF0)
    } else {
        return 0;
    };

    let mut cp = codepoint;
    for i in (1..utf8_length).rev() {
        output_pointer.push(((cp | 0x80) & 0xBF) as u8);
        cp >>= 6;
    }

    if utf8_length > 1 {
        output_pointer.push((cp | first_byte_mark as u32) as u8);
    } else {
        output_pointer.push((cp & 0x7F) as u8);
    }

    if first_code >= 0xD800 && first_code <= 0xDBFF {
        12
    } else {
        6
    }
}

fn parse_string(item: &mut cJSON, input_buffer: &mut parse_buffer) -> bool {
    if input_buffer.offset >= input_buffer.content.len()
        || input_buffer.content[input_buffer.offset] != b'"'
    {
        return false;
    }

    let mut input_end = input_buffer.offset + 1;
    let mut skipped_bytes = 0;

    while input_end < input_buffer.content.len() && input_buffer.content[input_end] != b'"' {
        if input_buffer.content[input_end] == b'\\' {
            if input_end + 1 >= input_buffer.content.len() {
                return false;
            }
            skipped_bytes += 1;
            input_end += 1;
        }
        input_end += 1;
    }

    if input_end >= input_buffer.content.len() || input_buffer.content[input_end] != b'"' {
        return false;
    }

    let allocation_length = input_end - input_buffer.offset - 1 - skipped_bytes;
    let mut output = Vec::new();

    let mut input_pointer = input_buffer.offset + 1;
    while input_pointer < input_end {
        if input_buffer.content[input_pointer] != b'\\' {
            output.push(input_buffer.content[input_pointer]);
            input_pointer += 1;
        } else {
            if input_pointer + 1 >= input_end {
                return false;
            }
            match input_buffer.content[input_pointer + 1] {
                b'b' => {
                    output.push(b'\x08');
                    input_pointer += 2;
                }
                b'f' => {
                    output.push(b'\x0C');
                    input_pointer += 2;
                }
                b'n' => {
                    output.push(b'\n');
                    input_pointer += 2;
                }
                b'r' => {
                    output.push(b'\r');
                    input_pointer += 2;
                }
                b't' => {
                    output.push(b'\t');
                    input_pointer += 2;
                }
                b'"' | b'\\' | b'/' => {
                    output.push(input_buffer.content[input_pointer + 1]);
                    input_pointer += 2;
                }
                b'u' => {
                    if input_pointer + 6 > input_end {
                        return false;
                    }
                    let seq_len =
                        utf16_literal_to_utf8(&input_buffer.content[input_pointer..], &input_buffer.content[input_end..], &mut output);
                    if seq_len == 0 {
                        return false;
                    }
                    input_pointer += seq_len as usize;
                }
                _ => {
                    return false;
                }
            }
        }
    }

    if let Ok(s) = String::from_utf8(output) {
        item.type_ = cJSON_String;
        item.valuestring = Some(s);
        input_buffer.offset = input_end + 1;
        true
    } else {
        false
    }
}

fn print_string_ptr(input: Option<&str>, output_buffer: &mut printbuffer) -> bool {
    if output_buffer.buffer.is_empty() {
        return false;
    }

    match input {
        None => {
            if !ensure(output_buffer, 2) {
                return false;
            }
            output_buffer.buffer[output_buffer.offset] = b'"';
            output_buffer.buffer[output_buffer.offset + 1] = b'"';
            output_buffer.offset += 2;
            true
        }
        Some(input_str) => {
            let mut escape_characters = 0;
            for ch in input_str.chars() {
                match ch {
                    '"' | '\\' | '\x08' | '\x0C' | '\n' | '\r' | '\t' => {
                        escape_characters += 1;
                    }
                    c if (c as u32) < 32 => {
                        escape_characters += 5;
                    }
                    _ => {}
                }
            }

            let output_length = input_str.len() + escape_characters;
            if !ensure(output_buffer, output_length + 2) {
                return false;
            }

            if escape_characters == 0 {
                output_buffer.buffer[output_buffer.offset] = b'"';
                let bytes = input_str.as_bytes();
                for (i, &b) in bytes.iter().enumerate() {
                    output_buffer.buffer[output_buffer.offset + 1 + i] = b;
                }
                output_buffer.buffer[output_buffer.offset + output_length + 1] = b'"';
                output_buffer.offset += output_length + 2;
                return true;
            }

            output_buffer.buffer[output_buffer.offset] = b'"';
            let mut output_pos = output_buffer.offset + 1;

            for ch in input_str.chars() {
                if ch as u32 > 31 && ch != '"' && ch != '\\' {
                    output_buffer.buffer[output_pos] = ch as u8;
                    output_pos += 1;
                } else {
                    output_buffer.buffer[output_pos] = b'\\';
                    output_pos += 1;
                    match ch {
                        '\\' => {
                            output_buffer.buffer[output_pos] = b'\\';
                        }
                        '"' => {
                            output_buffer.buffer[output_pos] = b'"';
                        }
                        '\x08' => {
                            output_buffer.buffer[output_pos] = b'b';
                        }
                        '\x0C' => {
                            output_buffer.buffer[output_pos] = b'f';
                        }
                        '\n' => {
                            output_buffer.buffer[output_pos] = b'n';
                        }
                        '\r' => {
                            output_buffer.buffer[output_pos] = b'r';
                        }
                        '\t' => {
                            output_buffer.buffer[output_pos] = b't';
                        }
                        _ => {
                            let escape_seq = format!("u{:04x}", ch as u32);
                            let escape_bytes = escape_seq.as_bytes();
                            for (i, &b) in escape_bytes.iter().enumerate() {
                                output_buffer.buffer[output_pos + i] = b;
                            }
                            output_pos += escape_bytes.len() - 1;
                        }
                    }
                    output_pos += 1;
                }
            }

            output_buffer.buffer[output_pos] = b'"';
            output_buffer.offset = output_pos + 1;
            true
        }
    }
}

fn print_string(item: &cJSON, output_buffer: &mut printbuffer) -> bool {
    print_string_ptr(item.valuestring.as_deref(), output_buffer)
}

fn buffer_skip_whitespace(buffer: &mut parse_buffer) {
    if buffer.content.is_empty() {
        return;
    }

    while buffer.offset < buffer.content.len() && buffer.content[buffer.offset] <= 32 {
        buffer.offset += 1;
    }

    if buffer.offset >= buffer.content.len() && buffer.offset > 0 {
        buffer.offset -= 1;
    }
}

fn skip_utf8_bom(buffer: &mut parse_buffer) {
    if buffer.content.is_empty() || buffer.offset != 0 {
        return;
    }

    if buffer.content.len() >= 3
        && buffer.content[0] == 0xEF
        && buffer.content[1] == 0xBB
        && buffer.content[2] == 0xBF
    {
        buffer.offset += 3;
    }
}

pub fn cJSON_ParseWithOpts(
    value: &str,
    return_parse_end: Option<&mut usize>,
    require_null_terminated: bool,
) -> Option<Box<cJSON>> {
    let buffer_length = value.len() + 1;
    cJSON_ParseWithLengthOpts(value, buffer_length, require_null_terminated)
}

pub fn cJSON_ParseWithLengthOpts(
    value: &str,
    buffer_length: usize,
    require_null_terminated: bool,
) -> Option<Box<cJSON>> {
    if value.is_empty() || buffer_length == 0 {
        return None;
    }

    let mut buffer = parse_buffer {
        content: value.as_bytes().to_vec(),
        length: buffer_length,
        offset: 0,
        depth: 0,
        hooks: GLOBAL_HOOKS.lock().unwrap().clone(),
    };

    let mut item = Box::new(cJSON_New_Item());

    skip_utf8_bom(&mut buffer);
    buffer_skip_whitespace(&mut buffer);

    if !parse_value(&mut item, &mut buffer) {
        let mut err = GLOBAL_ERROR.lock().unwrap();
        err.json = Some(value.as_bytes().to_vec());
        err.position = buffer.offset;
        return None;
    }

    if require_null_terminated {
        buffer_skip_whitespace(&mut buffer);
        if buffer.offset < buffer.length && buffer.content[buffer.offset] != 0 {
            let mut err = GLOBAL_ERROR.lock().unwrap();
            err.json = Some(value.as_bytes().to_vec());
            err.position = buffer.offset;
            return None;
        }
    }

    Some(item)
}

pub fn cJSON_Parse(value: &str) -> Option<Box<cJSON>> {
    let buffer_length = value.len() + 1;
    cJSON_ParseWithLengthOpts(value, buffer_length, true)
}

pub fn cJSON_ParseWithLength(value: &str, buffer_length: usize) -> Option<Box<cJSON>> {
    cJSON_ParseWithLengthOpts(value, buffer_length, false)
}

fn print(item: &cJSON, format: bool) -> Option<String> {
    const DEFAULT_BUFFER_SIZE: usize = 256;
    let mut buffer = printbuffer {
        buffer: vec![0u8; DEFAULT_BUFFER_SIZE],
        length: DEFAULT_BUFFER_SIZE,
        offset: 0,
        depth: 0,
        noalloc: false,
        format,
        hooks: GLOBAL_HOOKS.lock().unwrap().clone(),
    };

    if !print_value(item, &mut buffer) {
        return None;
    }

    update_offset(&mut buffer);

    String::from_utf8(buffer.buffer[..buffer.offset].to_vec()).ok()
}

pub fn cJSON_Print(item: &cJSON) -> Option<String> {
    print(item, true)
}

pub fn cJSON_PrintUnformatted(item: &cJSON) -> Option<String> {
    print(item, false)
}

pub fn cJSON_PrintBuffered(item: &cJSON, prebuffer: i32, fmt: bool) -> Option<String> {
    if prebuffer < 0 {
        return None;
    }

    let mut p = printbuffer {
        buffer: vec![0u8; prebuffer as usize],
        length: prebuffer as usize,
        offset: 0,
        depth: 0,
        noalloc: false,
        format: fmt,
        hooks: GLOBAL_HOOKS.lock().unwrap().clone(),
    };

    if !print_value(item, &mut p) {
        return None;
    }

    String::from_utf8(p.buffer[..p.offset].to_vec()).ok()
}

pub fn cJSON_PrintPreallocated(
    item: &cJSON,
    buffer: &mut [u8],
    length: i32,
    format: bool,
) -> bool {
    if length < 0 || buffer.is_empty() {
        return false;
    }

    let mut p = printbuffer {
        buffer: vec![0u8; length as usize],
        length: length as usize,
        offset: 0,
        depth: 0,
        noalloc: true,
        format,
        hooks: GLOBAL_HOOKS.lock().unwrap().clone(),
    };

    if !print_value(item, &mut p) {
        return false;
    }

    let copy_len = std::cmp::min(p.offset, buffer.len());
    buffer[..copy_len].copy_from_slice(&p.buffer[..copy_len]);
    true
}

fn parse_value(item: &mut cJSON, input_buffer: &mut parse_buffer) -> bool {
    if input_buffer.content.is_empty() {
        return false;
    }

    if can_read(input_buffer, 4)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 4] == b"null"
    {
        item.type_ = cJSON_NULL;
        input_buffer.offset += 4;
        return true;
    }

    if can_read(input_buffer, 5)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 5] == b"false"
    {
        item.type_ = cJSON_False;
        input_buffer.offset += 5;
        return true;
    }

    if can_read(input_buffer, 4)
        && &input_buffer.content[input_buffer.offset..input_buffer.offset + 4] == b"true"
    {
        item.type_ = cJSON_True;
        item.valueint = 1;
        input_buffer.offset += 4;
        return true;
    }

    if can_access_at_index(input_buffer, 0) && buffer_at_offset(input_buffer) == b'"' {
        return parse_string(item, input_buffer);
    }

    if can_access_at_index(input_buffer, 0) {
        let ch = buffer_at_offset(input_buffer);
        if ch == b'-' || (ch >= b'0' && ch <= b'9') {
            return parse_number(item, input_buffer);
        }
    }

    if can_access_at_index(input_buffer, 0) && buffer_at_offset(input_buffer) == b'[' {
        return parse_array(item, input_buffer);
    }

    if can_access_at_index(input_buffer, 0) && buffer_at_offset(input_buffer) == b'{' {
        return parse_object(item, input_buffer);
    }

    false
}

fn print_value(item: &cJSON, output_buffer: &mut printbuffer) -> bool {
    if output_buffer.buffer.is_empty() {
        return false;
    }

    match item.type_ & 0xFF {
        cJSON_NULL => {
            if !ensure(output_buffer, 5) {
                return false;
            }
            let null_bytes = b"null";
            for (i, &b) in null_bytes.iter().enumerate() {
                output_buffer.buffer[output_buffer.offset + i] = b;
            }
            output_buffer.offset += 4;
            true
        }
        cJSON_False => {
            if !ensure(output_buffer, 6) {
                return false;
            }
            let false_bytes = b"false";
            for (i, &b) in false_bytes.iter().enumerate() {
                output_buffer.buffer[output_buffer.offset + i] = b;
            }
            output_buffer.offset += 5;
            true
        }
        cJSON_True => {
            if !ensure(output_buffer, 5) {
                return false;
            }
            let true_bytes = b"true";
            for (i, &b) in true_bytes.iter().enumerate() {
                output_buffer.buffer[output_buffer.offset + i] = b;
            }
            output_buffer.offset += 4;
            true
        }
        cJSON_Number => print_number(item, output_buffer),
        cJSON_String => print_string(item, output_buffer),
        cJSON_Array => print_array(item, output_buffer),
        cJSON_Object => print_object(item, output_buffer),
        cJSON_Raw => {
            if let Some(ref vs) = item.valuestring {
                if !ensure(output_buffer, vs.len()) {
                    return false;
                }
                let bytes = vs.as_bytes();
                for (i, &b) in bytes.iter().enumerate() {
                    output_buffer.buffer[output_buffer.offset + i] = b;
                }
                output_buffer.offset += bytes.len();
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn parse_array(item: &mut cJSON, input_buffer: &mut parse_buffer) -> bool {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return false;
    }
    input_buffer.depth += 1;

    if input_buffer.offset >= input_buffer.content.len()
        || input_buffer.content[input_buffer.offset] != b'['
    {
        input_buffer.depth -= 1;
        return false;
    }

    input_buffer.offset += 1;
    buffer_skip_whitespace(input_buffer);

    if input_buffer.offset < input_buffer.content.len()
        && input_buffer.content[input_buffer.offset] == b']'
    {
        input_buffer.depth -= 1;
        item.type_ = cJSON_Array;
        input_buffer.offset += 1;
        return true;
    }

    if input_buffer.offset >= input_buffer.content.len() {
        input_buffer.offset = input_buffer.offset.saturating_sub(1);
        input_buffer.depth -= 1;
        return false;
    }

    input_buffer.offset = input_buffer.offset.saturating_sub(1);
    let mut items: Vec<Box<cJSON>> = Vec::new();

    loop {
        input_buffer.offset += 1;
        buffer_skip_whitespace(input_buffer);

        let mut new_item = Box::new(cJSON_New_Item());

        if !parse_value(&mut new_item, input_buffer) {
            input_buffer.depth -= 1;
            cJSON_Delete(items.into_iter().next());
            return false;
        }

        items.push(new_item);
        buffer_skip_whitespace(input_buffer);

        if input_buffer.offset >= input_buffer.content.len()
            || input_buffer.content[input_buffer.offset] != b','
        {
            break;
        }
    }

    if input_buffer.offset >= input_buffer.content.len()
        || input_buffer.content[input_buffer.offset] != b']'
    {
        input_buffer.depth -= 1;
        let mut head = items.into_iter().next();
        cJSON_Delete(head);
        return false;
    }

    input_buffer.depth -= 1;

    if !items.is_empty() {
        let mut first = items.remove(0);
        let mut current = &mut first;

        for mut node in items {
            current.next = Some(node);
            if let Some(ref mut next) = current.next {
                current = next;
            }
        }

        if let Some(ref mut first_node) = first.child {
            current.prev = Some(Box::new((*first_node).clone()));
        }

        item.type_ = cJSON_Array;
        item.child = Some(first);
    } else {
        item.type_ = cJSON_Array;
    }

    input_buffer.offset += 1;
    true
}

fn print_array(item: &cJSON, output_buffer: &mut printbuffer) -> bool {
    if output_buffer.buffer.is_empty() {
        return false;
    }

    if !ensure(output_buffer, 1) {
        return false;
    }
    output_buffer.buffer[output_buffer.offset] = b'[';
    output_buffer.offset += 1;
    output_buffer.depth += 1;

    let mut current_element = item.child.as_ref();

    while let Some(elem) = current_element {
        if !print_value(elem, output_buffer) {
            output_buffer.depth -= 1;
            return false;
        }
        update_offset(output_buffer);

        if elem.next.is_some() {
            let length = if output_buffer.format { 2 } else { 1 };
            if !ensure(output_buffer, length + 1) {
                output_buffer.depth -= 1;
                return false;
            }
            output_buffer.buffer[output_buffer.offset] = b',';
            output_buffer.offset += 1;
            if output_buffer.format {
                output_buffer.buffer[output_buffer.offset] = b' ';
                output_buffer.offset += 1;
            }
        }

        current_element = elem.next.as_ref();
    }

    if !ensure(output_buffer, 2) {
        output_buffer.depth -= 1;
        return false;
    }
    output_buffer.buffer[output_buffer.offset] = b']';
    output_buffer.offset += 1;
    output_buffer.depth -= 1;
    true
}

fn parse_object(item: &mut cJSON, input_buffer: &mut parse_buffer) -> bool {
    if input_buffer.depth >= CJSON_NESTING_LIMIT {
        return false;
    }
    input_buffer.depth += 1;

    if input_buffer.offset >= input_buffer.content.len()
        || input_buffer.content[input_buffer.offset] != b'{'
    {
        input_buffer.depth -= 1;
        return false;
    }

    input_buffer.offset += 1;
    buffer_skip_whitespace(input_buffer);

    if input_buffer.offset < input_buffer.content.len()
        && input_buffer.content[input_buffer.offset] == b'}'
    {
        input_buffer.depth -= 1;
        item.type_ = cJSON_Object;
        input_buffer.offset += 1;
        return true;
    }

    if input_buffer.offset >= input_buffer.content.len() {
        input_buffer.offset = input_buffer.offset.saturating_sub(1);
        input_buffer.depth -= 1;
        return false;
    }

    input_buffer.offset = input_buffer.offset.saturating_sub(1);
    let mut items: Vec<Box<cJSON>> = Vec::new();

    loop {
        if input_buffer.offset + 1 >= input_buffer.content.len() {
            input_buffer.depth -= 1;
            let mut head = items.into_iter().next();
            cJSON_Delete(head);
            return false;
        }

        input_buffer.offset += 1;
        buffer_skip_whitespace(input_buffer);

        let mut new_item = Box::new(cJSON_New_Item());

        if !parse_string(&mut new_item, input_buffer) {
            input_buffer.depth -= 1;
            let mut head = items.into_iter().next();
            cJSON_Delete(head);
            return false;
        }
        buffer_skip_whitespace(input_buffer);

        new_item.string = new_item.valuestring.clone();
        new_item.valuestring = None;

        if input_buffer.offset >= input_buffer.content.len()
            || input_buffer.content[input_buffer.offset] != b':'
        {
            input_buffer.depth -= 1;
            let mut head = items.into_iter().next();
            cJSON_Delete(head);
            return false;
        }

        input_buffer.offset += 1;
        buffer_skip_whitespace(input_buffer);

        if !parse_value(&mut new_item, input_buffer) {
            input_buffer.depth -= 1;
            let mut head = items.into_iter().next();
            cJSON_Delete(head);
            return false;
        }
        buffer_skip_whitespace(input_buffer);

        items.push(new_item);

        if input_buffer.offset >= input_buffer.content.len()
            || input_buffer.content[input_buffer.offset] != b','
        {
            break;
        }
    }

    if input_buffer.offset >= input_buffer.content.len()
        || input_buffer.content[input_buffer.offset] != b'}'
    {
        input_buffer.depth -= 1;
        let mut head = items.into_iter().next();
        cJSON_Delete(head);
        return false;
    }

    input_buffer.depth -= 1;

    if !items.is_empty() {
        let mut first = items.remove(0);
        let mut current = &mut first;

        for mut node in items {
            current.next = Some(node);
            if let Some(ref mut next) = current.next {
                current = next;
            }
        }

        if let Some(ref mut first_node) = first.child {
            current.prev = Some(Box::new((*first_node).clone()));
        }

        item.type_ = cJSON_Object;
        item.child = Some(first);
    } else {
        item.type_ = cJSON_Object;
    }

    input_buffer.offset += 1;
    true
}

fn print_object(item: &cJSON, output_buffer: &mut printbuffer) -> bool {
    if output_buffer.buffer.is_empty() {
        return false;
    }

    let length = if output_buffer.format { 2 } else { 1 };
    if !ensure(output_buffer, length + 1) {
        return false;
    }

    output_buffer.buffer[output_buffer.offset] = b'{';
    output_buffer.offset += 1;
    output_buffer.depth += 1;

    if output_buffer.format {
        output_buffer.buffer[output_buffer.offset] = b'\n';
        output_buffer.offset += 1;
    }

    let mut current_item = item.child.as_ref();

    while let Some(curr) = current_item {
        if output_buffer.format {
            if !ensure(output_buffer, output_buffer.depth) {
                output_buffer.depth -= 1;
                return false;
            }
            for _ in 0..output_buffer.depth {
                output_buffer.buffer[output_buffer.offset] = b'\t';
                output_buffer.offset += 1;
            }
        }

        if let Some(ref key) = curr.string {
            if !print_string_ptr(Some(key), output_buffer) {
                output_buffer.depth -= 1;
                return false;
            }
        }
        update_offset(output_buffer);

        let length = if output_buffer.format { 2 } else { 1 };
        if !ensure(output_buffer, length) {
            output_buffer.depth -= 1;
            return false;
        }

        output_buffer.buffer[output_buffer.offset] = b':';
        output_buffer.offset += 1;
        if output_buffer.format {
            output_buffer.buffer[output_buffer.offset] = b'\t';
            output_buffer.offset += 1;
        }

        if !print_value(curr, output_buffer) {
            output_buffer.depth -= 1;
            return false;
        }
        update_offset(output_buffer);

        let length = if curr.next.is_some() { 1 } else { 0 }
            + if output_buffer.format { 1 } else { 0 };
        if !ensure(output_buffer, length + 1) {
            output_buffer.depth -= 1;
            return false;
        }

        if curr.next.is_some() {
            output_buffer.buffer[output_buffer.offset] = b',';
            output_buffer.offset += 1;
        }
        if output_buffer.format {
            output_buffer.buffer[output_buffer.offset] = b'\n';
            output_buffer.offset += 1;
        }

        current_item = curr.next.as_ref();
    }

    let needed = if output_buffer.format {
        output_buffer.depth + 1
    } else {
        2
    };
    if !ensure(output_buffer, needed) {
        output_buffer.depth -= 1;
        return false;
    }

    if output_buffer.format {
        for _ in 0..(output_buffer.depth.saturating_sub(1)) {
            output_buffer.buffer[output_buffer.offset] = b'\t';
            output_buffer.offset += 1;
        }
    }

    output_buffer.buffer[output_buffer.offset] = b'}';
    output_buffer.offset += 1;
    output_buffer.depth -= 1;
    true
}

pub fn cJSON_GetArraySize(array: &cJSON) -> i32 {
    let mut size = 0;
    let mut child = array.child.as_ref();

    while let Some(c) = child {
        size += 1;
        child = c.next.as_ref();
    }

    size
}

fn get_array_item(array: &cJSON, index: usize) -> Option<&cJSON> {
    let mut current_child = array.child.as_ref();
    let mut idx = index;

    while let Some(child) = current_child {
        if idx == 0 {
            return Some(child);
        }
        idx -= 1;
        current_child = child.next.as_ref();
    }

    None
}

pub fn cJSON_GetArrayItem(array: &cJSON, index: i32) -> Option<&cJSON> {
    if index < 0 {
        None
    } else {
        get_array_item(array, index as usize)
    }
}

fn get_object_item(object: &cJSON, name: &str, case_sensitive: bool) -> Option<&cJSON> {
    let mut current_element = object.child.as_ref();

    if case_sensitive {
        while let Some(elem) = current_element {
            if let Some(ref key) = elem.string {
                if key == name {
                    return Some(elem);
                }
            }
            current_element = elem.next.as_ref();
        }
    } else {
        while let Some(elem) = current_element {
            if let Some(ref key) = elem.string {
                if key.to_lowercase() == name.to_lowercase() {
                    return Some(elem);
                }
            }
            current_element = elem.next.as_ref();
        }
    }

    None
}

pub fn cJSON_GetObjectItem(object: &cJSON, string: &str) -> Option<&cJSON> {
    get_object_item(object, string, false)
}

pub fn cJSON_GetObjectItemCaseSensitive(object: &cJSON, string: &str) -> Option<&cJSON> {
    get_object_item(object, string, true)
}

pub fn cJSON_HasObjectItem(object: &cJSON, string: &str) -> cJSON_bool {
    if cJSON_GetObjectItem(object, string).is_some() { 1 } else { 0 }
}

fn suffix_object(prev: &mut cJSON, item: Box<cJSON>) {
    prev.next = Some(item);
}

fn create_reference(item: &cJSON) -> Box<cJSON> {
    let mut reference = Box::new(item.clone());
    reference.string = None;
    reference.type_ |= cJSON_IsReference;
    reference.next = None;
    reference.prev = None;
    reference
}

fn add_item_to_array(array: &mut cJSON, item: Box<cJSON>) -> bool {
    if array as *mut cJSON as *const cJSON == &*item as *const cJSON {
        return false;
    }

    if let Some(ref mut child) = array.child {
        let mut current = child;
        loop {
            if current.next.is_none() {
                current.next = Some(item);
                break;
            } else {
                if let Some(ref mut next) = current.next {
                    current = next;
                } else {
                    break;
                }
            }
        }
    } else {
        array.child = Some(item);
    }

    true
}

pub fn cJSON_AddItemToArray(array: &mut cJSON, item: Box<cJSON>) -> cJSON_bool {
    if add_item_to_array(array, item) { 1 } else { 0 }
}

fn cast_away_const(s: &str) -> String {
    s.to_string()
}

fn add_item_to_object(
    object: &mut cJSON,
    string: &str,
    mut item: Box<cJSON>,
    constant_key: bool,
) -> bool {
    if object as *mut cJSON as *const cJSON == &*item as *const cJSON {
        return false;
    }

    if constant_key {
        item.string = Some(string.to_string());
        item.type_ |= cJSON_StringIsConst;
    } else {
        item.string = Some(cJSON_strdup(string));
        item.type_ &= !cJSON_StringIsConst;
    }

    if let Some(ref mut child) = object.child {
        let mut current = child;
        loop {
            if current.next.is_none() {
                current.next = Some(item);
                break;
            } else {
                if let Some(ref mut next) = current.next {
                    current = next;
                } else {
                    break;
                }
            }
        }
    } else {
        object.child = Some(item);
    }

    true
}

pub fn cJSON_AddItemToObject(
    object: &mut cJSON,
    string: &str,
    item: Box<cJSON>,
) -> cJSON_bool {
    if add_item_to_object(object, string, item, false) {
        1
    } else {
        0
    }
}

pub fn cJSON_AddItemToObjectCS(
    object: &mut cJSON,
    string: &str,
    item: Box<cJSON>,
) -> cJSON_bool {
    if add_item_to_object(object, string, item, true) {
        1
    } else {
        0
    }
}

pub fn cJSON_AddItemReferenceToArray(array: &mut cJSON, item: &cJSON) -> cJSON_bool {
    let reference = create_reference(item);
    if add_item_to_array(array, reference) { 1 } else { 0 }
}

pub fn cJSON_AddItemReferenceToObject(
    object: &mut cJSON,
    string: &str,
    item: &cJSON,
) -> cJSON_bool {
    let reference = create_reference(item);
    if add_item_to_object(object, string, reference, false) {
        1
    } else {
        0
    }
}

pub fn cJSON_AddNullToObject(object: &mut cJSON, name: &str) -> Option<Box<cJSON>> {
    let null = Box::new(cJSON {
        type_: cJSON_NULL,
        ..Default::default()
    });
    if add_item_to_object(object, name, null.clone(), false) {
        Some(null)
    } else {
        None
    }
}

pub fn cJSON_AddTrueToObject(object: &mut cJSON, name: &str) -> Option<Box<cJSON>> {
    let true_item = Box::new(cJSON {
        type_: cJSON_True,
        ..Default::default()
    });
    if add_item_to_object(object, name, true_item.clone(), false) {
        Some(true_item)
    } else {
        None
    }
}

pub fn cJSON_AddFalseToObject(object: &mut cJSON, name: &str) -> Option<Box<cJSON>> {
    let false_item = Box::new(cJSON {
        type_: cJSON_False,
        ..Default::default()
    });
    if add_item_to_object(object, name, false_item.clone(), false) {
        Some(false_item)
    } else {
        None
    }
}

pub fn cJSON_AddBoolToObject(object: &mut cJSON, name: &str, boolean: bool) -> Option<Box<cJSON>> {
    let bool_item = Box::new(cJSON {
        type_: if boolean { cJSON_True } else { cJSON_False },
        ..Default::default()
    });
    if add_item_to_object(object, name, bool_item.clone(), false) {
        Some(bool_item)
    } else {
        None
    }
}

pub fn cJSON_AddNumberToObject(object: &mut cJSON, name: &str, number: f64) -> Option<Box<cJSON>> {
    let mut number_item = Box::new(cJSON {
        type_: cJSON_Number,
        valuedouble: number,
        ..Default::default()
    });
    if number >= i32::MAX as f64 {
        number_item.valueint = i32::MAX;
    } else if number <= i32::MIN as f64 {
        number_item.valueint = i32::MIN;
    } else {
        number_item.valueint = number as i32;
    }
    if add_item_to_object(object, name, number_item.clone(), false) {
        Some(number_item)
    } else {
        None
    }
}

pub fn cJSON_AddStringToObject(
    object: &mut cJSON,
    name: &str,
    string: &str,
) -> Option<Box<cJSON>> {
    let string_item = Box::new(cJSON {
        type_: cJSON_String,
        valuestring: Some(string.to_string()),
        ..Default::default()
    });
    if add_item_to_object(object, name, string_item.clone(), false) {
        Some(string_item)
    } else {
        None
    }
}

pub fn cJSON_AddRawToObject(object: &mut cJSON, name: &str, raw: &str) -> Option<Box<cJSON>> {
    let raw_item = Box::new(cJSON {
        type_: cJSON_Raw,
        valuestring: Some(raw.to_string()),
        ..Default::default()
    });
    if add_item_to_object(object, name, raw_item.clone(), false) {
        Some(raw_item)
    } else {
        None
    }
}

pub fn cJSON_AddObjectToObject(object: &mut cJSON, name: &str) -> Option<Box<cJSON>> {
    let object_item = Box::new(cJSON {
        type_: cJSON_Object,
        ..Default::default()
    });
    if add_item_to_object(object, name, object_item.clone(), false) {
        Some(object_item)
    } else {
        None
    }
}

pub fn cJSON_AddArrayToObject(object: &mut cJSON, name: &str) -> Option<Box<cJSON>> {
    let array_item = Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    });
    if add_item_to_object(object, name, array_item.clone(), false) {
        Some(array_item)
    } else {
        None
    }
}

pub fn cJSON_DetachItemViaPointer(parent: &mut cJSON, item: &cJSON) -> Option<Box<cJSON>> {
    if parent.child.is_none() {
        return None;
    }

    let mut current = &mut parent.child;
    while let Some(ref mut c) = current {
        if std::ptr::eq(c.as_ref(), item) {
            let mut detached = current.take();
            if let Some(ref mut d) = detached {
                if let Some(ref mut next) = d.next.take() {
                    *current = Some(next.clone());
                }
            }
            return detached;
        }
        current = &mut c.next;
    }

    None
}

pub fn cJSON_DetachItemFromArray(array: &mut cJSON, which: i32) -> Option<Box<cJSON>> {
    if which < 0 {
        return None;
    }

    let mut index = which as usize;
    let mut current = array.child.take();

    if index == 0 {
        if let Some(mut c) = current {
            array.child = c.next.take();
            c.next = None;
            c.prev = None;
            return Some(c);
        }
        return None;
    }

    while let Some(mut c) = current {
        index -= 1;
        if index == 0 {
            let detached = c.next.take();
            if let Some(mut d) = detached {
                c.next = d.next.take();
                d.next = None;
                d.prev = None;
                return Some(d);
            }
            return None;
        }
        current = c.next.take();
    }

    None
}

pub fn cJSON_DeleteItemFromArray(array: &mut cJSON, which: i32) {
    if let Some(item) = cJSON_DetachItemFromArray(array, which) {
        cJSON_Delete(Some(item));
    }
}

pub fn cJSON_DetachItemFromObject(object: &mut cJSON, string: &str) -> Option<Box<cJSON>> {
    if let Some(item) = get_object_item(object, string, false) {
        cJSON_DetachItemViaPointer(object, item)
    } else {
        None
    }
}

pub fn cJSON_DetachItemFromObjectCaseSensitive(
    object: &mut cJSON,
    string: &str,
) -> Option<Box<cJSON>> {
    if let Some(item) = get_object_item(object, string, true) {
        cJSON_DetachItemViaPointer(object, item)
    } else {
        None
    }
}

pub fn cJSON_DeleteItemFromObject(object: &mut cJSON, string: &str) {
    if let Some(item) = cJSON_DetachItemFromObject(object, string) {
        cJSON_Delete(Some(item));
    }
}

pub fn cJSON_DeleteItemFromObjectCaseSensitive(object: &mut cJSON, string: &str) {
    if let Some(item) = cJSON_DetachItemFromObjectCaseSensitive(object, string) {
        cJSON_Delete(Some(item));
    }
}

pub fn cJSON_InsertItemInArray(
    array: &mut cJSON,
    which: i32,
    newitem: Box<cJSON>,
) -> cJSON_bool {
    if which < 0 {
        return 0;
    }

    let mut index = which as usize;
    let mut current = array.child.as_mut();

    while let Some(c) = current {
        if index == 0 {
            let mut item = newitem;
            item.next = c.next.take();
            c.next = Some(item);
            return 1;
        }
        index -= 1;
        current = c.next.as_mut();
    }

    if index == 0 {
        array.child = Some(newitem);
        return 1;
    }

    0
}

pub fn cJSON_ReplaceItemViaPointer(
    parent: &mut cJSON,
    item: &cJSON,
    replacement: Box<cJSON>,
) -> cJSON_bool {
    if parent.child.is_none() {
        return 0;
    }

    let mut current = parent.child.take();
    while let Some(mut c) = current {
        if std::ptr::eq(c.as_ref(), item) {
            let mut new_item = replacement;
            new_item.next = c.next.take();
            new_item.prev = c.prev.take();
            parent.child = Some(new_item);
            return 1;
        }
        current = c.next.take();
    }

    0
}

pub fn cJSON_ReplaceItemInArray(
    array: &mut cJSON,
    which: i32,
    newitem: Box<cJSON>,
) -> cJSON_bool {
    if which < 0 {
        return 0;
    }

    let mut index = which as usize;
    let mut current = array.child.take();

    while let Some(mut c) = current {
        if index == 0 {
            let mut new_item = newitem;
            new_item.next = c.next.take();
            new_item.prev = c.prev.take();
            array.child = Some(new_item);
            return 1;
        }
        index -= 1;
        current = c.next.take();
    }

    0
}

fn replace_item_in_object(
    object: &mut cJSON,
    string: &str,
    replacement: Box<cJSON>,
    case_sensitive: bool,
) -> cJSON_bool {
    if let Some(item) = get_object_item(object, string, case_sensitive) {
        if let Some(mut new_item) = Some(replacement) {
            new_item.string = Some(string.to_string());
            cJSON_ReplaceItemViaPointer(object, item, new_item)
        } else {
            0
        }
    } else {
        0
    }
}

pub fn cJSON_ReplaceItemInObject(
    object: &mut cJSON,
    string: &str,
    newitem: Box<cJSON>,
) -> cJSON_bool {
    replace_item_in_object(object, string, newitem, false)
}

pub fn cJSON_ReplaceItemInObjectCaseSensitive(
    object: &mut cJSON,
    string: &str,
    newitem: Box<cJSON>,
) -> cJSON_bool {
    replace_item_in_object(object, string, newitem, true)
}

pub fn cJSON_CreateNull() -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_NULL,
        ..Default::default()
    })
}

pub fn cJSON_CreateTrue() -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_True,
        ..Default::default()
    })
}

pub fn cJSON_CreateFalse() -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_False,
        ..Default::default()
    })
}

pub fn cJSON_CreateBool(boolean: bool) -> Box<cJSON> {
    Box::new(cJSON {
        type_: if boolean { cJSON_True } else { cJSON_False },
        ..Default::default()
    })
}

pub fn cJSON_CreateNumber(num: f64) -> Box<cJSON> {
    let mut item = Box::new(cJSON {
        type_: cJSON_Number,
        valuedouble: num,
        ..Default::default()
    });
    if num >= i32::MAX as f64 {
        item.valueint = i32::MAX;
    } else if num <= i32::MIN as f64 {
        item.valueint = i32::MIN;
    } else {
        item.valueint = num as i32;
    }
    item
}

pub fn cJSON_CreateString(string: &str) -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_String,
        valuestring: Some(string.to_string()),
        ..Default::default()
    })
}

pub fn cJSON_CreateStringReference(string: &str) -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_String | cJSON_IsReference,
        valuestring: Some(string.to_string()),
        ..Default::default()
    })
}

pub fn cJSON_CreateObjectReference(child: &cJSON) -> Box<cJSON> {
    let mut item = Box::new(cJSON {
        type_: cJSON_Object | cJSON_IsReference,
        ..Default::default()
    });
    item.child = Some(Box::new(child.clone()));
    item
}

pub fn cJSON_CreateArrayReference(child: &cJSON) -> Box<cJSON> {
    let mut item = Box::new(cJSON {
        type_: cJSON_Array | cJSON_IsReference,
        ..Default::default()
    });
    item.child = Some(Box::new(child.clone()));
    item
}

pub fn cJSON_CreateRaw(raw: &str) -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_Raw,
        valuestring: Some(raw.to_string()),
        ..Default::default()
    })
}

pub fn cJSON_CreateArray() -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    })
}

pub fn cJSON_CreateObject() -> Box<cJSON> {
    Box::new(cJSON {
        type_: cJSON_Object,
        ..Default::default()
    })
}

pub fn cJSON_CreateIntArray(numbers: &[i32]) -> Option<Box<cJSON>> {
    let mut a = Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    });

    for &num in numbers {
        let n = cJSON_CreateNumber(num as f64);
        if a.child.is_none() {
            a.child = Some(n);
        } else {
            let mut current = a.child.as_mut();
            while let Some(c) = current {
                if c.next.is_none() {
                    c.next = Some(n);
                    break;
                }
                current = c.next.as_mut();
            }
        }
    }

    Some(a)
}

pub fn cJSON_CreateFloatArray(numbers: &[f32]) -> Option<Box<cJSON>> {
    let mut a = Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    });

    for &num in numbers {
        let n = cJSON_CreateNumber(num as f64);
        if a.child.is_none() {
            a.child = Some(n);
        } else {
            let mut current = a.child.as_mut();
            while let Some(c) = current {
                if c.next.is_none() {
                    c.next = Some(n);
                    break;
                }
                current = c.next.as_mut();
            }
        }
    }

    Some(a)
}

pub fn cJSON_CreateDoubleArray(numbers: &[f64]) -> Option<Box<cJSON>> {
    let mut a = Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    });

    for &num in numbers {
        let n = cJSON_CreateNumber(num);
        if a.child.is_none() {
            a.child = Some(n);
        } else {
            let mut current = a.child.as_mut();
            while let Some(c) = current {
                if c.next.is_none() {
                    c.next = Some(n);
                    break;
                }
                current = c.next.as_mut();
            }
        }
    }

    Some(a)
}

pub fn cJSON_CreateStringArray(strings: &[&str]) -> Option<Box<cJSON>> {
    let mut a = Box::new(cJSON {
        type_: cJSON_Array,
        ..Default::default()
    });

    for &s in strings {
        let n = cJSON_CreateString(s);
        if a.child.is_none() {
            a.child = Some(n);
        } else {
            let mut current = a.child.as_mut();
            while let Some(c) = current {
                if c.next.is_none() {
                    c.next = Some(n);
                    break;
                }
                current = c.next.as_mut();
            }
        }
    }

    Some(a)
}

pub fn cJSON_Duplicate(item: &cJSON, recurse: bool) -> Option<Box<cJSON>> {
    cJSON_Duplicate_rec(item, 0, recurse)
}

fn cJSON_Duplicate_rec(item: &cJSON, depth: usize, recurse: bool) -> Option<Box<cJSON>> {
    if depth >= CJSON_CIRCULAR_LIMIT {
        return None;
    }

    let mut newitem = Box::new(cJSON {
        type_: item.type_ & !cJSON_IsReference,
        valueint: item.valueint,
        valuedouble: item.valuedouble,
        valuestring: item.valuestring.clone(),
        string: item.string.clone(),
        ..Default::default()
    });

    if !recurse {
        return Some(newitem);
    }

    let mut child = item.child.as_ref();
    while let Some(c) = child {
        if let Some(newchild) = cJSON_Duplicate_rec(c, depth + 1, true) {
            if newitem.child.is_none() {
                newitem.child = Some(newchild);
            } else {
                let mut current = newitem.child.as_mut();
                while let Some(curr) = current {
                    if curr.next.is_none() {
                        curr.next = Some(newchild);
                        break;
                    }
                    current = curr.next.as_mut();
                }
            }
        }
        child = c.next.as_ref();
    }

    Some(newitem)
}

fn skip_oneline_comment(input: &mut &str) {
    *input = &input[2..];
    while !input.is_empty() {
        if input.starts_with('\n') {
            *input = &input[1..];
            return;
        }
        *input = &input[1..];
    }
}

fn skip_multiline_comment(input: &mut &str) {
    *input = &input[2..];
    while input.len() >= 2 {
        if input.starts_with("*/") {
            *input = &input[2..];
            return;
        }
        *input = &input[1..];
    }
}

fn minify_string(input: &mut &str, output: &mut String) {
    if input.is_empty() {
        return;
    }
    output.push('"');
    *input = &input[1..];

    while !input.is_empty() {
        let ch = input.chars().next().unwrap();
        output.push(ch);
        *input = &input[1..];

        if ch == '"' {
            return;
        } else if ch == '\\' && !input.is_empty() {
            let next_ch = input.chars().next().unwrap();
            output.push(next_ch);
            *input = &input[1..];
        }
    }
}

pub fn cJSON_Minify(json: &mut String) {
    let mut result = String::new();
    let mut input = json.as_str();

    while !input.is_empty() {
        let ch = input.chars().next().unwrap();
        match ch {
            ' ' | '\t' | '\r' | '\n' => {
                input = &input[1..];
            }
            '/' => {
                if input.len() > 1 {
                    let next = input.chars().nth(1).unwrap();
                    if next == '/' {
                        skip_oneline_comment(&mut input);
                    } else if next == '*' {
                        skip_multiline_comment(&mut input);
                    } else {
                        result.push(ch);
                        input = &input[1..];
                    }
                } else {
                    result.push(ch);
                    input = &input[1..];
                }
            }
            '"' => {
                minify_string(&mut input, &mut result);
            }
            _ => {
                result.push(ch);
                input = &input[1..];
            }
        }
    }

    *json = result;
}

pub fn cJSON_IsInvalid(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_Invalid { 1 } else { 0 }
}

pub fn cJSON_IsFalse(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_False { 1 } else { 0 }
}

pub fn cJSON_IsTrue(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_True { 1 } else { 0 }
}

pub fn cJSON_IsBool(item: &cJSON) -> cJSON_bool {
    if (item.type_ & (cJSON_True | cJSON_False)) != 0 { 1 } else { 0 }
}

pub fn cJSON_IsNull(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_NULL { 1 } else { 0 }
}

pub fn cJSON_IsNumber(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_Number { 1 } else { 0 }
}

pub fn cJSON_IsString(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_String { 1 } else { 0 }
}

pub fn cJSON_IsArray(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_Array { 1 } else { 0 }
}

pub fn cJSON_IsObject(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_Object { 1 } else { 0 }
}

pub fn cJSON_IsRaw(item: &cJSON) -> cJSON_bool {
    if (item.type_ & 0xFF) == cJSON_Raw { 1 } else { 0 }
}

pub fn cJSON_Compare(a: &cJSON, b: &cJSON, case_sensitive: bool) -> cJSON_bool {
    if (a.type_ & 0xFF) != (b.type_ & 0xFF) {
        return 0;
    }

    match a.type_ & 0xFF {
        cJSON_False | cJSON_True | cJSON_NULL => 1,
        cJSON_Number => {
            if compare_double(a.valuedouble, b.valuedouble) {
                1
            } else {
                0
            }
        }
        cJSON_String | cJSON_Raw => {
            if let (Some(ref av), Some(ref bv)) = (&a.valuestring, &b.valuestring) {
                if av == bv { 1 } else { 0 }
            } else {
                0
            }
        }
        cJSON_Array => {
            let mut a_elem = a.child.as_ref();
            let mut b_elem = b.child.as_ref();

            loop {
                match (a_elem, b_elem) {
                    (None, None) => return 1,
                    (None, Some(_)) | (Some(_), None) => return 0,
                    (Some(ae), Some(be)) => {
                        if cJSON_Compare(ae, be, case_sensitive) == 0 {
                            return 0;
                        }
                        a_elem = ae.next.as_ref();
                        b_elem = be.next.as_ref();
                    }
                }
            }
        }
        cJSON_Object => {
            let mut a_elem = a.child.as_ref();
            while let Some(ae) = a_elem {
                if let Some(ref key) = ae.string {
                    if let Some(be) = get_object_item(b, key, case_sensitive) {
                        if cJSON_Compare(ae, be, case_sensitive) == 0 {
                            return 0;
                        }
                    } else {
                        return 0;
                    }
                }
                a_elem = ae.next.as_ref();
            }

            let mut b_elem = b.child.as_ref();
            while let Some(be) = b_elem {
                if let Some(ref key) = be.string {
                    if let Some(ae) = get_object_item(a, key, case_sensitive) {
                        if cJSON_Compare(be, ae, case_sensitive) == 0 {
                            return 0;
                        }
                    } else {
                        return 0;
                    }
                }
                b_elem = be.next.as_ref();
            }

            1
        }
        _ => 0,
    }
}

pub fn cJSON_malloc(size: usize) -> *mut u8 {
    GLOBAL_HOOKS.lock().unwrap().allocate(size)
}

pub fn cJSON_free(object: *mut u8) {
    GLOBAL_HOOKS.lock().unwrap().deallocate(object);
}