use super::{WordExp, wordexp};

#[test]
pub fn no_changes_001() {
	let s = "hello";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("hello", p.we_wordv[0].as_str());

}

#[test]
pub fn home_substitution_001() {
	// let s = "hello";
	std::env::set_var("HOME", "/home/wordexp");
	let s = "~";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("/home/wordexp", p.we_wordv[0].as_str());

}

#[test]
pub fn home_substitution_002() {
	// let s = "hello";
	std::env::set_var("HOME", "/home/wordexp");
	let s = "~/abcd";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("/home/wordexp/abcd", p.we_wordv[0].as_str());
}

#[test]
pub fn variable_substitution_001() {
	// let s = "hello";
	std::env::set_var("HOME", "/home/wordexp");
	let s = "$HOME";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("/home/wordexp", p.we_wordv[0].as_str());
}

#[test]
pub fn variable_substitution_002() {
	// let s = "hello";
	std::env::set_var("HOME", "/home/wordexp");
	let s = "${HOME}documents";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("/home/wordexpdocuments", p.we_wordv[0].as_str());
}

#[test]
pub fn variable_substitution_003() {
	// let s = "hello";
	// std::env::set_var("HOME", "/home/wordexp");
	let s = "${THISPFJSDFJSDKLFJSFHDKL}";
	let mut p = WordExp::new();
	let flags = super::WRDE_UNDEF;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Err(super::WRDE_BADVAL), res);
}

#[test]
pub fn command_substitution_001() {
	// let s = "hello";
	// std::env::set_var("HOME", "/home/wordexp");
	let s = "$(echo hello)";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(1, p.we_wordv.len());
	assert_eq!("hello", p.we_wordv[0].as_str());
}

#[test]
pub fn command_substitution_002() {
	// let s = "hello";
	// std::env::set_var("HOME", "/home/wordexp");
	let s = "$(echo hello)";
	let mut p = WordExp::new();
	let flags = super::WRDE_NOCMD;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Err(super::WRDE_CMDSUB), res);
}

#[test]
pub fn multiple_values_001() {
	// let s = "hello";
	// std::env::set_var("HOME", "/home/wordexp");
	let s = "hello world";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(2, p.we_wordv.len());
	assert_eq!("hello", p.we_wordv[0].as_str());
	assert_eq!("world", p.we_wordv[1].as_str());
}

#[test]
pub fn multiple_values_002() {
	// let s = "hello";
	std::env::set_var("USER", "wordexp");
	let s = "hello $USER";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Ok(()), res);
	assert_eq!(0, p.we_offs);
	assert_eq!(2, p.we_wordv.len());
	assert_eq!("hello", p.we_wordv[0].as_str());
	assert_eq!("wordexp", p.we_wordv[1].as_str());
}

#[test]
fn bad_char_001() {
	let s = "||||";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Err(super::WRDE_BADCHAR), res);
}

#[test]
fn bad_char_002() {
	let s = "cat file.txt | grep hello";
	let mut p = WordExp::new();
	let flags = 0;

	let res = wordexp(s, &mut p, flags);
	assert_eq!(Err(super::WRDE_BADCHAR), res);
}