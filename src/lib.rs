mod clib;

#[cfg(test)]
mod test;

use std::ffi::{CStr, CString};

pub const WRDE_DOOFFS: i32 = 1 << 0;	/* Insert PWORDEXP->we_offs NULLs.  */
pub const WRDE_APPEND: i32 = 1 << 1;	/* Append to results of a previous call.  */
pub const WRDE_NOCMD: i32 = 1 << 2;	/* Don't do command substitution.  */
pub const WRDE_REUSE: i32 = 1 << 3;	/* Reuse storage in PWORDEXP.  */
pub const WRDE_SHOWERR: i32 = 1 << 4;	/* Don't redirect stderr to /dev/null.  */
pub const WRDE_UNDEF: i32 = 1 << 5;	/* Error for expanding undefined variables.  */


// WRDE_NOSYS = -1;		/* Never used since we support `wordexp'.  */

pub const WRDE_NOSPACE: i32 = 1;		/* Ran out of memory.  */
pub const WRDE_BADCHAR: i32 = 2;		/* A metachar appears in the wrong place.  */
pub const WRDE_BADVAL: i32 = 3;		/* Undefined var reference with WRDE_UNDEF.  */
pub const WRDE_CMDSUB: i32 = 4;		/* Command substitution with WRDE_NOCMD.  */
pub const WRDE_SYNTAX: i32 = 5;			/* Shell syntax error.  */

pub struct WordExp {
	pub we_wordv: Vec<String>,
	pub we_offs: usize,
}

impl WordExp {
	pub fn new() -> Self {
		Self {
			we_wordv: Vec::new(),
			we_offs: 0,
		}
	}

	pub fn update(&mut self, p: &clib::wordexp_t) {
		self.we_offs = p.we_offs;
		self.we_wordv = Vec::new();
		
		let ptr: *const *const libc::c_char = p.we_wordv;

		for i in 0..(p.we_wordc + p.we_offs) {
			let item = unsafe {
				let nptr = ptr.add(i);
				if nptr == std::ptr::null() {
					"".to_string()
				} else {
					let cstr = CStr::from_ptr(*nptr);
					match cstr.to_str() {
						Ok(s) => s.to_string(),
						Err(_) => "".to_string(),
					}
				}
			};
			self.we_wordv.push(item);
		}
	}
}

pub fn wordexp(s: &str, p: &mut WordExp, flags: i32) -> Result<(), i32> {
	let cstr = match CString::new(s) {
		Ok(s) => s,
		Err(_) => return Err(WRDE_BADCHAR),
	};

	let c_ptr = cstr.as_ptr();

	let mut pt = clib::wordexp_t::new();

	let res = unsafe {
		clib::wordexp(c_ptr, &mut pt, flags)
	};

	if res == 0 {
		p.update(&pt);
		Ok(())
	} else {
		Err(res)
	}
}