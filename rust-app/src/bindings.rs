use std::ffi::{CStr, CString};

/// typedef struct {
///     char* name;
///     uint32_t age;
/// } person_t;
#[repr(C)]
#[allow(non_camel_case_types)]
struct person_t {
    name: *mut cty::c_char,
    age: cty::uint32_t,
}

#[link(name = "library", kind = "static")]
extern "C" {
    /// person_t* person_new(char* name, uint32_t name_len, uint32_t age);
    fn person_new(
        name: *const cty::c_char,
        name_len: cty::uint32_t,
        age: cty::uint32_t,
    ) -> *const person_t;

    /// void person_free(person_t* person);
    fn person_free(person: *const person_t);

    /// char* person_fmt(person_t* person);
    fn person_fmt(person: *const person_t) -> *const cty::c_char;
}

pub struct Person {
    c_person: *const person_t,
}

impl Person {
    pub fn fmt(&self) -> Result<String, ()> {
        unsafe {
            let char_ptr = person_fmt(self.c_person);
            let c_str = CStr::from_ptr(char_ptr);
            let result = c_str.to_str().map_err(|_| ()).map(ToString::to_string);

            libc::free(char_ptr as *mut libc::c_void);

            result
        }
    }
}

impl Drop for Person {
    fn drop(&mut self) {
        unsafe {
            person_free(self.c_person);
        }
    }
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        let len = name.len();
        let c_name = CString::new(name).unwrap();

        unsafe {
            Person {
                c_person: person_new(c_name.as_ptr(), len as u32, age),
            }
        }
    }
}
