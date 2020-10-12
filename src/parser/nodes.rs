use std::fmt;

use crate::config::{Config, ConfiguredWrite};
use crate::{cfg_write, cfg_write_helper};

#[derive(Debug)]
pub struct Loc(pub usize, pub usize);

#[derive(Debug)]
pub struct Str(pub &'static str);

impl ConfiguredWrite for Str {
    fn configured_write(&self, f: &mut dyn fmt::Write, _cfg: &Config, _buf: &str) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct LocOpt<'a>(pub &'a Loc, pub &'static str);

impl ConfiguredWrite for LocOpt<'_> {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        if cfg.keep_comments {
            let trimmed = &buf[self.0.0..self.0.1].trim_matches(' ');
            if trimmed.len() > 0 {
                let prefix = match trimmed.chars().next().unwrap() {
                    '-' => " ",
                    _ => "",
                };
                let suffix = match trimmed.chars().last().unwrap() {
                    '\n' => "",
                    _ => " ",
                };

                write!(f, "{}{}{}", prefix, trimmed, suffix)?;
            } else {
                write!(f, "{}", self.1)?;
            }
        } else {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Node {
    BinaryOp(Loc, [Loc; 2], Str, Box<Node>, Box<Node>),
    UnaryOp(Loc, [Loc; 1], Str, Box<Node>),
    UnaryNot(Loc, [Loc; 1], Box<Node>),

    Var(Loc, [Loc; 1], Box<Node>, Box<Node>),
    RoundBrackets(Loc, [Loc; 2], Box<Node>),
    RoundBracketsEmpty(Loc, [Loc; 1]),

    Nil(Loc),
    False(Loc),
    True(Loc),
    VarArg(Loc),
    Break(Loc),
    Numeral(Loc, String),
    NormalStringLiteral(Loc, String),
    CharStringLiteral(Loc, String),
    MultilineStringLiteral(Loc, usize, String),

    TableConstructor(Loc, [Loc; 2], Box<Node>),
    TableConstructorEmpty(Loc, [Loc; 1]),
    Fields(Loc, Vec<(Loc, Node, Loc)>),
    FieldNamedBracket(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FieldNamed(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FieldSequential(Loc, Box<Node>),

    TableIndex(Loc, [Loc; 2], Box<Node>),
    TableMember(Loc, [Loc; 1], Box<Node>),
    ExpList(Loc, Vec<(Loc, Node, Loc)>),
    NameList(Loc, Vec<(Loc, Node, Loc)>),
    ParList(Loc, Vec<(Loc, Node, Loc)>),
    VarList(Loc, Vec<(Loc, Node, Loc)>),
    VarRoundSuffix(Loc, [Loc; 3], Box<Node>, Box<Node>),
    VarSuffixList(Loc, Vec<(Loc, Node)>),
    FnMethodCall(Loc, [Loc; 2], Box<Node>, Box<Node>),
    FunctionDef(Loc, [Loc; 1], Box<Node>),
    FuncBody(Loc, [Loc; 4], Box<Node>, Box<Node>),
    FuncName(Loc, Vec<(Loc, Node, Loc)>),
    FuncNameSelf(Loc, [Loc; 2], Vec<(Loc, Node, Loc)>, Box<Node>),
    FuncDecl(Loc, Box<Node>, Box<Node>),

    StatementList(Loc, Vec<Node>),
    DoEnd(Loc, Box<Node>),
    VarsExprs(Loc, Box<Node>, Box<Node>),
    Name(Loc, String),
    Label(Loc, [Loc; 2], Box<Node>),
    GoTo(Loc, [Loc; 1], Box<Node>),
    While(Loc, Box<Node>, Box<Node>),
    Repeat(Loc, Box<Node>, Box<Node>),
    ForRange(Loc, Box<Node>, Box<Node>, Box<Node>),
    ForInt(Loc, Box<Node>, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    LocalNamesExprs(Loc, Box<Node>, Box<Node>),
    IfThenElse(Loc, Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    ElseIfThenVec(Loc, Vec<Node>),
    ElseIfThen(Loc, Box<Node>, Box<Node>),

    RetStatNone(Loc),
    RetStatExpr(Loc, [Loc; 1], Box<Node>),
    RetStatNoneComma(Loc, [Loc; 1]),
    RetStatExprComma(Loc, [Loc; 2], Box<Node>),
    StatsRetStat(Loc, [Loc; 1], Box<Node>, Box<Node>),

    Empty(Loc),
}

fn cfg_write_node_vec(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<Node>,
    sep: &str,
    ws: &str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        for elem in &elems[0..elems.len() - 1] {
            if let Node::Empty(_) = *elem {
                continue;
            }
            cfg_write!(f, cfg, buf, elem)?;
            write!(f, "{}{}", sep, ws)?;
        }
        cfg_write!(f, cfg, buf, (elems.last().unwrap()))?;
    }
    Ok(())
}

fn cfg_write_node_vec_locs_sep(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node, Loc)>,
    sep: &str,
    ws: &'static str,
) -> Result<(), core::fmt::Error> {
    if !elems.is_empty() {
        let first = &elems[0];
        cfg_write!(f, cfg, buf, LocOpt(&first.0, ""), first.1, LocOpt(&first.2, ""))?;

        for elem in &elems[1..elems.len()] {
            if let Node::Empty(_) = elem.1 {
                continue;
            }
            write!(f, "{}", sep)?;
            cfg_write!(f, cfg, buf, LocOpt(&elem.0, ws), elem.1, LocOpt(&elem.2, ""))?;
        }
    }
    Ok(())
}

fn cfg_write_node_vec_locs(
    f: &mut dyn fmt::Write,
    cfg: &Config,
    buf: &str,
    elems: &Vec<(Loc, Node)>,
    ws: &'static str,
) -> Result<(), core::fmt::Error> {
    for elem in elems {
        if let Node::Empty(_) = elem.1 {
            continue;
        }
        cfg_write!(f, cfg, buf, LocOpt(&elem.0, ws), elem.1)?;
    }
    Ok(())
}

impl ConfiguredWrite for Node {
    fn configured_write(&self, f: &mut dyn fmt::Write, cfg: &Config, buf: &str) -> fmt::Result {
        use Node::*;

        match self {
            BinaryOp(_, locs, tok, l, r) => {
                cfg_write!(f, cfg, buf, l, LocOpt(&locs[0], " "), tok, LocOpt(&locs[1], " "), r)
            }
            UnaryOp(_, locs, tok, r) => cfg_write!(f, cfg, buf, tok, LocOpt(&locs[0], ""), r),
            UnaryNot(_, locs, r) => cfg_write!(f, cfg, buf, "not", LocOpt(&locs[0], " "), r),

            Var(_, locs, n1, n2) => cfg_write!(f, cfg, buf, n1, LocOpt(&locs[0], ""), n2),
            RoundBrackets(_, locs, r) => cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), r, LocOpt(&locs[1], ""), ")"),
            RoundBracketsEmpty(_, locs) => cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), ")"),

            Nil(_) => write!(f, "nil"),
            False(_) => write!(f, "false"),
            True(_) => write!(f, "true"),
            VarArg(_) => write!(f, "..."),
            Break(_) => write!(f, "break"),

            // literals
            Numeral(_, s) => write!(f, "{}", s),
            NormalStringLiteral(_, s) => write!(f, "\"{}\"", s),
            CharStringLiteral(_, s) => write!(f, "'{}'", s),
            MultilineStringLiteral(_, level, s) => {
                let level_str = (0..*level).map(|_| "=").collect::<String>();
                write!(f, "[{}[{}]{}]", level_str, s, level_str)
            }

            TableConstructor(_, locs, r) => {
                cfg_write!(f, cfg, buf, "{{", LocOpt(&locs[0], " "), r, LocOpt(&locs[1], " "), "}}")
            }
            TableConstructorEmpty(_, locs) => cfg_write!(f, cfg, buf, "{{", LocOpt(&locs[0], ""), "}}"),
            Fields(_, fields) => cfg_write_node_vec_locs_sep(f, cfg, buf, fields, ",", " "),
            FieldNamedBracket(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, "[", LocOpt(&locs[0], ""), e1, LocOpt(&locs[1], ""), "]", LocOpt(&locs[2], " "),
                           "=", LocOpt(&locs[3], " "), e2)
            }
            FieldNamed(_, locs, e1, e2) => {
                cfg_write!(f, cfg, buf, e1, LocOpt(&locs[0], " "), "=", LocOpt(&locs[1], " "), e2)
            }
            FieldSequential(_, e) => cfg_write!(f, cfg, buf, e),

            TableIndex(_, locs, e) => cfg_write!(f, cfg, buf, "[", LocOpt(&locs[0], ""), e, LocOpt(&locs[1], ""), "]"),
            TableMember(_, locs, n) => cfg_write!(f, cfg, buf, ".", LocOpt(&locs[0], ""), n),
            ExpList(_, exps) => cfg_write_node_vec_locs_sep(f, cfg, buf, exps, ",", " "),
            NameList(_, names) => cfg_write_node_vec_locs_sep(f, cfg, buf, names, ",", " "),
            VarList(_, vars) => cfg_write_node_vec_locs_sep(f, cfg, buf, vars, ",", " "),
            StatementList(_, stts) => cfg_write_node_vec(f, cfg, buf, stts, ";", " "),
            DoEnd(_, n) => cfg_write!(f, cfg, buf, "do ", n, " end"),
            VarsExprs(_, n1, n2) => cfg_write!(f, cfg, buf, n1, " = ", n2),

            VarRoundSuffix(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")", LocOpt(&locs[2], ""), n2)
            }
            VarSuffixList(_, suffs) => cfg_write_node_vec_locs(f, cfg, buf, suffs, ""),
            FnMethodCall(_, locs, n1, n2) => {
                cfg_write!(f, cfg, buf, ":", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), n2)
            }
            ParList(_, pars) => cfg_write_node_vec_locs_sep(f, cfg, buf, pars, ",", " "),
            FunctionDef(_, locs, n) => cfg_write!(f, cfg, buf, "function", LocOpt(&locs[0], ""), n),
            FuncBody(_, locs, n1, n2) => match &**n2 {
                Node::StatementList(_, v2) if v2.is_empty() => {
                    cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")",
                               LocOpt(&locs[2], " "), "end")
                }
                _ => cfg_write!(f, cfg, buf, "(", LocOpt(&locs[0], ""), n1, LocOpt(&locs[1], ""), ")",
                                LocOpt(&locs[2], " "), n2, LocOpt(&locs[3], " "), "end"),
            },
            FuncName(_, names) => cfg_write_node_vec_locs_sep(f, cfg, buf, names, ".", ""),
            FuncNameSelf(_, locs, names, n) => {
                cfg_write_node_vec_locs_sep(f, cfg, buf, names, ".", "")?;
                cfg_write!(f, cfg, buf, LocOpt(&locs[0], ""), ":", LocOpt(&locs[1], ""), n)
            }
            FuncDecl(_, n1, n2) => cfg_write!(f, cfg, buf, "function ", n1, n2),
            LocalNamesExprs(_, n1, n2) => match &**n2 {
                Node::Empty(_) => cfg_write!(f, cfg, buf, "local ", n1),
                _ => cfg_write!(f, cfg, buf, "local ", n1, " = ", n2),
            },
            IfThenElse(_, e1, b1, n, b2) => match (&**n, &**b2) {
                (Node::ElseIfThenVec(_, v), Node::Empty(_)) if v.is_empty() => {
                    cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " end")
                }
                (Node::ElseIfThenVec(_, v), _) if v.is_empty() => {
                    cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " else ", b2, " end")
                }
                (_, Node::Empty(_)) => cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " ", n, " end"),
                _ => cfg_write!(f, cfg, buf, "if ", e1, " then ", b1, " ", n, " else ", b2, " end"),
            },
            ElseIfThenVec(_, elems) => cfg_write_node_vec(f, cfg, buf, elems, "", " "),
            ElseIfThen(_, e, n) => cfg_write!(f, cfg, buf, "elseif ", e, " then ", n),

            Name(_, s) => write!(f, "{}", s),
            Label(_, locs, n) => cfg_write!(f, cfg, buf, "::", LocOpt(&locs[0], ""), n, LocOpt(&locs[1], ""), "::"),
            GoTo(_, locs, n) => cfg_write!(f, cfg, buf, "goto", LocOpt(&locs[0], " "), n),
            While(_, e, n) => cfg_write!(f, cfg, buf, "while ", e, " do ", n, " end"),
            Repeat(_, n, e) => cfg_write!(f, cfg, buf, "repeat ", n, " until ", e),
            ForRange(_, n, e, b) => cfg_write!(f, cfg, buf, "for ", n, " in ", e, " do ", b, " end"),
            ForInt(_, n, e1, e2, e3, b) => match &**e3 {
                Node::Empty(_) => cfg_write!(f, cfg, buf, "for ", n, " = ", e1, ", ", e2, " do ", b, " end"),
                _ => cfg_write!(f, cfg, buf, "for ", n, " = ", e1, ", ", e2, ", ", e3, " do ", b, " end"),
            },

            RetStatNone(_) => write!(f, "return"),
            RetStatExpr(_, locs, n) => cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], " "), n),
            RetStatNoneComma(_, locs) => cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], ""), ";"),
            RetStatExprComma(_, locs, n) => cfg_write!(f, cfg, buf, "return", LocOpt(&locs[0], " "), n, LocOpt(&locs[1], ""), ";"),
            StatsRetStat(_, locs, n1, n2) => match &**n1 {
                Node::StatementList(_, ref v) if v.is_empty() => cfg_write!(f, cfg, buf, LocOpt(&locs[0], ""), n2),
                _ => cfg_write!(f, cfg, buf, n1, LocOpt(&locs[0], " "), n2),
            },

            Empty(_) => Ok(()),
        }
    }
}
