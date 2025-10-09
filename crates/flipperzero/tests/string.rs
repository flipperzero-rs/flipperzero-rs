#![no_std]
#![no_main]

#[flipperzero_test::tests]
mod tests {
    use flipperzero::furi::string::FuriString as String;

    /*
    #[test]
    fn from_utf8() {
        let xs = b"hello".to_vec();
        assert_eq!(String::from_utf8(xs).unwrap(), String::from("hello"));

        let xs = "ศไทย中华Việt Nam".as_bytes().to_vec();
        assert_eq!(
            String::from_utf8(xs).unwrap(),
            String::from("ศไทย中华Việt Nam")
        );

        let xs = b"hello\xFF".to_vec();
        let err = String::from_utf8(xs).unwrap_err();
        assert_eq!(err.as_bytes(), b"hello\xff");
        let err_clone = err.clone();
        assert_eq!(err, err_clone);
        assert_eq!(err.into_bytes(), b"hello\xff".to_vec());
        assert_eq!(err_clone.utf8_error().valid_up_to(), 5);
    }

    #[test]
    fn from_utf8_lossy() {
        let xs = b"hello";
        let ys: Cow<'_, str> = "hello".into_cow();
        assert_eq!(String::from_utf8_lossy(xs), ys);

        let xs = "ศไทย中华Việt Nam".as_bytes();
        let ys: Cow<'_, str> = "ศไทย中华Việt Nam".into_cow();
        assert_eq!(String::from_utf8_lossy(xs), ys);

        let xs = b"Hello\xC2 There\xFF Goodbye";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("Hello\u{FFFD} There\u{FFFD} Goodbye").into_cow()
        );

        let xs = b"Hello\xC0\x80 There\xE6\x83 Goodbye";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("Hello\u{FFFD}\u{FFFD} There\u{FFFD} Goodbye").into_cow()
        );

        let xs = b"\xF5foo\xF5\x80bar";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("\u{FFFD}foo\u{FFFD}\u{FFFD}bar").into_cow()
        );

        let xs = b"\xF1foo\xF1\x80bar\xF1\x80\x80baz";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("\u{FFFD}foo\u{FFFD}bar\u{FFFD}baz").into_cow()
        );

        let xs = b"\xF4foo\xF4\x80bar\xF4\xBFbaz";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("\u{FFFD}foo\u{FFFD}bar\u{FFFD}\u{FFFD}baz").into_cow()
        );

        let xs = b"\xF0\x80\x80\x80foo\xF0\x90\x80\x80bar";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}foo\u{10000}bar").into_cow()
        );

        // surrogates
        let xs = b"\xED\xA0\x80foo\xED\xBF\xBFbar";
        assert_eq!(
            String::from_utf8_lossy(xs),
            String::from("\u{FFFD}\u{FFFD}\u{FFFD}foo\u{FFFD}\u{FFFD}\u{FFFD}bar").into_cow()
        );
    }

    #[test]
    fn from_utf16() {
        let pairs = [
            (
                String::from("𐍅𐌿𐌻𐍆𐌹𐌻𐌰\n"),
                vec![
                    0xd800, 0xdf45, 0xd800, 0xdf3f, 0xd800, 0xdf3b, 0xd800, 0xdf46, 0xd800, 0xdf39,
                    0xd800, 0xdf3b, 0xd800, 0xdf30, 0x000a,
                ],
            ),
            (
                String::from("𐐒𐑉𐐮𐑀𐐲𐑋 𐐏𐐲𐑍\n"),
                vec![
                    0xd801, 0xdc12, 0xd801, 0xdc49, 0xd801, 0xdc2e, 0xd801, 0xdc40, 0xd801, 0xdc32,
                    0xd801, 0xdc4b, 0x0020, 0xd801, 0xdc0f, 0xd801, 0xdc32, 0xd801, 0xdc4d, 0x000a,
                ],
            ),
            (
                String::from("𐌀𐌖𐌋𐌄𐌑𐌉·𐌌𐌄𐌕𐌄𐌋𐌉𐌑\n"),
                vec![
                    0xd800, 0xdf00, 0xd800, 0xdf16, 0xd800, 0xdf0b, 0xd800, 0xdf04, 0xd800, 0xdf11,
                    0xd800, 0xdf09, 0x00b7, 0xd800, 0xdf0c, 0xd800, 0xdf04, 0xd800, 0xdf15, 0xd800,
                    0xdf04, 0xd800, 0xdf0b, 0xd800, 0xdf09, 0xd800, 0xdf11, 0x000a,
                ],
            ),
            (
                String::from("𐒋𐒘𐒈𐒑𐒛𐒒 𐒕𐒓 𐒈𐒚𐒍 𐒏𐒜𐒒𐒖𐒆 𐒕𐒆\n"),
                vec![
                    0xd801, 0xdc8b, 0xd801, 0xdc98, 0xd801, 0xdc88, 0xd801, 0xdc91, 0xd801, 0xdc9b,
                    0xd801, 0xdc92, 0x0020, 0xd801, 0xdc95, 0xd801, 0xdc93, 0x0020, 0xd801, 0xdc88,
                    0xd801, 0xdc9a, 0xd801, 0xdc8d, 0x0020, 0xd801, 0xdc8f, 0xd801, 0xdc9c, 0xd801,
                    0xdc92, 0xd801, 0xdc96, 0xd801, 0xdc86, 0x0020, 0xd801, 0xdc95, 0xd801, 0xdc86,
                    0x000a,
                ],
            ),
            // Issue #12318, even-numbered non-BMP planes
            (String::from("\u{20000}"), vec![0xD840, 0xDC00]),
        ];

        for p in &pairs {
            let (s, u) = (*p).clone();
            let s_as_utf16 = s.encode_utf16().collect::<Vec<u16>>();
            let u_as_string = String::from_utf16(&u).unwrap();

            assert!(core::char::decode_utf16(u.iter().cloned()).all(|r| r.is_ok()));
            assert_eq!(s_as_utf16, u);

            assert_eq!(u_as_string, s);
            assert_eq!(String::from_utf16_lossy(&u), s);

            assert_eq!(String::from_utf16(&s_as_utf16).unwrap(), s);
            assert_eq!(u_as_string.encode_utf16().collect::<Vec<u16>>(), u);
        }
    }

    #[test]
    fn utf16_invalid() {
        // completely positive cases tested above.
        // lead + eof
        assert!(String::from_utf16(&[0xD800]).is_err());
        // lead + lead
        assert!(String::from_utf16(&[0xD800, 0xD800]).is_err());

        // isolated trail
        assert!(String::from_utf16(&[0x0061, 0xDC00]).is_err());

        // general
        assert!(String::from_utf16(&[0xD800, 0xd801, 0xdc8b, 0xD800]).is_err());
    }

    #[test]
    fn from_utf16_lossy() {
        // completely positive cases tested above.
        // lead + eof
        assert_eq!(
            String::from_utf16_lossy(&[0xD800]),
            String::from("\u{FFFD}")
        );
        // lead + lead
        assert_eq!(
            String::from_utf16_lossy(&[0xD800, 0xD800]),
            String::from("\u{FFFD}\u{FFFD}")
        );

        // isolated trail
        assert_eq!(
            String::from_utf16_lossy(&[0x0061, 0xDC00]),
            String::from("a\u{FFFD}")
        );

        // general
        assert_eq!(
            String::from_utf16_lossy(&[0xD800, 0xd801, 0xdc8b, 0xD800]),
            String::from("\u{FFFD}𐒋\u{FFFD}")
        );
    }
    */

    #[test]
    fn push_str() {
        let mut s = String::new();
        s.push_str("");
        assert_eq!(s, "");
        s.push_str("abc");
        assert_eq!(s, "abc");
        s.push_str("ประเทศไทย中华Việt Nam");
        assert_eq!(s, "abcประเทศไทย中华Việt Nam");
    }

    #[test]
    fn add_assign() {
        let mut s = String::new();
        s += "";
        assert_eq!(s, "");
        s += "abc";
        assert_eq!(s, "abc");
        s += "ประเทศไทย中华Việt Nam";
        assert_eq!(s, "abcประเทศไทย中华Việt Nam");
    }

    #[test]
    fn push() {
        let mut data = String::from("ประเทศไทย中");
        data.push('华');
        data.push('b'); // 1 byte
        data.push('¢'); // 2 byte
        data.push('€'); // 3 byte
        data.push('𤭢'); // 4 byte
        assert_eq!(data, "ประเทศไทย中华b¢€𤭢");
    }

    /*
    #[test]
    fn pop() {
        let mut data = String::from("ประเทศไทย中华b¢€𤭢");
        assert_eq!(data.pop().unwrap(), '𤭢'); // 4 bytes
        assert_eq!(data.pop().unwrap(), '€'); // 3 bytes
        assert_eq!(data.pop().unwrap(), '¢'); // 2 bytes
        assert_eq!(data.pop().unwrap(), 'b'); // 1 bytes
        assert_eq!(data.pop().unwrap(), '华');
        assert_eq!(data, "ประเทศไทย中");
    }
    */

    #[test]
    fn split_off_empty() {
        let orig = "Hello, world!";
        let mut split = String::from(orig);
        let empty: String = split.split_off(orig.len());
        assert!(empty.is_empty());
    }

    /*
    #[test]
    #[should_panic]
    fn split_off_past_end() {
        let orig = "Hello, world!";
        let mut split = String::from(orig);
        let _ = split.split_off(orig.len() + 1);
    }

    #[test]
    #[should_panic]
    fn split_off_mid_char() {
        let mut shan = String::from("山");
        let _broken_mountain = shan.split_off(1);
    }
    */

    #[test]
    fn split_off_ascii() {
        let mut ab = String::from("ABCD");
        // let orig_capacity = ab.capacity();
        let cd = ab.split_off(2);
        assert_eq!(ab, "AB");
        assert_eq!(cd, "CD");
        // assert_eq!(ab.capacity(), orig_capacity);
    }

    #[test]
    fn split_off_unicode() {
        let mut nihon = String::from("日本語");
        // let orig_capacity = nihon.capacity();
        let go = nihon.split_off("日本".len());
        assert_eq!(nihon, "日本");
        assert_eq!(go, "語");
        // assert_eq!(nihon.capacity(), orig_capacity);
    }

    #[test]
    fn str_truncate() {
        let mut s = String::from("12345");
        s.truncate(5);
        assert_eq!(s, "12345");
        s.truncate(3);
        assert_eq!(s, "123");
        s.truncate(0);
        assert_eq!(s, "");

        let mut s = String::from("12345");
        let p = s.as_c_ptr();
        s.truncate(3);
        s.push_str("6");
        let p_ = s.as_c_ptr();
        assert_eq!(p_, p);
    }

    #[test]
    fn str_truncate_invalid_len() {
        let mut s = String::from("12345");
        s.truncate(6);
        assert_eq!(s, "12345");
    }

    /*
    #[test]
    #[should_panic]
    fn str_truncate_split_codepoint() {
        let mut s = String::from("\u{FC}"); // ü
        s.truncate(1);
    }
    */

    #[test]
    fn str_clear() {
        let mut s = String::from("12345");
        s.clear();
        assert_eq!(s.len(), 0);
        assert_eq!(s, "");
    }

    #[test]
    fn str_add() {
        let a = String::from("12345");
        let b = a + "2";
        let b = b + "2";
        assert_eq!(b.len(), 7);
        assert_eq!(b, "1234522");
    }

    /*
    #[test]
    fn remove() {
        let mut s = String::from("ศไทย中华Việt Nam; foobar");
        assert_eq!(s.remove(0), 'ศ');
        assert_eq!(s.len(), 33);
        assert_eq!(s, "ไทย中华Việt Nam; foobar");
        assert_eq!(s.remove(17), 'ệ');
        assert_eq!(s, "ไทย中华Vit Nam; foobar");
    }

    #[test]
    #[should_panic]
    fn remove_bad() {
        "ศ".to_string().remove(1);
    }

    #[test]
    fn remove_matches() {
        let mut s = String::from("abc");

        s.remove_matches('b');
        assert_eq!(s, "ac");
        s.remove_matches('b');
        assert_eq!(s, "ac");

        let mut s = String::from("abcb");

        s.remove_matches('b');
        assert_eq!(s, "ac");

        let mut s = String::from("ศไทย中华Việt Nam; foobarศ");
        s.remove_matches('ศ');
        assert_eq!(s, "ไทย中华Việt Nam; foobar");

        let mut s = String::from("");
        s.remove_matches("");
        assert_eq!(s, "");

        let mut s = String::from("aaaaa");
        s.remove_matches('a');
        assert_eq!(s, "");
    }

    #[test]
    fn retain() {
        let mut s = String::from("α_β_γ");

        s.retain(|_| true);
        assert_eq!(s, "α_β_γ");

        s.retain(|c| c != '_');
        assert_eq!(s, "αβγ");

        s.retain(|c| c != 'β');
        assert_eq!(s, "αγ");

        s.retain(|c| c == 'α');
        assert_eq!(s, "α");

        s.retain(|_| false);
        assert_eq!(s, "");

        let mut s = String::from("0è0");
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            let mut count = 0;
            s.retain(|_| {
                count += 1;
                match count {
                    1 => false,
                    2 => true,
                    _ => panic!(),
                }
            });
        }));
        assert!(std::str::from_utf8(s.as_bytes()).is_ok());
    }
    */

    #[test]
    fn insert() {
        let mut s = String::from("foobar");
        s.insert(0, 'ệ');
        assert_eq!(s, "ệfoobar");
        s.insert(6, 'ย');
        assert_eq!(s, "ệfooยbar");
    }

    /*
    #[test]
    #[should_panic]
    fn insert_bad1() {
        String::new().insert(1, 't');
    }
    #[test]
    #[should_panic]
    fn insert_bad2() {
        String::from("ệ").insert(1, 't');
    }

    #[test]
    fn slicing() {
        let s = String::from("foobar");
        assert_eq!("foobar", &s[..]);
        assert_eq!("foo", &s[..3]);
        assert_eq!("bar", &s[3..]);
        assert_eq!("oob", &s[1..4]);
    }

    #[test]
    fn simple_types() {
        assert_eq!(1.to_string(), "1");
        assert_eq!((-1).to_string(), "-1");
        assert_eq!(200.to_string(), "200");
        assert_eq!(2.to_string(), "2");
        assert_eq!(true.to_string(), "true");
        assert_eq!(false.to_string(), "false");
        assert_eq!(("hi".to_string()).to_string(), "hi");
    }

    #[test]
    fn vectors() {
        let x: Vec<i32> = vec![];
        assert_eq!(format!("{x:?}"), "[]");
        assert_eq!(format!("{:?}", vec![1]), "[1]");
        assert_eq!(format!("{:?}", vec![1, 2, 3]), "[1, 2, 3]");
        assert!(format!("{:?}", vec![vec![], vec![1], vec![1, 1]]) == "[[], [1], [1, 1]]");
    }
    */

    #[test]
    fn from_iterator() {
        let s = String::from("ศไทย中华Việt Nam");
        let t = "ศไทย中华";
        let u = "Việt Nam";

        let a: String = s.chars_lossy().collect();
        assert_eq!(s, a);

        let mut b = String::from(t);
        b.extend(u.chars());
        assert_eq!(s, b);

        let c: String = [t, u].into_iter().collect();
        assert_eq!(s, c);

        #[cfg(feature = "alloc")]
        {
            extern crate alloc;

            let mut d = String::from(t);
            d.extend(alloc::vec![u]);
            assert_eq!(s, d);
        }
    }

    /*
    #[test]
    fn drain() {
        let mut s = String::from("αβγ");
        assert_eq!(s.drain(2..4).collect::<String>(), "β");
        assert_eq!(s, "αγ");

        let mut t = String::from("abcd");
        t.drain(..0);
        assert_eq!(t, "abcd");
        t.drain(..1);
        assert_eq!(t, "bcd");
        t.drain(3..);
        assert_eq!(t, "bcd");
        t.drain(..);
        assert_eq!(t, "");
    }

    #[test]
    #[should_panic]
    fn drain_start_overflow() {
        let mut s = String::from("abc");
        s.drain((Excluded(usize::MAX), Included(0)));
    }

    #[test]
    #[should_panic]
    fn drain_end_overflow() {
        let mut s = String::from("abc");
        s.drain((Included(0), Included(usize::MAX)));
    }

    #[test]
    fn replace_range() {
        let mut s = "Hello, world!".to_owned();
        s.replace_range(7..12, "世界");
        assert_eq!(s, "Hello, 世界!");
    }

    #[test]
    #[should_panic]
    fn replace_range_char_boundary() {
        let mut s = "Hello, 世界!".to_owned();
        s.replace_range(..8, "");
    }

    #[test]
    fn replace_range_inclusive_range() {
        let mut v = String::from("12345");
        v.replace_range(2..=3, "789");
        assert_eq!(v, "127895");
        v.replace_range(1..=2, "A");
        assert_eq!(v, "1A895");
    }

    #[test]
    #[should_panic]
    fn replace_range_out_of_bounds() {
        let mut s = String::from("12345");
        s.replace_range(5..6, "789");
    }

    #[test]
    #[should_panic]
    fn replace_range_inclusive_out_of_bounds() {
        let mut s = String::from("12345");
        s.replace_range(5..=5, "789");
    }

    #[test]
    #[should_panic]
    fn replace_range_start_overflow() {
        let mut s = String::from("123");
        s.replace_range((Excluded(usize::MAX), Included(0)), "");
    }

    #[test]
    #[should_panic]
    fn replace_range_end_overflow() {
        let mut s = String::from("456");
        s.replace_range((Included(0), Included(usize::MAX)), "");
    }

    #[test]
    fn replace_range_empty() {
        let mut s = String::from("12345");
        s.replace_range(1..2, "");
        assert_eq!(s, "1345");
    }

    #[test]
    fn replace_range_unbounded() {
        let mut s = String::from("12345");
        s.replace_range(.., "");
        assert_eq!(s, "");
    }

    #[test]
    fn replace_range_evil_start_bound() {
        struct EvilRange(Cell<bool>);

        impl RangeBounds<usize> for EvilRange {
            fn start_bound(&self) -> Bound<&usize> {
                Bound::Included(if self.0.get() {
                    &1
                } else {
                    self.0.set(true);
                    &0
                })
            }
            fn end_bound(&self) -> Bound<&usize> {
                Bound::Unbounded
            }
        }

        let mut s = String::from("🦀");
        s.replace_range(EvilRange(Cell::new(false)), "");
        assert_eq!(Ok(""), str::from_utf8(s.as_bytes()));
    }

    #[test]
    fn replace_range_evil_end_bound() {
        struct EvilRange(Cell<bool>);

        impl RangeBounds<usize> for EvilRange {
            fn start_bound(&self) -> Bound<&usize> {
                Bound::Included(&0)
            }
            fn end_bound(&self) -> Bound<&usize> {
                Bound::Excluded(if self.0.get() {
                    &3
                } else {
                    self.0.set(true);
                    &4
                })
            }
        }

        let mut s = String::from("🦀");
        s.replace_range(EvilRange(Cell::new(false)), "");
        assert_eq!(Ok(""), str::from_utf8(s.as_bytes()));
    }
    */

    #[test]
    fn extend_ref() {
        let mut a = String::from("foo");
        a.extend(&['b', 'a', 'r']);

        assert_eq!(&a, "foobar");
    }

    /*
    #[test]
    fn reserve_exact() {
        // This is all the same as test_reserve

        let mut s = String::new();
        assert_eq!(s.capacity(), 0);

        s.reserve_exact(2);
        assert!(s.capacity() >= 2);

        for _i in 0..16 {
            s.push('0');
        }

        assert!(s.capacity() >= 16);
        s.reserve_exact(16);
        assert!(s.capacity() >= 32);

        s.push('0');

        s.reserve_exact(16);
        assert!(s.capacity() >= 33)
    }

    #[test]
    fn try_reserve() {
        // These are the interesting cases:
        // * exactly isize::MAX should never trigger a CapacityOverflow (can be OOM)
        // * > isize::MAX should always fail
        //    * On 16/32-bit should CapacityOverflow
        //    * On 64-bit should OOM
        // * overflow may trigger when adding `len` to `cap` (in number of elements)
        // * overflow may trigger when multiplying `new_cap` by size_of::<T> (to get bytes)

        const MAX_CAP: usize = isize::MAX as usize;
        const MAX_USIZE: usize = usize::MAX;

        {
            // Note: basic stuff is checked by test_reserve
            let mut empty_string: String = String::new();

            // Check isize::MAX doesn't count as an overflow
            if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.kind()) {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }
            // Play it again, frank! (just to be sure)
            if let Err(CapacityOverflow) = empty_string.try_reserve(MAX_CAP).map_err(|e| e.kind()) {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }

            // Check isize::MAX + 1 does count as overflow
            assert_matches!(
                empty_string.try_reserve(MAX_CAP + 1).map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "isize::MAX + 1 should trigger an overflow!"
            );

            // Check usize::MAX does count as overflow
            assert_matches!(
                empty_string.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "usize::MAX should trigger an overflow!"
            );
        }

        {
            // Same basic idea, but with non-zero len
            let mut ten_bytes: String = String::from("0123456789");

            if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }
            if let Err(CapacityOverflow) = ten_bytes.try_reserve(MAX_CAP - 10).map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }

            assert_matches!(
                ten_bytes.try_reserve(MAX_CAP - 9).map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "isize::MAX + 1 should trigger an overflow!"
            );

            // Should always overflow in the add-to-len
            assert_matches!(
                ten_bytes.try_reserve(MAX_USIZE).map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "usize::MAX should trigger an overflow!"
            );
        }
    }

    #[test]
    fn try_reserve_exact() {
        // This is exactly the same as test_try_reserve with the method changed.
        // See that test for comments.

        const MAX_CAP: usize = isize::MAX as usize;
        const MAX_USIZE: usize = usize::MAX;

        {
            let mut empty_string: String = String::new();

            if let Err(CapacityOverflow) = empty_string
                .try_reserve_exact(MAX_CAP)
                .map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }
            if let Err(CapacityOverflow) = empty_string
                .try_reserve_exact(MAX_CAP)
                .map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }

            assert_matches!(
                empty_string
                    .try_reserve_exact(MAX_CAP + 1)
                    .map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "isize::MAX + 1 should trigger an overflow!"
            );

            assert_matches!(
                empty_string
                    .try_reserve_exact(MAX_USIZE)
                    .map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "usize::MAX should trigger an overflow!"
            );
        }

        {
            let mut ten_bytes: String = String::from("0123456789");

            if let Err(CapacityOverflow) = ten_bytes
                .try_reserve_exact(MAX_CAP - 10)
                .map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }
            if let Err(CapacityOverflow) = ten_bytes
                .try_reserve_exact(MAX_CAP - 10)
                .map_err(|e| e.kind())
            {
                panic!("isize::MAX shouldn't trigger an overflow!");
            }

            assert_matches!(
                ten_bytes
                    .try_reserve_exact(MAX_CAP - 9)
                    .map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "isize::MAX + 1 should trigger an overflow!"
            );

            assert_matches!(
                ten_bytes.try_reserve_exact(MAX_USIZE).map_err(|e| e.kind()),
                Err(CapacityOverflow),
                "usize::MAX should trigger an overflow!"
            );
        }
    }

    #[test]
    fn from_char() {
        assert_eq!(String::from('a'), 'a'.to_string());
        let s: String = 'x'.into();
        assert_eq!(s, 'x'.to_string());
    }

    #[test]
    fn str_concat() {
        let a: String = String::from("hello");
        let b: String = String::from("world");
        let s: String = format!("{a}{b}");
        assert_eq!(s.as_bytes()[9], 'd' as u8);
    }
    */
}

#[flipperzero_test::tests]
mod str_tests {
    use core::cmp::Ordering::{Equal, Greater, Less};

    use flipperzero::furi::string::FuriString as String;

    #[test]
    fn le() {
        assert!(String::from("") <= String::from(""));
        assert!(String::from("") <= String::from("foo"));
        assert!(String::from("foo") <= String::from("foo"));
        assert_ne!(String::from("foo"), String::from("bar"));
    }

    #[test]
    fn find() {
        assert_eq!(String::from("hello").find('l'), Some(2));
        // assert_eq!(String::from("hello").find(|c: char| c == 'o'), Some(4));
        assert!(String::from("hello").find('x').is_none());
        // assert!(String::from("hello").find(|c: char| c == 'x').is_none());
        assert_eq!(String::from("ประเทศไทย中华Việt Nam").find('华'), Some(30));
        // assert_eq!(
        //     String::from("ประเทศไทย中华Việt Nam").find(|c: char| c == '华'),
        //     Some(30)
        // );
    }

    #[test]
    fn rfind() {
        assert_eq!(String::from("hello").rfind('l'), Some(3));
        // assert_eq!(String::from("hello").rfind(|c: char| c == 'o'), Some(4));
        assert!(String::from("hello").rfind('x').is_none());
        // assert!(String::from("hello").rfind(|c: char| c == 'x').is_none());
        assert_eq!(String::from("ประเทศไทย中华Việt Nam").rfind('华'), Some(30));
        // assert_eq!(
        //     String::from("ประเทศไทย中华Việt Nam").rfind(|c: char| c == '华'),
        //     Some(30)
        // );
    }

    #[test]
    fn collect() {
        let empty = String::from("");
        let s: String = empty.chars_lossy().collect();
        assert_eq!(empty, s);
        let data = String::from("ประเทศไทย中");
        let s: String = data.chars_lossy().collect();
        assert_eq!(data, s);
    }

    /*
    #[test]
    fn into_bytes() {
        let data = String::from("asdf");
        let buf = data.into_bytes();
        assert_eq!(buf, b"asdf");
    }
    */

    #[test]
    fn find_str() {
        // byte positions
        assert_eq!(String::from("").find(&String::from("")), Some(0));
        assert!(
            String::from("banana")
                .find(&String::from("apple pie"))
                .is_none()
        );

        let mut data = String::from("abcabc");
        let ab = String::from("ab");
        assert_eq!(data.find(&ab), Some(0));
        assert!(data.strip_prefix(&ab));
        assert_eq!(data.find(&ab), Some(3 - 2));
        data.truncate(2);
        assert!(data.find(&ab).is_none());

        let string = "ประเทศไทย中华Việt Nam";
        let mut data = String::from(string);
        data.push_str(string);
        assert!(data.find(&String::from("ไท华")).is_none());
        assert_eq!(data.find(&String::from("")), Some(0));
        // assert_eq!(data[6..43].find(""), Some(6 - 6));

        assert_eq!(data.find(&String::from("ประ")), Some(0));
        assert_eq!(data.find(&String::from("ทศไ")), Some(12));
        assert_eq!(data.find(&String::from("ย中")), Some(24));
        assert_eq!(data.find(&String::from("iệt")), Some(34));
        assert_eq!(data.find(&String::from("Nam")), Some(40));

        let data = data.split_off(43);
        assert_eq!(data.find(&String::from("ประ")), Some(43 - 43));
        assert_eq!(data.find(&String::from("ทศไ")), Some(55 - 43));
        assert_eq!(data.find(&String::from("ย中")), Some(67 - 43));
        assert_eq!(data.find(&String::from("iệt")), Some(77 - 43));
        assert_eq!(data.find(&String::from("Nam")), Some(83 - 43));

        // // find every substring -- assert that it finds it, or an earlier occurrence.
        // let string = "Việt Namacbaabcaabaaba";
        // for (i, ci) in string.char_indices() {
        //     let ip = i + ci.len_utf8();
        //     for j in string[ip..]
        //         .char_indices()
        //         .map(|(i, _)| i)
        //         .chain(Some(string.len() - ip))
        //     {
        //         let pat = &string[i..ip + j];
        //         assert!(match string.find(pat) {
        //             None => false,
        //             Some(x) => x <= i,
        //         });
        //         assert!(match string.rfind(pat) {
        //             None => false,
        //             Some(x) => x >= i,
        //         });
        //     }
        // }
    }

    /*
    fn s(x: &str) -> String {
        x.to_string()
    }

    macro_rules! test_concat {
        ($expected: expr, $string: expr) => {{
            let s: String = $string.concat();
            assert_eq!($expected, s);
        }};
    }

    #[test]
    fn concat_for_different_types() {
        test_concat!("ab", vec![s("a"), s("b")]);
        test_concat!("ab", vec!["a", "b"]);
    }

    #[test]
    fn concat_for_different_lengths() {
        let empty: &[&str] = &[];
        test_concat!("", empty);
        test_concat!("a", ["a"]);
        test_concat!("ab", ["a", "b"]);
        test_concat!("abc", ["", "a", "bc"]);
    }

    macro_rules! test_join {
        ($expected: expr, $string: expr, $delim: expr) => {{
            let s = $string.join($delim);
            assert_eq!($expected, s);
        }};
    }

    #[test]
    fn join_for_different_types() {
        test_join!("a-b", ["a", "b"], "-");
        let hyphen = "-".to_string();
        test_join!("a-b", [s("a"), s("b")], &*hyphen);
        test_join!("a-b", vec!["a", "b"], &*hyphen);
        test_join!("a-b", &*vec!["a", "b"], "-");
        test_join!("a-b", vec![s("a"), s("b")], "-");
    }

    #[test]
    fn join_for_different_lengths() {
        let empty: &[&str] = &[];
        test_join!("", empty, "-");
        test_join!("a", ["a"], "-");
        test_join!("a-b", ["a", "b"], "-");
        test_join!("-a-bc", ["", "a", "bc"], "-");
    }

    // join has fast paths for small separators up to 4 bytes
    // this tests the slow paths.
    #[test]
    fn join_for_different_lengths_with_long_separator() {
        assert_eq!("～～～～～".len(), 15);

        let empty: &[&str] = &[];
        test_join!("", empty, "～～～～～");
        test_join!("a", ["a"], "～～～～～");
        test_join!("a～～～～～b", ["a", "b"], "～～～～～");
        test_join!("～～～～～a～～～～～bc", ["", "a", "bc"], "～～～～～");
    }

    #[test]
    fn join_issue_80335() {
        use core::{borrow::Borrow, cell::Cell};

        struct WeirdBorrow {
            state: Cell<bool>,
        }

        impl Default for WeirdBorrow {
            fn default() -> Self {
                WeirdBorrow {
                    state: Cell::new(false),
                }
            }
        }

        impl Borrow<str> for WeirdBorrow {
            fn borrow(&self) -> &str {
                let state = self.state.get();
                if state {
                    "0"
                } else {
                    self.state.set(true);
                    "123456"
                }
            }
        }

        let arr: [WeirdBorrow; 3] = Default::default();
        test_join!("0-0-0", arr, "-");
    }

    #[test]
    fn unsafe_slice() {
        assert_eq!("ab", unsafe { "abc".get_unchecked(0..2) });
        assert_eq!("bc", unsafe { "abc".get_unchecked(1..3) });
        assert_eq!("", unsafe { "abc".get_unchecked(1..1) });
        fn a_million_letter_a() -> String {
            let mut i = 0;
            let mut rs = String::new();
            while i < 100000 {
                rs.push_str("aaaaaaaaaa");
                i += 1;
            }
            rs
        }
        fn half_a_million_letter_a() -> String {
            let mut i = 0;
            let mut rs = String::new();
            while i < 100000 {
                rs.push_str("aaaaa");
                i += 1;
            }
            rs
        }
        let letters = a_million_letter_a();
        assert_eq!(half_a_million_letter_a(), unsafe {
            letters.get_unchecked(0..500000)
        });
    }
    */

    #[test]
    fn starts_with() {
        assert!(String::from("").starts_with(&String::from("")));
        assert!(String::from("abc").starts_with(&String::from("")));
        assert!(String::from("abc").starts_with(&String::from("a")));
        assert!(!String::from("a").starts_with(&String::from("abc")));
        assert!(!String::from("").starts_with(&String::from("abc")));
        assert!(!String::from("ödd").starts_with(&String::from("-")));
        assert!(String::from("ödd").starts_with(&String::from("öd")));
    }

    #[test]
    fn ends_with() {
        assert!(String::from("").ends_with(&String::from("")));
        assert!(String::from("abc").ends_with(&String::from("")));
        assert!(String::from("abc").ends_with(&String::from("c")));
        assert!(!String::from("a").ends_with(&String::from("abc")));
        assert!(!String::from("").ends_with(&String::from("abc")));
        assert!(!String::from("ddö").ends_with(&String::from("-")));
        assert!(String::from("ddö").ends_with(&String::from("dö")));
    }

    #[test]
    fn is_empty() {
        assert!(String::from("").is_empty());
        assert!(!String::from("a").is_empty());
    }

    /*
    #[test]
    fn replacen() {
        assert_eq!(String::from("").replacen('a', "b", 5), "");
        assert_eq!(String::from("acaaa").replacen("a", "b", 3), "bcbba");
        assert_eq!(String::from("aaaa").replacen("a", "b", 0), "aaaa");

        let test = "test";
        assert_eq!(
            String::from(" test test ").replacen(test, "toast", 3),
            " toast toast "
        );
        assert_eq!(
            String::from(" test test ").replacen(test, "toast", 0),
            " test test "
        );
        assert_eq!(String::from(" test test ").replacen(test, "", 5), "   ");

        assert_eq!(
            String::from("qwer123zxc789").replacen(char::is_numeric, "", 3),
            "qwerzxc789"
        );
    }

    #[test]
    fn replace() {
        let a = "a";
        assert_eq!("".replace(a, "b"), "");
        assert_eq!("a".replace(a, "b"), "b");
        assert_eq!("ab".replace(a, "b"), "bb");
        let test = "test";
        assert_eq!(" test test ".replace(test, "toast"), " toast toast ");
        assert_eq!(" test test ".replace(test, ""), "   ");
    }

    #[test]
    fn replace_2a() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let a = "ประเ";
        let a2 = "دولة الكويتทศไทย中华";
        assert_eq!(data.replace(a, repl), a2);
    }

    #[test]
    fn replace_2b() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let b = "ะเ";
        let b2 = "ปรدولة الكويتทศไทย中华";
        assert_eq!(data.replace(b, repl), b2);
    }

    #[test]
    fn replace_2c() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let c = "中华";
        let c2 = "ประเทศไทยدولة الكويت";
        assert_eq!(data.replace(c, repl), c2);
    }

    #[test]
    fn replace_2d() {
        let data = "ประเทศไทย中华";
        let repl = "دولة الكويت";

        let d = "ไท华";
        assert_eq!(data.replace(d, repl), data);
    }

    #[test]
    fn replace_pattern() {
        let data = String::from("abcdαβγδabcdαβγδ");
        assert_eq!(data.replace("dαβ", "😺😺😺"), "abc😺😺😺γδabc😺😺😺γδ");
        assert_eq!(data.replace('γ', "😺😺😺"), "abcdαβ😺😺😺δabcdαβ😺😺😺δ");
        assert_eq!(
            data.replace(&['a', 'γ'] as &[_], "😺😺😺"),
            "😺😺😺bcdαβ😺😺😺δ😺😺😺bcdαβ😺😺😺δ"
        );
        assert_eq!(
            data.replace(|c| c == 'γ', "😺😺😺"),
            "abcdαβ😺😺😺δabcdαβ😺😺😺δ"
        );
    }

    // The current implementation of SliceIndex fails to handle methods
    // orthogonally from range types; therefore, it is worth testing
    // all of the indexing operations on each input.
    mod slice_index {
        // Test a slicing operation **that should succeed,**
        // testing it on all of the indexing methods.
        //
        // This is not suitable for testing failure on invalid inputs.
        macro_rules! assert_range_eq {
            ($s:expr, $range:expr, $expected:expr) => {
                let mut s: String = $s.to_owned();
                let mut expected: String = $expected.to_owned();
                {
                    let s: &str = &s;
                    let expected: &str = &expected;

                    assert_eq!(&s[$range], expected, "(in assertion for: index)");
                    assert_eq!(s.get($range), Some(expected), "(in assertion for: get)");
                    unsafe {
                        assert_eq!(
                            s.get_unchecked($range),
                            expected,
                            "(in assertion for: get_unchecked)",
                        );
                    }
                }
                {
                    let s: &mut str = &mut s;
                    let expected: &mut str = &mut expected;

                    assert_eq!(&mut s[$range], expected, "(in assertion for: index_mut)",);
                    assert_eq!(
                        s.get_mut($range),
                        Some(&mut expected[..]),
                        "(in assertion for: get_mut)",
                    );
                    unsafe {
                        assert_eq!(
                            s.get_unchecked_mut($range),
                            expected,
                            "(in assertion for: get_unchecked_mut)",
                        );
                    }
                }
            };
        }

        // Make sure the macro can actually detect bugs,
        // because if it can't, then what are we even doing here?
        //
        // (Be aware this only demonstrates the ability to detect bugs
        //  in the FIRST method that panics, as the macro is not designed
        //  to be used in `should_panic`)
        #[test]
        #[should_panic(expected = "out of bounds")]
        fn assert_range_eq_can_fail_by_panic() {
            assert_range_eq!("abc", 0..5, "abc");
        }

        // (Be aware this only demonstrates the ability to detect bugs
        //  in the FIRST method it calls, as the macro is not designed
        //  to be used in `should_panic`)
        #[test]
        #[should_panic(expected = "==")]
        fn assert_range_eq_can_fail_by_inequality() {
            assert_range_eq!("abc", 0..2, "abc");
        }

        // Generates test cases for bad index operations.
        //
        // This generates `should_panic` test cases for Index/IndexMut
        // and `None` test cases for get/get_mut.
        macro_rules! panic_cases {
            ($(
                in mod $case_name:ident {
                    data: $data:expr;

                    // optional:
                    //
                    // a similar input for which DATA[input] succeeds, and the corresponding
                    // output str. This helps validate "critical points" where an input range
                    // straddles the boundary between valid and invalid.
                    // (such as the input `len..len`, which is just barely valid)
                    $(
                        good: data[$good:expr] == $output:expr;
                    )*

                    bad: data[$bad:expr];
                    message: $expect_msg:expr; // must be a literal
                }
            )*) => {$(
                mod $case_name {
                    #[test]
                    fn pass() {
                        let mut v: String = $data.into();

                        $( assert_range_eq!(v, $good, $output); )*

                        {
                            let v: &str = &v;
                            assert_eq!(v.get($bad), None, "(in None assertion for get)");
                        }

                        {
                            let v: &mut str = &mut v;
                            assert_eq!(v.get_mut($bad), None, "(in None assertion for get_mut)");
                        }
                    }

                    #[test]
                    #[should_panic(expected = $expect_msg)]
                    fn index_fail() {
                        let v: String = $data.into();
                        let v: &str = &v;
                        let _v = &v[$bad];
                    }

                    #[test]
                    #[should_panic(expected = $expect_msg)]
                    fn index_mut_fail() {
                        let mut v: String = $data.into();
                        let v: &mut str = &mut v;
                        let _v = &mut v[$bad];
                    }
                }
            )*};
        }

        #[test]
        fn simple_ascii() {
            assert_range_eq!("abc", .., "abc");

            assert_range_eq!("abc", 0..2, "ab");
            assert_range_eq!("abc", 0..=1, "ab");
            assert_range_eq!("abc", ..2, "ab");
            assert_range_eq!("abc", ..=1, "ab");

            assert_range_eq!("abc", 1..3, "bc");
            assert_range_eq!("abc", 1..=2, "bc");
            assert_range_eq!("abc", 1..1, "");
            assert_range_eq!("abc", 1..=0, "");
        }

        #[test]
        fn simple_unicode() {
            // 日本
            assert_range_eq!("\u{65e5}\u{672c}", .., "\u{65e5}\u{672c}");

            assert_range_eq!("\u{65e5}\u{672c}", 0..3, "\u{65e5}");
            assert_range_eq!("\u{65e5}\u{672c}", 0..=2, "\u{65e5}");
            assert_range_eq!("\u{65e5}\u{672c}", ..3, "\u{65e5}");
            assert_range_eq!("\u{65e5}\u{672c}", ..=2, "\u{65e5}");

            assert_range_eq!("\u{65e5}\u{672c}", 3..6, "\u{672c}");
            assert_range_eq!("\u{65e5}\u{672c}", 3..=5, "\u{672c}");
            assert_range_eq!("\u{65e5}\u{672c}", 3.., "\u{672c}");

            let data = "ประเทศไทย中华";
            assert_range_eq!(data, 0..3, "ป");
            assert_range_eq!(data, 3..6, "ร");
            assert_range_eq!(data, 3..3, "");
            assert_range_eq!(data, 30..33, "华");

            /*0: 中
             3: 华
             6: V
             7: i
             8: ệ
            11: t
            12:
            13: N
            14: a
            15: m */
            let ss = "中华Việt Nam";
            assert_range_eq!(ss, 3..6, "华");
            assert_range_eq!(ss, 6..16, "Việt Nam");
            assert_range_eq!(ss, 6..=15, "Việt Nam");
            assert_range_eq!(ss, 6.., "Việt Nam");

            assert_range_eq!(ss, 0..3, "中");
            assert_range_eq!(ss, 3..7, "华V");
            assert_range_eq!(ss, 3..=6, "华V");
            assert_range_eq!(ss, 3..3, "");
            assert_range_eq!(ss, 3..=2, "");
        }

        #[test]
        #[cfg_attr(target_os = "emscripten", ignore)] // hits an OOM
        #[cfg_attr(miri, ignore)] // Miri is too slow
        fn simple_big() {
            fn a_million_letter_x() -> String {
                let mut i = 0;
                let mut rs = String::new();
                while i < 100000 {
                    rs.push_str("华华华华华华华华华华");
                    i += 1;
                }
                rs
            }
            fn half_a_million_letter_x() -> String {
                let mut i = 0;
                let mut rs = String::new();
                while i < 100000 {
                    rs.push_str("华华华华华");
                    i += 1;
                }
                rs
            }
            let letters = a_million_letter_x();
            assert_range_eq!(letters, 0..3 * 500000, half_a_million_letter_x());
        }

        #[test]
        #[should_panic]
        fn slice_fail() {
            let _ = &"中华Việt Nam"[0..2];
        }

        panic_cases! {
            in mod rangefrom_len {
                data: "abcdef";
                good: data[6..] == "";
                bad: data[7..];
                message: "out of bounds";
            }

            in mod rangeto_len {
                data: "abcdef";
                good: data[..6] == "abcdef";
                bad: data[..7];
                message: "out of bounds";
            }

            in mod rangetoinclusive_len {
                data: "abcdef";
                good: data[..=5] == "abcdef";
                bad: data[..=6];
                message: "out of bounds";
            }

            in mod rangeinclusive_len {
                data: "abcdef";
                good: data[0..=5] == "abcdef";
                bad: data[0..=6];
                message: "out of bounds";
            }

            in mod range_len_len {
                data: "abcdef";
                good: data[6..6] == "";
                bad: data[7..7];
                message: "out of bounds";
            }

            in mod rangeinclusive_len_len {
                data: "abcdef";
                good: data[6..=5] == "";
                bad: data[7..=6];
                message: "out of bounds";
            }
        }

        panic_cases! {
            in mod rangeinclusive_exhausted {
                data: "abcdef";

                good: data[0..=5] == "abcdef";
                good: data[{
                    let mut iter = 0..=5;
                    iter.by_ref().count(); // exhaust it
                    iter
                }] == "";

                // 0..=6 is out of bounds before exhaustion, so it
                // stands to reason that it still would be after.
                bad: data[{
                    let mut iter = 0..=6;
                    iter.by_ref().count(); // exhaust it
                    iter
                }];
                message: "out of bounds";
            }
        }

        panic_cases! {
            in mod range_neg_width {
                data: "abcdef";
                good: data[4..4] == "";
                bad: data[4..3];
                message: "begin <= end (4 <= 3)";
            }

            in mod rangeinclusive_neg_width {
                data: "abcdef";
                good: data[4..=3] == "";
                bad: data[4..=2];
                message: "begin <= end (4 <= 3)";
            }
        }

        mod overflow {
            panic_cases! {
                in mod rangeinclusive {
                    data: "hello";
                    // note: using 0 specifically ensures that the result of overflowing is 0..0,
                    //       so that `get` doesn't simply return None for the wrong reason.
                    bad: data[0..=usize::MAX];
                    message: "maximum usize";
                }

                in mod rangetoinclusive {
                    data: "hello";
                    bad: data[..=usize::MAX];
                    message: "maximum usize";
                }
            }
        }

        mod boundary {
            const DATA: &str = "abcαβγ";

            const BAD_START: usize = 4;
            const GOOD_START: usize = 3;
            const BAD_END: usize = 6;
            const GOOD_END: usize = 7;
            const BAD_END_INCL: usize = BAD_END - 1;
            const GOOD_END_INCL: usize = GOOD_END - 1;

            // it is especially important to test all of the different range types here
            // because some of the logic may be duplicated as part of micro-optimizations
            // to dodge unicode boundary checks on half-ranges.
            panic_cases! {
                in mod range_1 {
                    data: super::DATA;
                    bad: data[super::BAD_START..super::GOOD_END];
                    message:
                        "byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of";
                }

                in mod range_2 {
                    data: super::DATA;
                    bad: data[super::GOOD_START..super::BAD_END];
                    message:
                        "byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of";
                }

                in mod rangefrom {
                    data: super::DATA;
                    bad: data[super::BAD_START..];
                    message:
                        "byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of";
                }

                in mod rangeto {
                    data: super::DATA;
                    bad: data[..super::BAD_END];
                    message:
                        "byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of";
                }

                in mod rangeinclusive_1 {
                    data: super::DATA;
                    bad: data[super::BAD_START..=super::GOOD_END_INCL];
                    message:
                        "byte index 4 is not a char boundary; it is inside 'α' (bytes 3..5) of";
                }

                in mod rangeinclusive_2 {
                    data: super::DATA;
                    bad: data[super::GOOD_START..=super::BAD_END_INCL];
                    message:
                        "byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of";
                }

                in mod rangetoinclusive {
                    data: super::DATA;
                    bad: data[..=super::BAD_END_INCL];
                    message:
                        "byte index 6 is not a char boundary; it is inside 'β' (bytes 5..7) of";
                }
            }
        }

        const LOREM_PARAGRAPH: &str = "\
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Suspendisse quis lorem \
        sit amet dolor ultricies condimentum. Praesent iaculis purus elit, ac malesuada \
        quam malesuada in. Duis sed orci eros. Suspendisse sit amet magna mollis, mollis \
        nunc luctus, imperdiet mi. Integer fringilla non sem ut lacinia. Fusce varius \
        tortor a risus porttitor hendrerit. Morbi mauris dui, ultricies nec tempus vel, \
        gravida nec quam.";

        // check the panic includes the prefix of the sliced string
        #[test]
        #[should_panic(
            expected = "byte index 1024 is out of bounds of `Lorem ipsum dolor sit amet"
        )]
        fn slice_fail_truncated_1() {
            let _ = &LOREM_PARAGRAPH[..1024];
        }
        // check the truncation in the panic message
        #[test]
        #[should_panic(expected = "luctus, im`[...]")]
        fn slice_fail_truncated_2() {
            let _ = &LOREM_PARAGRAPH[..1024];
        }
    }

    #[test]
    fn str_slice_rangetoinclusive_ok() {
        let s = String::from("abcαβγ");
        assert_eq!(&s[..=2], "abc");
        assert_eq!(&s[..=4], "abcα");
    }

    #[test]
    #[should_panic]
    fn str_slice_rangetoinclusive_notok() {
        let s = String::from("abcαβγ");
        let _ = &s[..=3];
    }

    #[test]
    fn str_slicemut_rangetoinclusive_ok() {
        let mut s = "abcαβγ".to_owned();
        let s: &mut str = &mut s;
        assert_eq!(&mut s[..=2], "abc");
        assert_eq!(&mut s[..=4], "abcα");
    }

    #[test]
    #[should_panic]
    fn str_slicemut_rangetoinclusive_notok() {
        let mut s = "abcαβγ".to_owned();
        let s: &mut str = &mut s;
        let _ = &mut s[..=3];
    }

    #[test]
    fn is_char_boundary() {
        let s = String::from("ศไทย中华Việt Nam β-release 🐱123");
        assert!(s.is_char_boundary(0));
        assert!(s.is_char_boundary(s.len()));
        assert!(!s.is_char_boundary(s.len() + 1));
        for (i, ch) in s.char_indices() {
            // ensure character locations are boundaries and continuation bytes are not
            assert!(s.is_char_boundary(i), "{} is a char boundary in {:?}", i, s);
            for j in 1..ch.len_utf8() {
                assert!(
                    !s.is_char_boundary(i + j),
                    "{} should not be a char boundary in {:?}",
                    i + j,
                    s
                );
            }
        }
    }
    */

    fn check_trim(s: &str, f: impl FnOnce(&mut String), r: &str) {
        let mut s = String::from(s);
        f(&mut s);
        assert_eq!(s, String::from(r));
    }

    #[test]
    fn trim_start_matches() {
        let v: &[char] = &[];
        check_trim(
            " *** foo *** ",
            |s| s.trim_start_matches(v),
            " *** foo *** ",
        );
        let chars: &[char] = &['*', ' '];
        check_trim(" *** foo *** ", |s| s.trim_start_matches(chars), "foo *** ");
        check_trim(" ***  *** ", |s| s.trim_start_matches(chars), "");
        check_trim("foo *** ", |s| s.trim_start_matches(chars), "foo *** ");

        check_trim("11foo1bar11", |s| s.trim_start_matches('1'), "foo1bar11");
        let chars: &[char] = &['1', '2'];
        check_trim("12foo1bar12", |s| s.trim_start_matches(chars), "foo1bar12");
        // check_trim(
        //     "123foo1bar123",
        //     |s| s.trim_start_matches(|c: char| c.is_numeric()),
        //     "foo1bar123",
        // );
    }

    #[test]
    fn trim_end_matches() {
        let v: &[char] = &[];
        check_trim(" *** foo *** ", |s| s.trim_end_matches(v), " *** foo *** ");
        let chars: &[char] = &['*', ' '];
        check_trim(" *** foo *** ", |s| s.trim_end_matches(chars), " *** foo");
        check_trim(" ***  *** ", |s| s.trim_end_matches(chars), "");
        check_trim(" *** foo", |s| s.trim_end_matches(chars), " *** foo");

        check_trim("11foo1bar11", |s| s.trim_end_matches('1'), "11foo1bar");
        let chars: &[char] = &['1', '2'];
        check_trim("12foo1bar12", |s| s.trim_end_matches(chars), "12foo1bar");
        // check_trim(
        //     "123foo1bar123",
        //     |s| s.trim_end_matches(|c: char| c.is_numeric()),
        //     "123foo1bar",
        // );
    }

    #[test]
    fn trim_matches() {
        let v: &[char] = &[];
        check_trim(" *** foo *** ", |s| s.trim_matches(v), " *** foo *** ");
        let chars: &[char] = &['*', ' '];
        check_trim(" *** foo *** ", |s| s.trim_matches(chars), "foo");
        check_trim(" ***  *** ", |s| s.trim_matches(chars), "");
        check_trim("foo", |s| s.trim_matches(chars), "foo");

        check_trim("11foo1bar11", |s| s.trim_matches('1'), "foo1bar");
        let chars: &[char] = &['1', '2'];
        check_trim("12foo1bar12", |s| s.trim_matches(chars), "foo1bar");
        // check_trim(
        //     "123foo1bar123",
        //     |s| s.trim_matches(|c: char| c.is_numeric()),
        //     "foo1bar",
        // );
    }

    #[test]
    fn trim_start() {
        check_trim("", |s| s.trim_start(), "");
        check_trim("a", |s| s.trim_start(), "a");
        check_trim("    ", |s| s.trim_start(), "");
        check_trim("     blah", |s| s.trim_start(), "blah");
        check_trim("   \u{3000}  wut", |s| s.trim_start(), "wut");
        check_trim("hey ", |s| s.trim_start(), "hey ");
    }

    #[test]
    fn trim_end() {
        check_trim("", |s| s.trim_end(), "");
        check_trim("a", |s| s.trim_end(), "a");
        check_trim("    ", |s| s.trim_end(), "");
        check_trim("blah     ", |s| s.trim_end(), "blah");
        check_trim("wut   \u{3000}  ", |s| s.trim_end(), "wut");
        check_trim(" hey", |s| s.trim_end(), " hey");
    }

    #[test]
    fn trim() {
        check_trim("", |s| s.trim(), "");
        check_trim("a", |s| s.trim(), "a");
        check_trim("    ", |s| s.trim(), "");
        check_trim("    blah     ", |s| s.trim(), "blah");
        check_trim("\nwut   \u{3000}  ", |s| s.trim(), "wut");
        check_trim(" hey dude ", |s| s.trim(), "hey dude");
    }

    #[test]
    fn to_bytes() {
        // no null
        let v = [
            224, 184, 168, 224, 185, 132, 224, 184, 151, 224, 184, 162, 228, 184, 173, 229, 141,
            142, 86, 105, 225, 187, 135, 116, 32, 78, 97, 109,
        ];
        let b: &[u8] = &[];
        assert_eq!(String::from("").to_bytes(), b);
        assert_eq!(String::from("abc").to_bytes(), b"abc");
        assert_eq!(String::from("ศไทย中华Việt Nam").to_bytes(), v);
    }

    /*
    #[test]
    fn as_ptr() {
        let s = String::from("hello");
        let buf = s.as_ptr();
        unsafe {
            assert_eq!(*buf.offset(0), b'h');
            assert_eq!(*buf.offset(1), b'e');
            assert_eq!(*buf.offset(2), b'l');
            assert_eq!(*buf.offset(3), b'l');
            assert_eq!(*buf.offset(4), b'o');
        }
    }

    #[test]
    fn vec_str_conversions() {
        let s1: String = String::from("All mimsy were the borogoves");

        let v: Vec<u8> = s1.as_bytes().to_vec();
        let s2: String = String::from(from_utf8(&v).unwrap());
        let mut i = 0;
        let n1 = s1.len();
        let n2 = v.len();
        assert_eq!(n1, n2);
        while i < n1 {
            let a: u8 = s1.as_bytes()[i];
            let b: u8 = s2.as_bytes()[i];
            assert_eq!(a, b);
            i += 1;
        }
    }
    */

    #[test]
    fn contains() {
        let empty = String::new();
        let abcde = String::from("abcde");
        assert!(abcde.contains(&String::from("bcd")));
        assert!(abcde.contains(&String::from("abcd")));
        assert!(abcde.contains(&String::from("bcde")));
        assert!(abcde.contains(&String::from("")));
        assert!(empty.contains(&String::from("")));
        assert!(!abcde.contains(&String::from("def")));
        assert!(!empty.contains(&String::from("a")));

        let data = String::from("ประเทศไทย中华Việt Nam");
        assert!(data.contains(&String::from("ประเ")));
        assert!(data.contains(&String::from("ะเ")));
        assert!(data.contains(&String::from("中华")));
        assert!(!data.contains(&String::from("ไท华")));
    }

    #[test]
    fn contains_char() {
        assert!(String::from("abc").contains('b'));
        assert!(String::from("a").contains('a'));
        assert!(!String::from("abc").contains('d'));
        assert!(!String::from("").contains('a'));
    }

    /*
    #[test]
    fn split_at() {
        let s = String::from("ศไทย中华Việt Nam");
        for (index, _) in s.char_indices() {
            let (a, b) = s.split_at(index);
            assert_eq!(&s[..a.len()], a);
            assert_eq!(&s[a.len()..], b);
        }
        let (a, b) = s.split_at(s.len());
        assert_eq!(a, s);
        assert_eq!(b, "");
    }

    #[test]
    fn split_at_mut() {
        let mut s = String::from("Hello World");
        {
            let (a, b) = s.split_at_mut(5);
            a.make_ascii_uppercase();
            b.make_ascii_lowercase();
        }
        assert_eq!(s, "HELLO world");
    }

    #[test]
    #[should_panic]
    fn split_at_boundscheck() {
        let s = "ศไทย中华Việt Nam";
        let _ = s.split_at(1);
    }

    #[test]
    fn escape_unicode() {
        assert_eq!("abc".escape_unicode().to_string(), "\\u{61}\\u{62}\\u{63}");
        assert_eq!("a c".escape_unicode().to_string(), "\\u{61}\\u{20}\\u{63}");
        assert_eq!("\r\n\t".escape_unicode().to_string(), "\\u{d}\\u{a}\\u{9}");
        assert_eq!(
            "'\"\\".escape_unicode().to_string(),
            "\\u{27}\\u{22}\\u{5c}"
        );
        assert_eq!(
            "\x00\x01\u{fe}\u{ff}".escape_unicode().to_string(),
            "\\u{0}\\u{1}\\u{fe}\\u{ff}"
        );
        assert_eq!(
            "\u{100}\u{ffff}".escape_unicode().to_string(),
            "\\u{100}\\u{ffff}"
        );
        assert_eq!(
            "\u{10000}\u{10ffff}".escape_unicode().to_string(),
            "\\u{10000}\\u{10ffff}"
        );
        assert_eq!(
            "ab\u{fb00}".escape_unicode().to_string(),
            "\\u{61}\\u{62}\\u{fb00}"
        );
        assert_eq!(
            "\u{1d4ea}\r".escape_unicode().to_string(),
            "\\u{1d4ea}\\u{d}"
        );
    }

    #[test]
    fn escape_debug() {
        // Note that there are subtleties with the number of backslashes
        // on the left- and right-hand sides. In particular, Unicode code points
        // are usually escaped with two backslashes on the right-hand side, as
        // they are escaped. However, when the character is unescaped (e.g., for
        // printable characters), only a single backslash appears (as the character
        // itself appears in the debug string).
        assert_eq!("abc".escape_debug().to_string(), "abc");
        assert_eq!("a c".escape_debug().to_string(), "a c");
        assert_eq!("éèê".escape_debug().to_string(), "éèê");
        assert_eq!("\r\n\t".escape_debug().to_string(), "\\r\\n\\t");
        assert_eq!("'\"\\".escape_debug().to_string(), "\\'\\\"\\\\");
        assert_eq!("\u{7f}\u{ff}".escape_debug().to_string(), "\\u{7f}\u{ff}");
        assert_eq!(
            "\u{100}\u{ffff}".escape_debug().to_string(),
            "\u{100}\\u{ffff}"
        );
        assert_eq!(
            "\u{10000}\u{10ffff}".escape_debug().to_string(),
            "\u{10000}\\u{10ffff}"
        );
        assert_eq!("ab\u{200b}".escape_debug().to_string(), "ab\\u{200b}");
        assert_eq!("\u{10d4ea}\r".escape_debug().to_string(), "\\u{10d4ea}\\r");
        assert_eq!(
            "\u{301}a\u{301}bé\u{e000}".escape_debug().to_string(),
            "\\u{301}a\u{301}bé\\u{e000}"
        );
    }

    #[test]
    fn escape_default() {
        assert_eq!("abc".escape_default().to_string(), "abc");
        assert_eq!("a c".escape_default().to_string(), "a c");
        assert_eq!("éèê".escape_default().to_string(), "\\u{e9}\\u{e8}\\u{ea}");
        assert_eq!("\r\n\t".escape_default().to_string(), "\\r\\n\\t");
        assert_eq!("'\"\\".escape_default().to_string(), "\\'\\\"\\\\");
        assert_eq!(
            "\u{7f}\u{ff}".escape_default().to_string(),
            "\\u{7f}\\u{ff}"
        );
        assert_eq!(
            "\u{100}\u{ffff}".escape_default().to_string(),
            "\\u{100}\\u{ffff}"
        );
        assert_eq!(
            "\u{10000}\u{10ffff}".escape_default().to_string(),
            "\\u{10000}\\u{10ffff}"
        );
        assert_eq!("ab\u{200b}".escape_default().to_string(), "ab\\u{200b}");
        assert_eq!(
            "\u{10d4ea}\r".escape_default().to_string(),
            "\\u{10d4ea}\\r"
        );
    }
    */

    #[test]
    fn total_ord() {
        assert_eq!(String::from("1234").cmp(&String::from("123")), Greater);
        assert_eq!(String::from("123").cmp(&String::from("1234")), Less);
        assert_eq!(String::from("1234").cmp(&String::from("1234")), Equal);
        assert_eq!(String::from("12345555").cmp(&String::from("123456")), Less);
        assert_eq!(String::from("22").cmp(&String::from("1234")), Greater);
    }

    #[test]
    fn iterator() {
        let s = String::from("ศไทย中华Việt Nam");
        let v = [
            'ศ', 'ไ', 'ท', 'ย', '中', '华', 'V', 'i', 'ệ', 't', ' ', 'N', 'a', 'm',
        ];

        let mut pos = 0;
        let it = s.chars_lossy();

        for c in it {
            assert_eq!(c, v[pos]);
            pos += 1;
        }
        assert_eq!(pos, v.len());
        assert_eq!(s.chars_lossy().count(), v.len());
    }

    /*
    #[test]
    fn rev_iterator() {
        let s = String::from("ศไทย中华Việt Nam");
        let v = [
            'm', 'a', 'N', ' ', 't', 'ệ', 'i', 'V', '华', '中', 'ย', 'ท', 'ไ', 'ศ',
        ];

        let mut pos = 0;
        let it = s.chars().rev();

        for c in it {
            assert_eq!(c, v[pos]);
            pos += 1;
        }
        assert_eq!(pos, v.len());
    }

    #[test]
    fn to_lowercase_rev_iterator() {
        let s = String::from("AÖßÜ💩ΣΤΙΓΜΑΣǅﬁİ");
        let v = [
            '\u{307}', 'i', 'ﬁ', 'ǆ', 'σ', 'α', 'μ', 'γ', 'ι', 'τ', 'σ', '💩', 'ü', 'ß', 'ö', 'a',
        ];

        let mut pos = 0;
        let it = s.chars().flat_map(|c| c.to_lowercase()).rev();

        for c in it {
            assert_eq!(c, v[pos]);
            pos += 1;
        }
        assert_eq!(pos, v.len());
    }

    #[test]
    fn to_uppercase_rev_iterator() {
        let s = String::from("aößü💩στιγμαςǅﬁᾀ");
        let v = [
            'Ι', 'Ἀ', 'I', 'F', 'Ǆ', 'Σ', 'Α', 'Μ', 'Γ', 'Ι', 'Τ', 'Σ', '💩', 'Ü', 'S', 'S', 'Ö',
            'A',
        ];

        let mut pos = 0;
        let it = s.chars().flat_map(|c| c.to_uppercase()).rev();

        for c in it {
            assert_eq!(c, v[pos]);
            pos += 1;
        }
        assert_eq!(pos, v.len());
    }

    #[test]
    fn chars_decoding() {
        let mut bytes = [0; 4];
        for c in (0..0x110000).filter_map(core::char::from_u32) {
            let s = String::from(c.encode_utf8(&mut bytes));
            if Some(c) != s.chars_lossy().next() {
                panic!("character {:x}={} does not decode correctly", c as u32, c);
            }
        }
    }

    #[test]
    fn chars_rev_decoding() {
        let mut bytes = [0; 4];
        for c in (0..0x110000).filter_map(core::char::from_u32) {
            let s = c.encode_utf8(&mut bytes);
            if Some(c) != s.chars().rev().next() {
                panic!("character {:x}={} does not decode correctly", c as u32, c);
            }
        }
    }
    */

    #[test]
    fn iterator_clone() {
        let s = String::from("ศไทย中华Việt Nam");
        let mut it = s.chars_lossy();
        it.next();
        assert!(it.clone().zip(it).all(|(x, y)| x == y));
    }

    #[test]
    fn iterator_last() {
        let s = String::from("ศไทย中华Việt Nam");
        let mut it = s.chars_lossy();
        it.next();
        assert_eq!(it.last(), Some('m'));
    }

    /*
    #[test]
    fn chars_debug() {
        let s = String::from("ศไทย中华Việt Nam");
        let c = s.chars_lossy();
        assert_eq!(
            format!("{c:?}"),
            r#"Chars(['ศ', 'ไ', 'ท', 'ย', '中', '华', 'V', 'i', 'ệ', 't', ' ', 'N', 'a', 'm'])"#
        );
    }
    */

    #[test]
    fn bytesator() {
        let s = String::from("ศไทย中华Việt Nam");
        let v = [
            224, 184, 168, 224, 185, 132, 224, 184, 151, 224, 184, 162, 228, 184, 173, 229, 141,
            142, 86, 105, 225, 187, 135, 116, 32, 78, 97, 109,
        ];
        let mut pos = 0;

        for b in s.bytes() {
            assert_eq!(b, v[pos]);
            pos += 1;
        }
    }

    #[test]
    fn bytes_revator() {
        let s = String::from("ศไทย中华Việt Nam");
        let v = [
            224, 184, 168, 224, 185, 132, 224, 184, 151, 224, 184, 162, 228, 184, 173, 229, 141,
            142, 86, 105, 225, 187, 135, 116, 32, 78, 97, 109,
        ];
        let mut pos = v.len();

        for b in s.bytes().rev() {
            pos -= 1;
            assert_eq!(b, v[pos]);
        }
    }

    #[test]
    fn bytesator_nth() {
        let s = String::from("ศไทย中华Việt Nam");
        let v = [
            224, 184, 168, 224, 185, 132, 224, 184, 151, 224, 184, 162, 228, 184, 173, 229, 141,
            142, 86, 105, 225, 187, 135, 116, 32, 78, 97, 109,
        ];

        let mut b = s.bytes();
        assert_eq!(b.nth(2).unwrap(), v[2]);
        assert_eq!(b.nth(10).unwrap(), v[10]);
        assert_eq!(b.nth(200), None);
    }

    #[test]
    fn bytesator_count() {
        let s = String::from("ศไทย中华Việt Nam");

        let b = s.bytes();
        assert_eq!(b.count(), 28)
    }

    #[test]
    fn bytesator_last() {
        let s = String::from("ศไทย中华Việt Nam");

        let b = s.bytes();
        assert_eq!(b.last().unwrap(), 109)
    }

    #[test]
    fn char_indicesator() {
        let s = String::from("ศไทย中华Việt Nam");
        let p = [0, 3, 6, 9, 12, 15, 18, 19, 20, 23, 24, 25, 26, 27];
        let v = [
            'ศ', 'ไ', 'ท', 'ย', '中', '华', 'V', 'i', 'ệ', 't', ' ', 'N', 'a', 'm',
        ];

        let mut pos = 0;
        let it = s.char_indices_lossy();

        for c in it {
            assert_eq!(c, (p[pos], v[pos]));
            pos += 1;
        }
        assert_eq!(pos, v.len());
        assert_eq!(pos, p.len());
    }

    /*
    #[test]
    fn char_indices_revator() {
        let s = String::from("ศไทย中华Việt Nam");
        let p = [27, 26, 25, 24, 23, 20, 19, 18, 15, 12, 9, 6, 3, 0];
        let v = [
            'm', 'a', 'N', ' ', 't', 'ệ', 'i', 'V', '华', '中', 'ย', 'ท', 'ไ', 'ศ',
        ];

        let mut pos = 0;
        let it = s.char_indices().rev();

        for c in it {
            assert_eq!(c, (p[pos], v[pos]));
            pos += 1;
        }
        assert_eq!(pos, v.len());
        assert_eq!(pos, p.len());
    }
    */

    #[test]
    fn char_indices_last() {
        let s = String::from("ศไทย中华Việt Nam");
        let mut it = s.char_indices_lossy();
        it.next();
        assert_eq!(it.last(), Some((27, 'm')));
    }

    /*
    #[test]
    fn splitn_char_iterator() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.splitn(4, ' ').collect();
        assert_eq!(split, ["\nMäry", "häd", "ä", "little lämb\nLittle lämb\n"]);

        let split: Vec<&str> = data.splitn(4, |c: char| c == ' ').collect();
        assert_eq!(split, ["\nMäry", "häd", "ä", "little lämb\nLittle lämb\n"]);

        // Unicode
        let split: Vec<&str> = data.splitn(4, 'ä').collect();
        assert_eq!(split, ["\nM", "ry h", "d ", " little lämb\nLittle lämb\n"]);

        let split: Vec<&str> = data.splitn(4, |c: char| c == 'ä').collect();
        assert_eq!(split, ["\nM", "ry h", "d ", " little lämb\nLittle lämb\n"]);
    }

    #[test]
    fn split_char_iterator_no_trailing() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.split('\n').collect();
        assert_eq!(split, ["", "Märy häd ä little lämb", "Little lämb", ""]);

        let split: Vec<&str> = data.split_terminator('\n').collect();
        assert_eq!(split, ["", "Märy häd ä little lämb", "Little lämb"]);
    }

    #[test]
    fn split_char_iterator_inclusive() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.split_inclusive('\n').collect();
        assert_eq!(split, ["\n", "Märy häd ä little lämb\n", "Little lämb\n"]);

        let uppercase_separated = "SheePSharKTurtlECaT";
        let mut first_char = true;
        let split: Vec<&str> = uppercase_separated
            .split_inclusive(|c: char| {
                let split = !first_char && c.is_uppercase();
                first_char = split;
                split
            })
            .collect();
        assert_eq!(split, ["SheeP", "SharK", "TurtlE", "CaT"]);
    }

    #[test]
    fn split_char_iterator_inclusive_rev() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.split_inclusive('\n').rev().collect();
        assert_eq!(split, ["Little lämb\n", "Märy häd ä little lämb\n", "\n"]);

        // Note that the predicate is stateful and thus dependent
        // on the iteration order.
        // (A different predicate is needed for reverse iterator vs normal iterator.)
        // Not sure if anything can be done though.
        let uppercase_separated = "SheePSharKTurtlECaT";
        let mut term_char = true;
        let split: Vec<&str> = uppercase_separated
            .split_inclusive(|c: char| {
                let split = term_char && c.is_uppercase();
                term_char = c.is_uppercase();
                split
            })
            .rev()
            .collect();
        assert_eq!(split, ["CaT", "TurtlE", "SharK", "SheeP"]);
    }

    #[test]
    fn rsplit() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.rsplit(' ').collect();
        assert_eq!(
            split,
            ["lämb\n", "lämb\nLittle", "little", "ä", "häd", "\nMäry"]
        );

        let split: Vec<&str> = data.rsplit("lämb").collect();
        assert_eq!(split, ["\n", "\nLittle ", "\nMäry häd ä little "]);

        let split: Vec<&str> = data.rsplit(|c: char| c == 'ä').collect();
        assert_eq!(
            split,
            ["mb\n", "mb\nLittle l", " little l", "d ", "ry h", "\nM"]
        );
    }

    #[test]
    fn rsplitn() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.rsplitn(2, ' ').collect();
        assert_eq!(split, ["lämb\n", "\nMäry häd ä little lämb\nLittle"]);

        let split: Vec<&str> = data.rsplitn(2, "lämb").collect();
        assert_eq!(split, ["\n", "\nMäry häd ä little lämb\nLittle "]);

        let split: Vec<&str> = data.rsplitn(2, |c: char| c == 'ä').collect();
        assert_eq!(split, ["mb\n", "\nMäry häd ä little lämb\nLittle l"]);
    }

    #[test]
    fn split_once() {
        assert_eq!(String::from("").split_once("->"), None);
        assert_eq!(String::from("-").split_once("->"), None);
        assert_eq!(String::from("->").split_once("->"), Some(("", "")));
        assert_eq!(String::from("a->").split_once("->"), Some(("a", "")));
        assert_eq!(String::from("->b").split_once("->"), Some(("", "b")));
        assert_eq!(String::from("a->b").split_once("->"), Some(("a", "b")));
        assert_eq!(
            String::from("a->b->c").split_once("->"),
            Some(("a", "b->c"))
        );
        assert_eq!(String::from("---").split_once("--"), Some(("", "-")));
    }

    #[test]
    fn rsplit_once() {
        assert_eq!(String::from("").rsplit_once("->"), None);
        assert_eq!(String::from("-").rsplit_once("->"), None);
        assert_eq!(String::from("->").rsplit_once("->"), Some(("", "")));
        assert_eq!(String::from("a->").rsplit_once("->"), Some(("a", "")));
        assert_eq!(String::from("->b").rsplit_once("->"), Some(("", "b")));
        assert_eq!(String::from("a->b").rsplit_once("->"), Some(("a", "b")));
        assert_eq!(
            String::from("a->b->c").rsplit_once("->"),
            Some(("a->b", "c"))
        );
        assert_eq!(String::from("---").rsplit_once("--"), Some(("-", "")));
    }

    #[test]
    fn split_whitespace() {
        let data = "\n \tMäry   häd\tä  little lämb\nLittle lämb\n";
        let words: Vec<&str> = data.split_whitespace().collect();
        assert_eq!(
            words,
            ["Märy", "häd", "ä", "little", "lämb", "Little", "lämb"]
        )
    }

    #[test]
    fn lines() {
        let data = "\nMäry häd ä little lämb\n\r\nLittle lämb\n";
        let lines: Vec<&str> = data.lines().collect();
        assert_eq!(lines, ["", "Märy häd ä little lämb", "", "Little lämb"]);

        let data = "\r\nMäry häd ä little lämb\n\nLittle lämb"; // no trailing \n
        let lines: Vec<&str> = data.lines().collect();
        assert_eq!(lines, ["", "Märy häd ä little lämb", "", "Little lämb"]);
    }

    #[test]
    fn splitator() {
        fn t(s: &str, sep: &str, u: &[&str]) {
            let v: Vec<&str> = s.split(sep).collect();
            assert_eq!(v, u);
        }
        t("--1233345--", "12345", &["--1233345--"]);
        t("abc::hello::there", "::", &["abc", "hello", "there"]);
        t("::hello::there", "::", &["", "hello", "there"]);
        t("hello::there::", "::", &["hello", "there", ""]);
        t("::hello::there::", "::", &["", "hello", "there", ""]);
        t("ประเทศไทย中华Việt Nam", "中华", &["ประเทศไทย", "Việt Nam"]);
        t("zzXXXzzYYYzz", "zz", &["", "XXX", "YYY", ""]);
        t("zzXXXzYYYz", "XXX", &["zz", "zYYYz"]);
        t(".XXX.YYY.", ".", &["", "XXX", "YYY", ""]);
        t("", ".", &[""]);
        t("zz", "zz", &["", ""]);
        t("ok", "z", &["ok"]);
        t("zzz", "zz", &["", "z"]);
        t("zzzzz", "zz", &["", "", "z"]);
    }

    #[test]
    fn str_default() {
        use core::default::Default;

        fn t<S: Default + AsRef<str>>() {
            let s: S = Default::default();
            assert_eq!(s.as_ref(), "");
        }

        t::<&str>();
        t::<String>();
        t::<&mut str>();
    }

    #[test]
    fn pattern_deref_forward() {
        let data = "aabcdaa";
        assert!(data.contains("bcd"));
        assert!(data.contains(&"bcd"));
        assert!(data.contains(&"bcd".to_string()));
    }

    #[test]
    fn empty_match_indices() {
        let data = "aä中!";
        let vec: Vec<_> = data.match_indices("").collect();
        assert_eq!(vec, [(0, ""), (1, ""), (3, ""), (6, ""), (7, "")]);
    }
    */

    fn check_contains_all_substrings(s: &String) {
        assert!(s.contains(&String::from("")));
        // for i in 0..s.len() {
        //     for j in i + 1..=s.len() {
        //         assert!(s.contains(&s[i..j]));
        //     }
        // }
    }

    #[test]
    fn strslice_issue_16589() {
        assert!(String::from("bananas").contains(&String::from("nana")));

        // prior to the fix for #16589, x.contains("abcdabcd") returned false
        // test all substrings for good measure
        check_contains_all_substrings(&String::from("012345678901234567890123456789bcdabcdabcd"));
    }

    #[test]
    fn strslice_issue_16878() {
        assert!(!String::from("1234567ah012345678901ah").contains(&String::from("hah")));
        assert!(!String::from("00abc01234567890123456789abc").contains(&String::from("bcabc")));
    }

    #[test]
    fn strslice_contains() {
        let x =
            String::from("There are moments, Jeeves, when one asks oneself, 'Do trousers matter?'");
        check_contains_all_substrings(&x);
    }

    /*
    #[test]
    fn rsplitn_char_iterator() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let mut split: Vec<&str> = data.rsplitn(4, ' ').collect();
        split.reverse();
        assert_eq!(split, ["\nMäry häd ä", "little", "lämb\nLittle", "lämb\n"]);

        let mut split: Vec<&str> = data.rsplitn(4, |c: char| c == ' ').collect();
        split.reverse();
        assert_eq!(split, ["\nMäry häd ä", "little", "lämb\nLittle", "lämb\n"]);

        // Unicode
        let mut split: Vec<&str> = data.rsplitn(4, 'ä').collect();
        split.reverse();
        assert_eq!(split, ["\nMäry häd ", " little l", "mb\nLittle l", "mb\n"]);

        let mut split: Vec<&str> = data.rsplitn(4, |c: char| c == 'ä').collect();
        split.reverse();
        assert_eq!(split, ["\nMäry häd ", " little l", "mb\nLittle l", "mb\n"]);
    }

    #[test]
    fn split_char_iterator() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let split: Vec<&str> = data.split(' ').collect();
        assert_eq!(
            split,
            ["\nMäry", "häd", "ä", "little", "lämb\nLittle", "lämb\n"]
        );

        let mut rsplit: Vec<&str> = data.split(' ').rev().collect();
        rsplit.reverse();
        assert_eq!(
            rsplit,
            ["\nMäry", "häd", "ä", "little", "lämb\nLittle", "lämb\n"]
        );

        let split: Vec<&str> = data.split(|c: char| c == ' ').collect();
        assert_eq!(
            split,
            ["\nMäry", "häd", "ä", "little", "lämb\nLittle", "lämb\n"]
        );

        let mut rsplit: Vec<&str> = data.split(|c: char| c == ' ').rev().collect();
        rsplit.reverse();
        assert_eq!(
            rsplit,
            ["\nMäry", "häd", "ä", "little", "lämb\nLittle", "lämb\n"]
        );

        // Unicode
        let split: Vec<&str> = data.split('ä').collect();
        assert_eq!(
            split,
            ["\nM", "ry h", "d ", " little l", "mb\nLittle l", "mb\n"]
        );

        let mut rsplit: Vec<&str> = data.split('ä').rev().collect();
        rsplit.reverse();
        assert_eq!(
            rsplit,
            ["\nM", "ry h", "d ", " little l", "mb\nLittle l", "mb\n"]
        );

        let split: Vec<&str> = data.split(|c: char| c == 'ä').collect();
        assert_eq!(
            split,
            ["\nM", "ry h", "d ", " little l", "mb\nLittle l", "mb\n"]
        );

        let mut rsplit: Vec<&str> = data.split(|c: char| c == 'ä').rev().collect();
        rsplit.reverse();
        assert_eq!(
            rsplit,
            ["\nM", "ry h", "d ", " little l", "mb\nLittle l", "mb\n"]
        );
    }

    #[test]
    fn rev_split_char_iterator_no_trailing() {
        let data = "\nMäry häd ä little lämb\nLittle lämb\n";

        let mut split: Vec<&str> = data.split('\n').rev().collect();
        split.reverse();
        assert_eq!(split, ["", "Märy häd ä little lämb", "Little lämb", ""]);

        let mut split: Vec<&str> = data.split_terminator('\n').rev().collect();
        split.reverse();
        assert_eq!(split, ["", "Märy häd ä little lämb", "Little lämb"]);
    }

    #[test]
    fn utf16_code_units() {
        assert_eq!(
            "é\u{1F4A9}".encode_utf16().collect::<Vec<u16>>(),
            [0xE9, 0xD83D, 0xDCA9]
        )
    }
    */

    #[test]
    fn starts_with_in_unicode() {
        assert!(!String::from("├── Cargo.toml").starts_with(&String::from("# ")));
    }

    #[test]
    fn starts_short_long() {
        let empty = String::from("");
        assert!(!empty.starts_with(&String::from("##")));
        assert!(!String::from("##").starts_with(&String::from("####")));
        assert!(String::from("####").starts_with(&String::from("##")));
        assert!(!String::from("##ä").starts_with(&String::from("####")));
        assert!(String::from("####ä").starts_with(&String::from("##")));
        assert!(!String::from("##").starts_with(&String::from("####ä")));
        assert!(String::from("##ä##").starts_with(&String::from("##ä")));

        assert!(empty.starts_with(&empty));
        assert!(String::from("ä").starts_with(&empty));
        assert!(String::from("#ä").starts_with(&empty));
        assert!(String::from("##ä").starts_with(&empty));
        assert!(String::from("ä###").starts_with(&empty));
        assert!(String::from("#ä##").starts_with(&empty));
        assert!(String::from("##ä#").starts_with(&empty));
    }

    #[test]
    fn contains_weird_cases() {
        assert!(String::from("* \t").contains(' '));
        assert!(!String::from("* \t").contains('?'));
        assert!(!String::from("* \t").contains('\u{1F4A9}'));
    }

    /*
    #[test]
    fn trim_ws() {
        assert_eq!(
            " \t  a \t  ".trim_start_matches(|c: char| c.is_whitespace()),
            "a \t  "
        );
        assert_eq!(
            " \t  a \t  ".trim_end_matches(|c: char| c.is_whitespace()),
            " \t  a"
        );
        assert_eq!(
            " \t  a \t  ".trim_start_matches(|c: char| c.is_whitespace()),
            "a \t  "
        );
        assert_eq!(
            " \t  a \t  ".trim_end_matches(|c: char| c.is_whitespace()),
            " \t  a"
        );
        assert_eq!(" \t  a \t  ".trim_matches(|c: char| c.is_whitespace()), "a");
        assert_eq!(
            " \t   \t  ".trim_start_matches(|c: char| c.is_whitespace()),
            ""
        );
        assert_eq!(
            " \t   \t  ".trim_end_matches(|c: char| c.is_whitespace()),
            ""
        );
        assert_eq!(
            " \t   \t  ".trim_start_matches(|c: char| c.is_whitespace()),
            ""
        );
        assert_eq!(
            " \t   \t  ".trim_end_matches(|c: char| c.is_whitespace()),
            ""
        );
        assert_eq!(" \t   \t  ".trim_matches(|c: char| c.is_whitespace()), "");
    }

    #[test]
    fn to_lowercase() {
        assert_eq!("".to_lowercase(), "");
        assert_eq!("AÉǅaé ".to_lowercase(), "aéǆaé ");

        // https://github.com/rust-lang/rust/issues/26035
        assert_eq!("ΑΣ".to_lowercase(), "ας");
        assert_eq!("Α'Σ".to_lowercase(), "α'ς");
        assert_eq!("Α''Σ".to_lowercase(), "α''ς");

        assert_eq!("ΑΣ Α".to_lowercase(), "ας α");
        assert_eq!("Α'Σ Α".to_lowercase(), "α'ς α");
        assert_eq!("Α''Σ Α".to_lowercase(), "α''ς α");

        assert_eq!("ΑΣ' Α".to_lowercase(), "ας' α");
        assert_eq!("ΑΣ'' Α".to_lowercase(), "ας'' α");

        assert_eq!("Α'Σ' Α".to_lowercase(), "α'ς' α");
        assert_eq!("Α''Σ'' Α".to_lowercase(), "α''ς'' α");

        assert_eq!("Α Σ".to_lowercase(), "α σ");
        assert_eq!("Α 'Σ".to_lowercase(), "α 'σ");
        assert_eq!("Α ''Σ".to_lowercase(), "α ''σ");

        assert_eq!("Σ".to_lowercase(), "σ");
        assert_eq!("'Σ".to_lowercase(), "'σ");
        assert_eq!("''Σ".to_lowercase(), "''σ");

        assert_eq!("ΑΣΑ".to_lowercase(), "ασα");
        assert_eq!("ΑΣ'Α".to_lowercase(), "ασ'α");
        assert_eq!("ΑΣ''Α".to_lowercase(), "ασ''α");
    }

    #[test]
    fn to_uppercase() {
        assert_eq!("".to_uppercase(), "");
        assert_eq!("aéǅßﬁᾀ".to_uppercase(), "AÉǄSSFIἈΙ");
    }

    #[test]
    fn into_string() {
        // The only way to acquire a Box<str> in the first place is through a String, so just
        // test that we can round-trip between Box<str> and String.
        let string = String::from("Some text goes here");
        assert_eq!(string.clone().into_boxed_str().into_string(), string);
    }

    #[test]
    fn box_slice_clone() {
        let data = String::from("hello HELLO hello HELLO yes YES 5 中ä华!!!");
        let data2 = data.clone().into_boxed_str().clone().into_string();

        assert_eq!(data, data2);
    }

    #[test]
    fn cow_from() {
        let borrowed = "borrowed";
        let owned = String::from("owned");
        match (Cow::from(owned.clone()), Cow::from(borrowed)) {
            (Cow::Owned(o), Cow::Borrowed(b)) => {
                assert!(o == owned && b == borrowed);
            }
            _ => panic!("invalid `Cow::from`"),
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn repeat() {
        assert_eq!("".repeat(3), "");
        assert_eq!("abc".repeat(0), "");
        assert_eq!("α".repeat(3), "ααα");
    }

    mod pattern {
        use std::str::pattern::SearchStep::{self, Done, Match, Reject};
        use std::str::pattern::{Pattern, ReverseSearcher, Searcher};

        macro_rules! make_test {
            ($name:ident, $p:expr, $h:expr, [$($e:expr,)*]) => {
                #[allow(unused_imports)]
                mod $name {
                    use std::str::pattern::SearchStep::{Match, Reject};
                    use super::cmp_search_to_vec;
                    #[test]
                    fn fwd() {
                        cmp_search_to_vec(false, $p, $h, vec![$($e),*]);
                    }
                    #[test]
                    fn bwd() {
                        cmp_search_to_vec(true, $p, $h, vec![$($e),*]);
                    }
                }
            }
        }

        fn cmp_search_to_vec<'a>(
            rev: bool,
            pat: impl Pattern<'a, Searcher: ReverseSearcher<'a>>,
            haystack: &'a str,
            right: Vec<SearchStep>,
        ) {
            let mut searcher = pat.into_searcher(haystack);
            let mut v = vec![];
            loop {
                match if !rev {
                    searcher.next()
                } else {
                    searcher.next_back()
                } {
                    Match(a, b) => v.push(Match(a, b)),
                    Reject(a, b) => v.push(Reject(a, b)),
                    Done => break,
                }
            }
            if rev {
                v.reverse();
            }

            let mut first_index = 0;
            let mut err = None;

            for (i, e) in right.iter().enumerate() {
                match *e {
                    Match(a, b) | Reject(a, b) if a <= b && a == first_index => {
                        first_index = b;
                    }
                    _ => {
                        err = Some(i);
                        break;
                    }
                }
            }

            if let Some(err) = err {
                panic!("Input skipped range at {err}");
            }

            if first_index != haystack.len() {
                panic!("Did not cover whole input");
            }

            assert_eq!(v, right);
        }

        make_test!(
            str_searcher_ascii_haystack,
            "bb",
            "abbcbbd",
            [
                Reject(0, 1),
                Match(1, 3),
                Reject(3, 4),
                Match(4, 6),
                Reject(6, 7),
            ]
        );
        make_test!(
            str_searcher_ascii_haystack_seq,
            "bb",
            "abbcbbbbd",
            [
                Reject(0, 1),
                Match(1, 3),
                Reject(3, 4),
                Match(4, 6),
                Match(6, 8),
                Reject(8, 9),
            ]
        );
        make_test!(
            str_searcher_empty_needle_ascii_haystack,
            "",
            "abbcbbd",
            [
                Match(0, 0),
                Reject(0, 1),
                Match(1, 1),
                Reject(1, 2),
                Match(2, 2),
                Reject(2, 3),
                Match(3, 3),
                Reject(3, 4),
                Match(4, 4),
                Reject(4, 5),
                Match(5, 5),
                Reject(5, 6),
                Match(6, 6),
                Reject(6, 7),
                Match(7, 7),
            ]
        );
        make_test!(
            str_searcher_multibyte_haystack,
            " ",
            "├──",
            [Reject(0, 3), Reject(3, 6), Reject(6, 9),]
        );
        make_test!(
            str_searcher_empty_needle_multibyte_haystack,
            "",
            "├──",
            [
                Match(0, 0),
                Reject(0, 3),
                Match(3, 3),
                Reject(3, 6),
                Match(6, 6),
                Reject(6, 9),
                Match(9, 9),
            ]
        );
        make_test!(
            str_searcher_empty_needle_empty_haystack,
            "",
            "",
            [Match(0, 0),]
        );
        make_test!(str_searcher_nonempty_needle_empty_haystack, "├", "", []);
        make_test!(
            char_searcher_ascii_haystack,
            'b',
            "abbcbbd",
            [
                Reject(0, 1),
                Match(1, 2),
                Match(2, 3),
                Reject(3, 4),
                Match(4, 5),
                Match(5, 6),
                Reject(6, 7),
            ]
        );
        make_test!(
            char_searcher_multibyte_haystack,
            ' ',
            "├──",
            [Reject(0, 3), Reject(3, 6), Reject(6, 9),]
        );
        make_test!(
            char_searcher_short_haystack,
            '\u{1F4A9}',
            "* \t",
            [Reject(0, 1), Reject(1, 2), Reject(2, 3),]
        );

        // See #85462
        #[test]
        fn str_searcher_empty_needle_after_done() {
            // Empty needle and haystack
            {
                let mut searcher = "".into_searcher("");

                assert_eq!(searcher.next(), SearchStep::Match(0, 0));
                assert_eq!(searcher.next(), SearchStep::Done);
                assert_eq!(searcher.next(), SearchStep::Done);
                assert_eq!(searcher.next(), SearchStep::Done);

                let mut searcher = "".into_searcher("");

                assert_eq!(searcher.next_back(), SearchStep::Match(0, 0));
                assert_eq!(searcher.next_back(), SearchStep::Done);
                assert_eq!(searcher.next_back(), SearchStep::Done);
                assert_eq!(searcher.next_back(), SearchStep::Done);
            }
            // Empty needle and non-empty haystack
            {
                let mut searcher = "".into_searcher("a");

                assert_eq!(searcher.next(), SearchStep::Match(0, 0));
                assert_eq!(searcher.next(), SearchStep::Reject(0, 1));
                assert_eq!(searcher.next(), SearchStep::Match(1, 1));
                assert_eq!(searcher.next(), SearchStep::Done);
                assert_eq!(searcher.next(), SearchStep::Done);
                assert_eq!(searcher.next(), SearchStep::Done);

                let mut searcher = "".into_searcher("a");

                assert_eq!(searcher.next_back(), SearchStep::Match(1, 1));
                assert_eq!(searcher.next_back(), SearchStep::Reject(0, 1));
                assert_eq!(searcher.next_back(), SearchStep::Match(0, 0));
                assert_eq!(searcher.next_back(), SearchStep::Done);
                assert_eq!(searcher.next_back(), SearchStep::Done);
                assert_eq!(searcher.next_back(), SearchStep::Done);
            }
        }
    }

    macro_rules! generate_iterator_test {
        {
            $name:ident {
                $(
                    ($($arg:expr),*) -> [$($t:tt)*];
                )*
            }
            with $fwd:expr, $bwd:expr;
        } => {
            #[test]
            fn $name() {
                $(
                    {
                        let res = vec![$($t)*];

                        let fwd_vec: Vec<_> = ($fwd)($($arg),*).collect();
                        assert_eq!(fwd_vec, res);

                        let mut bwd_vec: Vec<_> = ($bwd)($($arg),*).collect();
                        bwd_vec.reverse();
                        assert_eq!(bwd_vec, res);
                    }
                )*
            }
        };
        {
            $name:ident {
                $(
                    ($($arg:expr),*) -> [$($t:tt)*];
                )*
            }
            with $fwd:expr;
        } => {
            #[test]
            fn $name() {
                $(
                    {
                        let res = vec![$($t)*];

                        let fwd_vec: Vec<_> = ($fwd)($($arg),*).collect();
                        assert_eq!(fwd_vec, res);
                    }
                )*
            }
        }
    }

    generate_iterator_test! {
        double_ended_split {
            ("foo.bar.baz", '.') -> ["foo", "bar", "baz"];
            ("foo::bar::baz", "::") -> ["foo", "bar", "baz"];
        }
        with str::split, str::rsplit;
    }

    generate_iterator_test! {
        double_ended_split_terminator {
            ("foo;bar;baz;", ';') -> ["foo", "bar", "baz"];
        }
        with str::split_terminator, str::rsplit_terminator;
    }

    generate_iterator_test! {
        double_ended_matches {
            ("a1b2c3", char::is_numeric) -> ["1", "2", "3"];
        }
        with str::matches, str::rmatches;
    }

    generate_iterator_test! {
        double_ended_match_indices {
            ("a1b2c3", char::is_numeric) -> [(1, "1"), (3, "2"), (5, "3")];
        }
        with str::match_indices, str::rmatch_indices;
    }

    generate_iterator_test! {
        not_double_ended_splitn {
            ("foo::bar::baz", 2, "::") -> ["foo", "bar::baz"];
        }
        with str::splitn;
    }

    generate_iterator_test! {
        not_double_ended_rsplitn {
            ("foo::bar::baz", 2, "::") -> ["baz", "foo::bar"];
        }
        with str::rsplitn;
    }

    #[test]
    fn different_str_pattern_forwarding_lifetimes() {
        use std::str::pattern::Pattern;

        fn foo<'a, P>(p: P)
        where
            for<'b> &'b P: Pattern<'a>,
        {
            for _ in 0..3 {
                "asdf".find(&p);
            }
        }

        foo::<&str>("x");
    }

    #[test]
    fn const_str_ptr() {
        const A: [u8; 2] = ['h' as u8, 'i' as u8];
        const B: &'static [u8; 2] = &A;
        const C: *const u8 = B as *const u8;

        {
            let foo = &A as *const u8;
            assert_eq!(foo, C);
        }

        unsafe {
            assert_eq!(from_utf8_unchecked(&A), "hi");
            assert_eq!(*C, A[0]);
            assert_eq!(*(&B[0] as *const u8), A[0]);
        }
    }

    #[test]
    fn utf8() {
        let yen: char = '¥'; // 0xa5
        let c_cedilla: char = 'ç'; // 0xe7
        let thorn: char = 'þ'; // 0xfe
        let y_diaeresis: char = 'ÿ'; // 0xff
        let pi: char = 'Π'; // 0x3a0

        assert_eq!(yen as isize, 0xa5);
        assert_eq!(c_cedilla as isize, 0xe7);
        assert_eq!(thorn as isize, 0xfe);
        assert_eq!(y_diaeresis as isize, 0xff);
        assert_eq!(pi as isize, 0x3a0);

        assert_eq!(pi as isize, '\u{3a0}' as isize);
        assert_eq!('\x0a' as isize, '\n' as isize);

        let bhutan: String = "འབྲུག་ཡུལ།".to_string();
        let japan: String = "日本".to_string();
        let uzbekistan: String = "Ўзбекистон".to_string();
        let austria: String = "Österreich".to_string();

        let bhutan_e: String =
            "\u{f60}\u{f56}\u{fb2}\u{f74}\u{f42}\u{f0b}\u{f61}\u{f74}\u{f63}\u{f0d}".to_string();
        let japan_e: String = "\u{65e5}\u{672c}".to_string();
        let uzbekistan_e: String =
            "\u{40e}\u{437}\u{431}\u{435}\u{43a}\u{438}\u{441}\u{442}\u{43e}\u{43d}".to_string();
        let austria_e: String = "\u{d6}sterreich".to_string();

        let oo: char = 'Ö';
        assert_eq!(oo as isize, 0xd6);

        fn check_str_eq(a: String, b: String) {
            let mut i: isize = 0;
            for ab in a.bytes() {
                println!("{i}");
                println!("{ab}");
                let bb: u8 = b.as_bytes()[i as usize];
                println!("{bb}");
                assert_eq!(ab, bb);
                i += 1;
            }
        }

        check_str_eq(bhutan, bhutan_e);
        check_str_eq(japan, japan_e);
        check_str_eq(uzbekistan, uzbekistan_e);
        check_str_eq(austria, austria_e);
    }

    #[test]
    fn utf8_chars() {
        // Chars of 1, 2, 3, and 4 bytes
        let chs: Vec<char> = vec!['e', 'é', '€', '\u{10000}'];
        let s: String = chs.iter().cloned().collect();
        let schs: Vec<char> = s.chars().collect();

        assert_eq!(s.len(), 10);
        assert_eq!(s.chars().count(), 4);
        assert_eq!(schs.len(), 4);
        assert_eq!(schs.iter().cloned().collect::<String>(), s);

        assert!((from_utf8(s.as_bytes()).is_ok()));
        // invalid prefix
        assert!((!from_utf8(&[0x80]).is_ok()));
        // invalid 2 byte prefix
        assert!((!from_utf8(&[0xc0]).is_ok()));
        assert!((!from_utf8(&[0xc0, 0x10]).is_ok()));
        // invalid 3 byte prefix
        assert!((!from_utf8(&[0xe0]).is_ok()));
        assert!((!from_utf8(&[0xe0, 0x10]).is_ok()));
        assert!((!from_utf8(&[0xe0, 0xff, 0x10]).is_ok()));
        // invalid 4 byte prefix
        assert!((!from_utf8(&[0xf0]).is_ok()));
        assert!((!from_utf8(&[0xf0, 0x10]).is_ok()));
        assert!((!from_utf8(&[0xf0, 0xff, 0x10]).is_ok()));
        assert!((!from_utf8(&[0xf0, 0xff, 0xff, 0x10]).is_ok()));
    }

    #[test]
    fn utf8_char_counts() {
        let strs = [
            ("e", 1),
            ("é", 1),
            ("€", 1),
            ("\u{10000}", 1),
            ("eé€\u{10000}", 4),
        ];
        let mut reps = [8, 64, 256, 512, 1024]
            .iter()
            .copied()
            .flat_map(|n| n - 8..=n + 8)
            .collect::<Vec<usize>>();
        if cfg!(not(miri)) {
            let big = 1 << 16;
            reps.extend(big - 8..=big + 8);
        }
        let counts = if cfg!(miri) { 0..1 } else { 0..8 };
        let padding = counts.map(|len| " ".repeat(len)).collect::<Vec<String>>();

        for repeat in reps {
            for (tmpl_str, tmpl_char_count) in strs {
                for pad_start in &padding {
                    for pad_end in &padding {
                        // Create a string with padding...
                        let with_padding =
                            format!("{}{}{}", pad_start, tmpl_str.repeat(repeat), pad_end);
                        // ...and then skip past that padding. This should ensure
                        // that we test several different alignments for both head
                        // and tail.
                        let si = pad_start.len();
                        let ei = with_padding.len() - pad_end.len();
                        let target = &with_padding[si..ei];

                        assert!(!target.starts_with(" ") && !target.ends_with(" "));
                        let expected_count = tmpl_char_count * repeat;
                        assert_eq!(
                            expected_count,
                            target.chars().count(),
                            "wrong count for `{:?}.repeat({})` (padding: `{:?}`)",
                            tmpl_str,
                            repeat,
                            (pad_start.len(), pad_end.len()),
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn floor_char_boundary() {
        fn check_many(s: &str, arg: impl IntoIterator<Item = usize>, ret: usize) {
            for idx in arg {
                assert_eq!(
                    s.floor_char_boundary(idx),
                    ret,
                    "{:?}.floor_char_boundary({:?}) != {:?}",
                    s,
                    idx,
                    ret
                );
            }
        }

        // edge case
        check_many("", [0, 1, isize::MAX as usize, usize::MAX], 0);

        // basic check
        check_many("x", [0], 0);
        check_many("x", [1, isize::MAX as usize, usize::MAX], 1);

        // 1-byte chars
        check_many("jp", [0], 0);
        check_many("jp", [1], 1);
        check_many("jp", 2..4, 2);

        // 2-byte chars
        check_many("ĵƥ", 0..2, 0);
        check_many("ĵƥ", 2..4, 2);
        check_many("ĵƥ", 4..6, 4);

        // 3-byte chars
        check_many("日本", 0..3, 0);
        check_many("日本", 3..6, 3);
        check_many("日本", 6..8, 6);

        // 4-byte chars
        check_many("🇯🇵", 0..4, 0);
        check_many("🇯🇵", 4..8, 4);
        check_many("🇯🇵", 8..10, 8);
    }

    #[test]
    fn ceil_char_boundary() {
        fn check_many(s: &str, arg: impl IntoIterator<Item = usize>, ret: usize) {
            for idx in arg {
                assert_eq!(
                    s.ceil_char_boundary(idx),
                    ret,
                    "{:?}.ceil_char_boundary({:?}) != {:?}",
                    s,
                    idx,
                    ret
                );
            }
        }

        // edge case
        check_many("", [0], 0);

        // basic check
        check_many("x", [0], 0);
        check_many("x", [1], 1);

        // 1-byte chars
        check_many("jp", [0], 0);
        check_many("jp", [1], 1);
        check_many("jp", [2], 2);

        // 2-byte chars
        check_many("ĵƥ", 0..=0, 0);
        check_many("ĵƥ", 1..=2, 2);
        check_many("ĵƥ", 3..=4, 4);

        // 3-byte chars
        check_many("日本", 0..=0, 0);
        check_many("日本", 1..=3, 3);
        check_many("日本", 4..=6, 6);

        // 4-byte chars
        check_many("🇯🇵", 0..=0, 0);
        check_many("🇯🇵", 1..=4, 4);
        check_many("🇯🇵", 5..=8, 8);
    }

    #[test]
    #[should_panic]
    fn ceil_char_boundary_above_len_panic() {
        let _ = "x".ceil_char_boundary(2);
    }
    */
}

flipperzero_test::tests_runner!(
    name = "String Integration Test",
    stack_size = 4096,
    [crate::tests, crate::str_tests]
);
