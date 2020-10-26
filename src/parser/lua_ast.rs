use std::fmt::Write;

use super::common::*;
use crate::config::*;
use crate::formatting::decoration::*;
use crate::formatting::list;
use crate::formatting::loc_hint::*;
use crate::formatting::util;
use crate::{cfg_write, cfg_write_helper, test_oneline, test_oneline_no_nl, test_oneline_no_nl_after_nl};

#[derive(Debug)]
pub struct TableConstructorOpts {
    pub is_all_sequential: Option<bool>,
    pub is_single_child: Option<bool>,
    pub is_first_child: Option<bool>,
}

impl TableConstructorOpts {
    pub const fn default() -> Self {
        TableConstructorOpts { is_all_sequential: None, is_single_child: None, is_first_child: None }
    }
}

#[derive(Debug)]
pub struct FieldsOpts {
    pub is_all_sequential: Option<bool>,
    pub is_single_child: Option<bool>,
}

impl FieldsOpts {
    pub const fn default() -> Self {
        FieldsOpts { is_all_sequential: None, is_single_child: None }
    }
}

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str<'static>, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str<'static>, Box<Node>),
    UnaryNot(Loc, [Loc; 1], Box<Node>),

    Var(Loc, [Loc; 1], Box<Node>, Box<Node>),
    RoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBrackets(Loc, [Loc; 2], Box<Node>),
    ArgsRoundBracketsEmpty(Loc, [Loc; 1]),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultiLineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, [Loc; 2], Box<Node>, TableConstructorOpts),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc, String)>, FieldsOpts),
    FieldNamedBracket(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FieldNamed(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, [Loc; 2], Box<Node>),
    TableMember(Loc, [Loc; 1], Box<Node>),
    ExpList(Loc, Vec<(Loc, Node, Loc, String)>),
    NameList(Loc, Vec<(Loc, Node, Loc, String)>),
    ParList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarList(Loc, Vec<(Loc, Node, Loc, String)>),
    VarRoundSuffix(Loc, [Loc; 3], Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<(Loc, Node)>),
    FnMethodCall(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FunctionDef(Loc, [Loc; 1], Box<Node>),
    FuncBody(Loc, [Loc; 2]),
    FuncBodyB(Loc, [Loc; 3], Box<Node>),
    FuncPBody(Loc, [Loc; 3], Box<Node>),
    FuncPBodyB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FuncName(Loc, Vec<(Loc, Node, Loc, String)>),
    FuncNameSelf(Loc, [Loc; 2], Vec<(Loc, Node, Loc, String)>, Box<Node>),
    FuncDecl(Loc, [Loc; 2], Box<Node>, Box<Node>),
    LocalFuncDecl(Loc, [Loc; 3], Box<Node>, Box<Node>),

    StatementList(Loc, Vec<(Loc, Node)>),
    DoEnd(Loc, [Loc; 1]),
    DoBEnd(Loc, [Loc; 2], Box<Node>),
    VarsExprs(Loc, [Loc; 2], Box<Node>, Box<Node>),
    Name(Loc, String),
    Label(Loc, [Loc; 2], Box<Node>),
    GoTo(Loc, [Loc; 1], Box<Node>),
    WhileDo(Loc, [Loc; 3], Box<Node>),
    WhileDoB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    RepeatUntil(Loc, [Loc; 2], Box<Node>),
    RepeatBUntil(Loc, [Loc; 3], Box<Node>, Box<Node>),
    ForInt(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>),
    ForIntB(Loc, [Loc; 8], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStep(Loc, [Loc; 9], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForIntStepB(Loc, [Loc; 10], Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ForRange(Loc, [Loc; 5], Box<Node>, Box<Node>),
    ForRangeB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),

    LocalNames(Loc, [Loc; 1], Box<Node>),
    LocalNamesExprs(Loc, [Loc; 3], Box<Node>, Box<Node>),

    IfThen(Loc, [Loc; 3], Box<Node>),
    IfThenB(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenElse(Loc, [Loc; 4], Box<Node>),
    IfThenBElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenElseB(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIf(Loc, [Loc; 4], Box<Node>, Box<Node>),
    IfThenBElseIf(Loc, [Loc; 5], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElse(Loc, [Loc; 5], Box<Node>, Box<Node>),
    IfThenBElseIfElse(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenElseIfElseB(Loc, [Loc; 6], Box<Node>, Box<Node>, Box<Node>),
    IfThenBElseIfElseB(Loc, [Loc; 7], Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ElseIfThenVec(Loc, Vec<(Loc, Node)>),
    ElseIfThen(Loc, [Loc; 2], Box<Node>),
    ElseIfThenB(Loc, [Loc; 3], Box<Node>, Box<Node>),

    RetStatNone(Loc),
    RetStatExpr(Loc, [Loc; 1], Box<Node>),
    RetStatNoneComma(Loc, [Loc; 1]),
    RetStatExprComma(Loc, [Loc; 2], Box<Node>),
    StatsRetStat(Loc, [Loc; 1], Box<Node>, Box<Node>),
    Chunk(Loc, Box<Node>, Loc),
    SheBangChunk(Loc, Box<Node>, Loc, Box<Node>, Loc),
    Semicolon(Loc),
    SheBang(Loc, String),
}

impl<'a> list::AnyListItem<'a, Node> for Node {
    fn list_item_prefix_hint(&self, _: &'a Config) -> &'a str {
        use Node::*;
        match self {
            Semicolon(..)
            | ArgsRoundBrackets(..)
            | ArgsRoundBracketsEmpty(..)
            | TableIndex(..)
            | TableMember(..)
            | FnMethodCall(..)
            | TableConstructor(..)
            | TableConstructorEmpty(..) => "",
            _ => " ",
        }
    }

    fn need_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool {
        use Node::*;
        match parent {
            StatementList(..) => cfg.newline_format_statement.is_some(),
            ExpList(..) => match cfg.newline_format_exp_list {
                Some(1) => match cfg.enable_oneline_exp_list {
                    Some(true) => test_oneline!(f, cfg, buf, state, self).is_none(),
                    _ => true,
                },
                _ => false,
            },
            ElseIfThenVec(..) => cfg.newline_format_if == Some(1),
            Fields(_, _, opts) => match cfg.newline_format_table_field.is_some() {
                true => {
                    if cfg.enable_oneline_iv_table == Some(true) && opts.is_all_sequential == Some(true) {
                        return false;
                    } else {
                        match cfg.enable_oneline_table_field {
                            Some(true) => self.test_oneline_table_field(f, cfg, buf, state) == None,
                            _ => true,
                        }
                    }
                }
                _ => false,
            },
            VarSuffixList(..) => match self {
                FnMethodCall(..) | TableMember(..) => match cfg.newline_format_var_suffix.is_some() {
                    true => match cfg.enable_oneline_var_suffix {
                        Some(true) => match self {
                            FnMethodCall(_, locs, n1, n2) => match &**n2 {
                                TableConstructorEmpty(..) | TableConstructor(..) | ArgsRoundBracketsEmpty(..) => {
                                    test_oneline!(f, cfg, buf, state, ":", CommentLocHint(&locs[0], ""), n1).is_none()
                                }
                                ArgsRoundBrackets(..) if cfg.newline_format_exp_list_first.is_some() => {
                                    test_oneline!(f, cfg, buf, state, ":", CommentLocHint(&locs[0], ""), n1).is_none()
                                }
                                _ => test_oneline!(f, cfg, buf, state, self).is_none(),
                            },
                            TableMember(..) => test_oneline!(f, cfg, buf, state, self).is_none(),
                            _ => false,
                        },
                        _ => true,
                    },
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        }
    }

    fn need_first_newline(&self, parent: &Node, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> bool {
        use Node::*;
        match parent {
            Fields(_, _, opts) => match cfg.newline_format_table_field.is_some() {
                true => {
                    if cfg.enable_oneline_iv_table == Some(true) && opts.is_all_sequential == Some(true) {
                        return opts.is_single_child != Some(true);
                    } else {
                        match cfg.enable_oneline_table_field {
                            Some(true) => self.test_oneline_table_field(f, cfg, buf, state) == None,
                            _ => true,
                        }
                    }
                }
                _ => false,
            },
            ExpList(..) => match cfg.newline_format_exp_list_first {
                // first!
                Some(1) => match cfg.enable_oneline_exp_list {
                    Some(true) => test_oneline!(f, cfg, buf, state, self).is_none(),
                    _ => true,
                },
                _ => false,
            },
            VarSuffixList(..) => self.need_newline(parent, f, cfg, buf, state),
            _ => false,
        }
    }
}

impl list::SepListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec<(Loc, Node, Loc, String)>> {
        use Node::*;
        match self {
            Fields(_, items, _)
            | ExpList(_, items)
            | NameList(_, items)
            | VarList(_, items)
            | ParList(_, items)
            | FuncName(_, items)
            | FuncNameSelf(_, _, items, _) => Some(items),
            _ => None,
        }
    }

    fn element_prefix_hint(&self) -> &str {
        use Node::*;
        match self {
            Fields(..) | ExpList(..) | NameList(..) | VarList(..) | ParList(..) => " ",
            FuncName(..) | FuncNameSelf(..) => "",
            _ => "",
        }
    }

    fn separator(&self, cfg: &Config) -> Option<String> {
        use Node::*;
        match self {
            Fields(..) => cfg.field_separator.clone(),
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) => Some(",".to_string()),
            FuncName(..) | FuncNameSelf(..) => Some(".".to_string()),
            _ => None,
        }
    }

    fn trailing_separator(&self, cfg: &Config) -> Option<bool> {
        use Node::*;
        match self {
            Fields(_, _, opts) => {
                if cfg.enable_oneline_iv_table == Some(true) && opts.is_all_sequential == Some(true) {
                    Some(false)
                } else {
                    cfg.write_trailing_field_separator.clone()
                }
            }
            ExpList(..) | NameList(..) | VarList(..) | ParList(..) | FuncName(..) | FuncNameSelf(..) => Some(false),
            _ => None,
        }
    }

    fn need_newlines(&self, cfg: &Config) -> bool {
        use Node::*;
        match self {
            Fields(_, _, _) => cfg.newline_format_table_field == Some(1),
            ExpList(..) => cfg.newline_format_exp_list.is_some() || cfg.newline_format_exp_list_first.is_some(),
            _ => false,
        }
    }
}

impl list::ListOfItems<Node> for Node {
    fn items(&self) -> Option<&Vec<(Loc, Node)>> {
        use Node::*;
        match self {
            StatementList(_, items) | VarSuffixList(_, items) | ElseIfThenVec(_, items) => Some(items),
            _ => None,
        }
    }

    fn need_newlines(&self, cfg: &Config) -> bool {
        use Node::*;
        match self {
            StatementList(..) => cfg.newline_format_statement.is_some(),
            ElseIfThenVec(..) => cfg.newline_format_if.is_some(),
            VarSuffixList(..) => cfg.newline_format_var_suffix.is_some(),
            _ => false,
        }
    }
}

impl Node {
    fn test_oneline_table_constructor_cfg(cfg: &Config) -> Option<Config> {
        if cfg.max_width.is_some()
            && cfg.newline_format_table_constructor.is_some()
            && cfg.enable_oneline_table_constructor == Some(true)
        {
            // disable IfNewLine within table constructor
            // one-line tables are forced to have no trailing separator
            let mut test_cfg = cfg.clone();
            test_cfg.newline_format_table_constructor = None;
            test_cfg.newline_format_table_field = None;
            test_cfg.write_trailing_field_separator = Some(false);

            return Some(test_cfg);
        }
        None
    }

    fn test_oneline_table_field(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.max_width.is_some() && cfg.newline_format_table_field.is_some() {
            return test_oneline_no_nl!(f, cfg, buf, state, self);
        }
        None
    }

    fn test_oneline_if(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.max_width.is_some() && cfg.enable_oneline_if == Some(true) && cfg.newline_format_if.is_some() {
            // disable IfNewLine within table constructor
            let mut test_cfg = cfg.clone();
            test_cfg.newline_format_if = None;

            return test_oneline_no_nl!(f, &test_cfg, buf, state, self);
        }
        None
    }

    fn test_oneline_function(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> Option<String> {
        if cfg.max_width.is_some()
            && cfg.newline_format_function.is_some()
            && ((state.function_nested_level == 0 && cfg.enable_oneline_top_level_function == Some(true))
                || (state.function_nested_level > 0 && cfg.enable_oneline_scoped_function == Some(true)))
        {
            // disable IfNewLine within function body
            let mut test_cfg = cfg.clone();
            test_cfg.newline_format_function = None;

            return test_oneline_no_nl!(f, &test_cfg, buf, state, self);
        }
        None
    }

    fn test_indent(
        &self, f: &mut String, cfg: &Config, buf: &str, state: &mut State, hint: CommentLocHint,
    ) -> Result<bool, std::fmt::Error> {
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;
        let cfg_write_list = list::cfg_write_list::<Node, CommentLocHint>;

        let ind = match self {
            Node::VarSuffixList(_, suffs) => match suffs.len() {
                0 => false,
                _ => {
                    let mut test_f = f.clone();
                    let mut test_state = state.clone();
                    cfg_write!(&mut test_f, cfg, buf, &mut test_state, hint)?;
                    cfg.indent_var_suffix == Some(true)
                        && (cfg.indent_one_line_var_suffix == Some(true)
                            || cfg_write_list(&mut test_f, cfg, buf, &mut test_state, self) == Ok(true))
                }
            },
            Node::ExpList(_, exprs) => match exprs.len() {
                0 => false,
                // ExpList cannot indent it's first item without this flag
                1 if cfg.newline_format_exp_list_first.is_none() => {
                    cfg.indent_exp_list == Some(true) && cfg.indent_one_line_exp_list == Some(true)
                }
                _ => {
                    let mut test_f = f.clone();
                    let mut test_state = state.clone();
                    cfg_write!(&mut test_f, cfg, buf, &mut test_state, hint)?;
                    cfg.indent_exp_list == Some(true)
                        && (cfg.indent_one_line_exp_list == Some(true)
                            || cfg_write_sep_list(&mut test_f, cfg, buf, &mut test_state, self) == Ok(true))
                }
            },
            _ => false,
        };

        Ok(ind)
    }
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut String, cfg: &Config, buf: &str, state: &mut State) -> std::fmt::Result {
        use Node::*;

        #[allow(non_snake_case)]
        let Hint = CommentLocHint;
        let cfg_write_list = list::cfg_write_list::<Node, CommentLocHint>;
        let cfg_write_sep_list = list::cfg_write_sep_list::<Node, CommentLocHint>;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                cfg_write!(f, cfg, buf, state, IncIndent(Some(tok.0)), l)?;

                let mut nl1 = cfg.newline_format_binary_op == Some(1);
                let mut nl2 = cfg.newline_format_binary_op == Some(2);
                if (nl1 || nl2) && cfg.enable_oneline_binary_op == Some(true) {
                    if test_oneline!(f, cfg, buf, state, Hint(&locs[0], " "), tok, Hint(&locs[1], " "), r).is_some() {
                        nl1 = false;
                        nl2 = false;
                    }
                }

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, IfNewLine(nl1, Hint(&locs[0], " ")), tok,
                           IfNewLine(nl2, Hint(&locs[1], " ")), r, DecIndent())
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, state, tok, Hint(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, state, "not", Hint(&locs[0], " "), r),

            Var(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, state, n1)?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], ""), n2, If(ind, &DecIndent()))
            }
            RoundBrackets(_, locs, r) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), r, Hint(&locs[1], ""), ")")
            }
            ArgsRoundBrackets(_, locs, r) => {
                cfg_write!(f, cfg, buf, state, "(")?;
                let ind = r.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], ""), r, If(ind, &DecIndent()),
                           Hint(&locs[1], ""), ")")
            }
            ArgsRoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")"),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),

            // literals
            Numeral(_, s) => write!(f, "{}", s),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),
            MultiLineStringLiteral(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            TableConstructor(_, locs, r, opts) => {
                let ind_ol = opts.is_first_child == Some(true);
                cfg_write!(f, cfg, buf, state, If(ind_ol, &IncIndent(None)))?;

                if let Some(test_cfg) = Node::test_oneline_table_constructor_cfg(cfg) {
                    match test_oneline_no_nl!(f, &test_cfg, buf, state, self) {
                        Some(line) => {
                            #[cfg_attr(rustfmt, rustfmt_skip)]
                            return cfg_write!(f, cfg, buf, state, Str(&line), If(ind_ol, &DecIndent()));
                        }
                        _ => {
                            if let Some(line) = test_oneline_no_nl_after_nl!(f, &test_cfg, buf, state, self) {
                                #[cfg_attr(rustfmt, rustfmt_skip)]
                                return cfg_write!(f, cfg, buf, state, IfNewLine(true, Hint(&Loc(0, 0), "")), Str(&line),
                                                  If(ind_ol, &DecIndent()));
                            }
                        }
                    }
                }
                cfg_write!(f, cfg, buf, state, If(ind_ol, &DecIndent()))?;

                let default_hint = String::new();
                let hint = cfg.hint_table_constructor.as_ref().unwrap_or(&default_hint);
                let mut nl = cfg.newline_format_table_constructor == Some(1);

                let ind = !(cfg.enable_oneline_iv_table == Some(true)
                    && opts.is_all_sequential == Some(true)
                    && opts.is_single_child != Some(false));
                if !ind && cfg.enable_oneline_table_constructor == Some(true) {
                    // do test_indent
                }

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), "{{", IncFuncLevel(), Hint(&locs[0], &hint), r)?;

                if cfg.max_width.is_some() && util::get_len_after_newline(f, cfg) >= cfg.max_width.unwrap() {
                    nl = true;
                }
                nl = nl && ind;

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &DecIndent()), DecFuncLevel(),
                           IfNewLine(nl, Hint(&locs[1], &hint)), "}}")
            }
            TableConstructorEmpty(_, locs) => {
                let default_hint = String::new();
                let hint = cfg.hint_table_constructor.as_ref().unwrap_or(&default_hint);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "{{", Hint(&locs[0], &hint), "}}")
            }
            Fields(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FieldNamedBracket(_, locs, e1, e2) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e1, Hint(&locs[1], ""), "]", Hint(&locs[2], " "),
                           "=", Hint(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, state, e1, Hint(&locs[0], " "), "=", Hint(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, state, e),

            TableIndex(_, locs, e) => {
                cfg_write!(f, cfg, buf, state, "[", Hint(&locs[0], ""), e, Hint(&locs[1], ""), "]")
            }
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, state, ".", Hint(&locs[0], ""), n),
            ExpList(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            NameList(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            VarList(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            stts @ StatementList(..) => {
                cfg_write_list(f, cfg, buf, state, stts)?;
                Ok(())
            }
            DoEnd(_, locs) => cfg_write!(f, cfg, buf, state, "do", Hint(&locs[0], " "), "end"),
            DoBEnd(_, locs, b) => {
                let nl = cfg.newline_format_do_end == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "do", IncIndent(None), IfNewLine(nl, Hint(&locs[0], " ")), b, DecIndent(),
                           IfNewLine(nl, Hint(&locs[1], " ")), "end")
            }
            VarsExprs(_, locs, n1, n2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, n1, Hint(&locs[0], " "), "=")?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[1], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[1], " "), n2, If(ind, &DecIndent()))
            }

            VarRoundSuffix(_, locs, n1, n2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), ")")?;

                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[0], "")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], ""), n2, If(ind, &DecIndent()))
            }
            suffs @ VarSuffixList(..) => {
                cfg_write_list(f, cfg, buf, state, suffs)?;
                Ok(())
            }
            FnMethodCall(_, locs, n1, n2) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, ":", Hint(&locs[0], ""), n1, Hint(&locs[1], ""), n2)
            }
            ParList(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], ""), n),
            FuncBody(_, locs) => {
                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", IfNewLine(nl, Hint(&locs[1], " ")), "end")
            }
            FuncBodyB(_, locs, n2) => {
                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", Hint(&locs[0], ""), ")", IncIndent(None), IncFuncLevel(),
                           IfNewLine(nl, Hint(&locs[1], " ")), n2, DecIndent(), DecFuncLevel(),
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            FuncPBody(_, locs, n1) => {
                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", IncIndent(None), Hint(&locs[0], ""), n1, Hint(&locs[1], ""),
                           DecIndent(), ")", IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            FuncPBodyB(_, locs, n1, n2) => {
                if let Some(line) = self.test_oneline_function(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_function == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "(", IncIndent(None), Hint(&locs[0], ""), n1, Hint(&locs[1], ""),
                           DecIndent(), ")", IncIndent(None), IncFuncLevel(), IfNewLine(nl, Hint(&locs[2], " ")), n2,
                           DecIndent(), DecFuncLevel(), IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            FuncName(..) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;
                Ok(())
            }
            FuncNameSelf(_, locs, _, n) => {
                cfg_write_sep_list(f, cfg, buf, state, self)?;

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, Hint(&locs[0], ""), ":", Hint(&locs[1], ""), n)
            }
            FuncDecl(_, locs, n1, n2) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "function", Hint(&locs[0], " "), n1, Hint(&locs[1], ""), n2)
            }
            LocalFuncDecl(_, locs, n1, n2) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), "function", Hint(&locs[1], " "), n1,
                           Hint(&locs[2], ""), n2)
            }

            LocalNames(_, locs, n) => cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n),
            LocalNamesExprs(_, locs, n1, n2) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "local", Hint(&locs[0], " "), n1, Hint(&locs[1], " "), "=")?;
                let ind = n2.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), n2,
                           If(ind, &DecIndent()))
            }

            // if
            IfThen(_, locs, e1) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            IfThenB(_, locs, e1, b1) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenElse(_, locs, e1) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenBElse(_, locs, e1, b1) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "else", IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenElseB(_, locs, e1, b2) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           Hint(&locs[2], " "), "else", IncIndent(None), IfNewLine(nl, Hint(&locs[3], " ")), b2,
                           DecIndent(), IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenBElseB(_, locs, e1, b1, b2) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "else", IncIndent(None),
                           IfNewLine(nl, Hint(&locs[4], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenElseIf(_, locs, e1, n) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            IfThenBElseIf(_, locs, e1, b1, n) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenElseIfElse(_, locs, e1, n) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "else",
                           IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            IfThenBElseIfElse(_, locs, e1, b1, n) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "else",
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenElseIfElseB(_, locs, e1, n, b2) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IfNewLine(nl, Hint(&locs[2], " ")), n, IfNewLine(nl, Hint(&locs[3], " ")), "else",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[4], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }
            IfThenBElseIfElseB(_, locs, e1, b1, n, b2) => {
                if let Some(line) = self.test_oneline_if(f, cfg, buf, state) {
                    return write!(f, "{}", line);
                }

                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "if", Hint(&locs[0], " "), e1, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b1, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), n, IfNewLine(nl, Hint(&locs[4], " ")), "else",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[5], " ")), b2, DecIndent(),
                           IfNewLine(nl, Hint(&locs[6], " ")), "end")
            }
            elems @ ElseIfThenVec(..) => {
                cfg_write_list(f, cfg, buf, state, elems)?;
                Ok(())
            }
            ElseIfThen(_, locs, e) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then")
            }
            ElseIfThenB(_, locs, e, b) => {
                let nl = cfg.newline_format_if == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "elseif", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "then",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), b, DecIndent())
            }

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, state, "::", Hint(&locs[0], ""), n, Hint(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, state, "goto", Hint(&locs[0], " "), n),
            WhileDo(_, locs, e) => {
                let nl = cfg.newline_format_while == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           IfNewLine(nl, Hint(&locs[2], " ")), "end")
            }
            WhileDoB(_, locs, e, n) => {
                let nl = cfg.newline_format_while == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "while", Hint(&locs[0], " "), e, Hint(&locs[1], " "), "do",
                           IncIndent(None), IfNewLine(nl, Hint(&locs[2], " ")), n, DecIndent(),
                           IfNewLine(nl, Hint(&locs[3], " ")), "end")
            }
            RepeatUntil(_, locs, e) => {
                let nl = cfg.newline_format_repeat_until == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "repeat", IfNewLine(nl, Hint(&locs[0], " ")), "until",
                           Hint(&locs[1], " "), e)
            }
            RepeatBUntil(_, locs, b, e) => {
                let nl = cfg.newline_format_repeat_until == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "repeat", IncIndent(None), IfNewLine(nl, Hint(&locs[0], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[1], " ")), "until", Hint(&locs[2], " "), e)
            }

            ForInt(_, locs, n, e1, e2) =>
            {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], " "), "do", Hint(&locs[6], " "), "end")
            }
            ForIntB(_, locs, n, e1, e2, b) => {
                let nl = cfg.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2,
                           Hint(&locs[5], " "), "do", IncIndent(None), IfNewLine(nl, Hint(&locs[6], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[7], " ")), "end")
            }
            ForIntStep(_, locs, n, e1, e2, e3) => {
                let nl = cfg.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", IfNewLine(nl, Hint(&locs[8], " ")),
                           "end")
            }
            ForIntStepB(_, locs, n, e1, e2, e3, b) => {
                let nl = cfg.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "=",
                           Hint(&locs[2], " "), e1, Hint(&locs[3], ""), ",", Hint(&locs[4], " "), e2, Hint(&locs[5], ""),
                           ",", Hint(&locs[6], " "), e3, Hint(&locs[7], " "), "do", IncIndent(None),
                           IfNewLine(nl, Hint(&locs[8], " ")), b, DecIndent(), IfNewLine(nl, Hint(&locs[9], " ")), "end")
            }
            ForRange(_, locs, n, e) => {
                let nl = cfg.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in")?;

                let ind = e.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), e, If(ind, &DecIndent()),
                           Hint(&locs[3], " "), "do", IfNewLine(nl, Hint(&locs[4], " ")), "end")
            }
            ForRangeB(_, locs, n, e, b) => {
                let nl = cfg.newline_format_for == Some(1);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "for", Hint(&locs[0], " "), n, Hint(&locs[1], " "), "in")?;
                let ind = e.test_indent(f, cfg, buf, state, Hint(&locs[2], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[2], " "), e, If(ind, &DecIndent()),
                           Hint(&locs[3], " "), "do", IncIndent(None), IfNewLine(nl, Hint(&locs[4], " ")), b,
                           DecIndent(), IfNewLine(nl, Hint(&locs[5], " ")), "end")
            }

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => {
                cfg_write!(f, cfg, buf, state, "return")?;
                let ind = n.test_indent(f, cfg, buf, state, Hint(&locs[0], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], " "), n, If(ind, &DecIndent()))
            }
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, state, "return", Hint(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => {
                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, "return")?;

                let ind = n.test_indent(f, cfg, buf, state, Hint(&locs[0], " ")) == Ok(true);

                #[cfg_attr(rustfmt, rustfmt_skip)]
                cfg_write!(f, cfg, buf, state, If(ind, &IncIndent(None)), Hint(&locs[0], " "), n, Hint(&locs[1], ""),
                           If(ind, &DecIndent()), ";"
                )
            }
            StatsRetStat(_, locs, n1, n2) => {
                let nl = cfg.newline_format_statement.is_some();
                cfg_write!(f, cfg, buf, state, n1, IfNewLine(nl, Hint(&locs[0], " ")), n2)
            }
            Chunk(locl, n, locr) => {
                let nl = cfg.write_newline_at_eof == Some(true);
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, IfNewLine(nl, Hint(&locr, "")))
            }
            SheBangChunk(locl, n, locm, b, locr) => {
                let nl = cfg.write_newline_at_eof == Some(true);
                cfg_write!(f, cfg, buf, state, Hint(&locl, ""), n, Hint(&locm, ""), b, IfNewLine(nl, Hint(&locr, "")))
            }

            Semicolon(_) => write!(f, ";"),
            SheBang(_, s) => write!(f, "{}\n", s),
        }
    }
}
