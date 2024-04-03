extern "C" {
	pub fn wordexp(_: *const libc::c_char, _: &mut wordexp_t, _: i32) -> i32;

	pub fn wordfree(_: &mut wordexp_t);

	// /* Do word expansion of WORDS into PWORDEXP.  */
	// extern int wordexp (const char *__restrict __words,
	// 	    wordexp_t *__restrict __pwordexp, int __flags);

	// /* Free the storage allocated by a `wordexp' call.  */
	// extern void wordfree (wordexp_t *__wordexp) __THROW;
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct wordexp_t {
	/* Count of words matched.  */
	pub we_wordc: libc::size_t,
	/* List of expanded words.  */
	pub we_wordv: *const *const libc::c_char,
	/* Slots to reserve in `we_wordv'.  */
	pub we_offs: libc::size_t,

	// typedef struct
  // {
  //   size_t we_wordc;		/* Count of words matched.  */
  //   char **we_wordv;		/* List of expanded words.  */
  //   size_t we_offs;		/* Slots to reserve in `we_wordv'.  */
  // } wordexp_t;
}

impl wordexp_t {
	pub fn new() -> Self {
		Self {
			we_wordc: 0,
			we_wordv: std::ptr::null(),
			we_offs: 0,
		}
	}
}

impl std::ops::Drop for wordexp_t {
	fn drop(&mut self) {
		unsafe {
			wordfree(self);
		}
	}
}