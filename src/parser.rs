// auto-generated: "lalrpop 0.19.6"
// sha3: 57748fe6c1102c92aab076fac7e127c145efe5918a4beb1330c4951b348d67cf
use std::str::FromStr;
use crate::ast::{Type, Node, Operator};
use crate::ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use crate::ast::{Type, Node, Operator};
    use crate::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Node),
        Variant2(Vec<Node>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 0, 0, 23, 0, 24,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 0, 0, 23, 0, 24,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 3
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 35, 0, 23, 0, 24,
        // State 6
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 7
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 5, 6, 42, 0, 23, 0, 24,
        // State 10
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 11
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 12
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 13
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 33, 24,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, 0, -22, 0, -22,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, -25, -25, -25, -25, -25, 0, -25, 0, -25,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, 0, -27, 0, -27,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, 0, -24, 0, -24,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, 0, -23, 0, -23,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, -26, -26, -26, -26, -26, 0, -26, 0, -26,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19, -19, 0, 0, -19, 0, -19,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, 0, -5, 0, -5,
        // State 23
        0, -12, -12, -12, -12, -12, -12, -12, 0, 0, 0, -12, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20, -20, 0, 0, -20, 0, -20,
        // State 25
        0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -15, -15, -15, -15, -15, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -6, 11, 12, 13, 14, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, -13, -13, -13, -13, -13, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -14, -14, -14, -14, -14, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -21, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, -16, -16, -16, -16, -16, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, -28, -28, -28, -28, -28, 0, -28, 0, -28,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, -2, -2, -2, -2, -2, 0, -2, 0, -2,
        // State 35
        0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, 0, -4, 0, -4,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17, -17, -17, 0, -17, 0, -17,
        // State 38
        0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18, -18, -18, 0, -18, 0, -18,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, -29, -29, -29, -29, -29, 0, -29, 0, -29,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, -3, -3, -3, -3, -3, 0, -3, 0, -3,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, 0, -1, 0, -1,
        // State 43
        0, -9, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, -7, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, -8, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -10, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, -11, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 17 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -30,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -22,
        // State 15
        -25,
        // State 16
        -27,
        // State 17
        -24,
        // State 18
        0,
        // State 19
        -23,
        // State 20
        -26,
        // State 21
        -19,
        // State 22
        -5,
        // State 23
        0,
        // State 24
        -20,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -2,
        // State 35
        0,
        // State 36
        -4,
        // State 37
        -17,
        // State 38
        0,
        // State 39
        -18,
        // State 40
        0,
        // State 41
        -3,
        // State 42
        -1,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 14,
            1 => match state {
                8 => 39,
                _ => 15,
            },
            2 => 16,
            3 => 17,
            4 => match state {
                6 => 35,
                7 => 38,
                10 => 43,
                11 => 44,
                12 => 45,
                13 => 46,
                _ => 26,
            },
            5 => match state {
                4 => 8,
                2 => 25,
                3 | 6..=7 | 10..=13 => 27,
                _ => 18,
            },
            6 => 28,
            7 => 29,
            8 => 19,
            9 => 20,
            10 => 1,
            11 => 30,
            12 => match state {
                1 => 24,
                5 => 33,
                9 => 40,
                _ => 21,
            },
            13 => 9,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###""call""###,
            r###""print""###,
            r###""proc""###,
            r###""{""###,
            r###""}""###,
            r###"r#"\"(\\\\.|[^\"])*\""#"###,
            r###"r#"//.*"#"###,
            r###"r#"[0-9]+(\\.[0-9]+)?"#"###,
            r###"r#"[a-zA-Z_][a-zA-Z0-9_]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Vec<Node>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 17 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(4, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(6, _) if true => Some(2),
            Token(7, _) if true => Some(3),
            Token(8, _) if true => Some(4),
            Token(9, _) if true => Some(5),
            Token(10, _) if true => Some(6),
            Token(11, _) if true => Some(7),
            Token(12, _) if true => Some(8),
            Token(13, _) if true => Some(9),
            Token(14, _) if true => Some(10),
            Token(15, _) if true => Some(11),
            Token(16, _) if true => Some(12),
            Token(0, _) if true => Some(13),
            Token(1, _) if true => Some(14),
            Token(2, _) if true => Some(15),
            Token(3, _) if true => Some(16),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 => match __token {
                Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            let __builder = super::__intern_token::new_builder();
            ProgramParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Vec<Node>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Vec<Node>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Node, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Node>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Assign = Ident, "=", Expr, ";" => ActionFn(13);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Block = "{", "}" => ActionFn(24);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action24::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Block = "{", TopList, "}" => ActionFn(25);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Call = "call", Ident, ";" => ActionFn(27);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action27::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Comment = r#"//.*"# => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Literal => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Literal, "+", Expr => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Literal, "-", Expr => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Literal, "*", Expr => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Literal, "/", Expr => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = "(", Expr, ")" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = r#"[a-zA-Z_][a-zA-Z0-9_]*"# => ActionFn(2);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = Num => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = Str => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = Ident => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[0-9]+(\\.[0-9]+)?"# => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Print = "print", Expr, ";" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Proc = "proc", Ident, Block => ActionFn(26);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 9)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Top => ActionFn(28);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = Program, Top => ActionFn(29);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Str = r#"\"(\\\\.|[^\"])*\""# => ActionFn(3);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Assign => ActionFn(16);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Print => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Comment => ActionFn(18);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Block => ActionFn(19);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Proc => ActionFn(20);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Top = Call => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TopList = Top => ActionFn(22);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TopList = TopList, Top => ActionFn(23);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 13)
    }
}
pub use self::__parse__Program::ProgramParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use crate::ast::{Type, Node, Operator};
    use crate::ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^(\"(\\\\[\u{0}-\t\u{b}-\u{10ffff}]|[\u{0}-!\\#-\u{10ffff}])*\")", false),
            ("^(//[\u{0}-\t\u{b}-\u{10ffff}]*)", false),
            ("^([0-9]+(\\.[0-9]+)?)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(;)", false),
            ("^(=)", false),
            ("^(call)", false),
            ("^(print)", false),
            ("^(proc)", false),
            ("^(\\{)", false),
            ("^(\\})", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Node>, usize),
) -> Vec<Node>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> Node
{
    Node::Num(n.to_string())
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> Node
{
    Node::Identifier(n.to_string())
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> Node
{
    Node::Str(n.to_string())
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
) -> Node
{
    n
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Node, usize),
) -> Node
{
    Node::Add(Box::new(n), Box::new(m))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Node, usize),
) -> Node
{
    Node::Sub(Box::new(n), Box::new(m))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Node, usize),
) -> Node
{
    Node::Mul(Box::new(n), Box::new(m))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Node, usize),
) -> Node
{
    Node::Div(Box::new(n), Box::new(m))
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Node
{
    e
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, m, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Node
{
    ast::Node::Assign(Box::new(n), Box::new(m))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Node
{
    ast::Node::Print(Box::new(e))
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, &'input str, usize),
) -> Node
{
    Node::Comment(n.to_string())
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Node
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Vec<Node>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, Vec<Node>, usize),
    (_, tail, _): (usize, Node, usize),
) -> Vec<Node>
{
    vec![head, vec![tail]].concat()
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
    (_, __1, _): (usize, &'input str, usize),
) -> Node
{
    Node::Block(vec![])
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, body, _): (usize, Vec<Node>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Node
{
    Node::Block(body)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, name, _): (usize, Node, usize),
    (_, body, _): (usize, Node, usize),
) -> Node
{
    ast::Node::Proc(Box::new(name), Box::new(body))
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, proc_name, _): (usize, Node, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Node
{
    ast::Node::Call(Box::new(proc_name))
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Node, usize),
) -> Vec<Node>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, head, _): (usize, Vec<Node>, usize),
    (_, tail, _): (usize, Node, usize),
) -> Vec<Node>
{
    vec![head, vec![tail]].concat()
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
