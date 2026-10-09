//! Eigener, sicherer Ausdrucks-Parser.
//!
//! Der Parser führt niemals Code aus, sondern zerlegt die Eingabe in Tokens
//! und wertet sie mit einem rekursiven Abstiegsparser aus. Eingabelänge und
//! Verschachtelungstiefe sind begrenzt, damit bösartige oder versehentlich
//! riesige Eingaben weder den Stack sprengen noch das Programm blockieren.
//!
//! Grammatik (von niedriger zu hoher Bindung):
//!
//! ```text
//! ausdruck  = term { ("+" | "-") term }
//! term      = vorzeichen { ("*" | "/" | "%" | implizit) vorzeichen }
//! vorzeichen= ("+" | "-") vorzeichen | potenz
//! potenz    = postfix [ "^" vorzeichen ]          (rechtsassoziativ)
//! postfix   = primär { "!" }
//! primär    = zahl | konstante | funktion "(" ausdruck ")" | "(" ausdruck ")"
//! ```

use std::fmt;

/// Maximale Länge der Eingabe in Zeichen.
pub const MAX_INPUT_LEN: usize = 1_000;
/// Maximale Verschachtelungstiefe (Klammern, Vorzeichen, Potenzen).
pub const MAX_DEPTH: usize = 100;
/// Größte Zahl, deren Fakultät noch berechnet wird (171! ist bereits unendlich).
const MAX_FACTORIAL: f64 = 170.0;

/// Winkelmaß für trigonometrische Funktionen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AngleMode {
    Deg,
    Rad,
}

/// Fehler, die beim Zerlegen oder Auswerten auftreten können.
#[derive(Clone, Debug, PartialEq)]
pub enum CalcError {
    Empty,
    TooLong,
    TooDeep,
    UnexpectedChar(char),
    UnexpectedEnd,
    UnexpectedToken(String),
    UnknownIdentifier(String),
    DivisionByZero,
    Domain(&'static str),
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "Keine Eingabe"),
            Self::TooLong => write!(f, "Eingabe zu lang (max. {MAX_INPUT_LEN} Zeichen)"),
            Self::TooDeep => write!(f, "Ausdruck zu tief verschachtelt"),
            Self::UnexpectedChar(c) => write!(f, "Ungültiges Zeichen: '{c}'"),
            Self::UnexpectedEnd => write!(f, "Ausdruck unvollständig"),
            Self::UnexpectedToken(t) => write!(f, "Unerwartet: '{t}'"),
            Self::UnknownIdentifier(s) => write!(f, "Unbekannt: '{s}'"),
            Self::DivisionByZero => write!(f, "Division durch null"),
            Self::Domain(msg) => write!(f, "Mathematischer Fehler: {msg}"),
            Self::Overflow => write!(f, "Ergebnis zu groß"),
        }
    }
}

impl std::error::Error for CalcError {}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Func {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Sqrt,
    Cbrt,
    Ln,
    Log,
    Log2,
    Exp,
    Abs,
    Floor,
    Ceil,
    Round,
}

impl Func {
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "sin" => Self::Sin,
            "cos" => Self::Cos,
            "tan" => Self::Tan,
            "asin" => Self::Asin,
            "acos" => Self::Acos,
            "atan" => Self::Atan,
            "sinh" => Self::Sinh,
            "cosh" => Self::Cosh,
            "tanh" => Self::Tanh,
            "sqrt" => Self::Sqrt,
            "cbrt" => Self::Cbrt,
            "ln" => Self::Ln,
            "log" => Self::Log,
            "log2" => Self::Log2,
            "exp" => Self::Exp,
            "abs" => Self::Abs,
            "floor" => Self::Floor,
            "ceil" => Self::Ceil,
            "round" => Self::Round,
            _ => return None,
        })
    }

    fn apply(self, x: f64, mode: AngleMode) -> Result<f64, CalcError> {
        let to_rad = |v: f64| match mode {
            AngleMode::Deg => v.to_radians(),
            AngleMode::Rad => v,
        };
        let from_rad = |v: f64| match mode {
            AngleMode::Deg => v.to_degrees(),
            AngleMode::Rad => v,
        };
        let r = match self {
            Self::Sin => clean_trig(to_rad(x).sin()),
            Self::Cos => clean_trig(to_rad(x).cos()),
            Self::Tan => {
                let c = clean_trig(to_rad(x).cos());
                if c == 0.0 {
                    return Err(CalcError::Domain("tan ist hier nicht definiert"));
                }
                clean_trig(to_rad(x).sin()) / c
            }
            Self::Asin | Self::Acos if !(-1.0..=1.0).contains(&x) => {
                return Err(CalcError::Domain("Argument muss zwischen -1 und 1 liegen"));
            }
            Self::Asin => from_rad(x.asin()),
            Self::Acos => from_rad(x.acos()),
            Self::Atan => from_rad(x.atan()),
            Self::Sinh => x.sinh(),
            Self::Cosh => x.cosh(),
            Self::Tanh => x.tanh(),
            Self::Sqrt if x < 0.0 => {
                return Err(CalcError::Domain("Wurzel aus negativer Zahl"));
            }
            Self::Sqrt => x.sqrt(),
            Self::Cbrt => x.cbrt(),
            Self::Ln | Self::Log | Self::Log2 if x <= 0.0 => {
                return Err(CalcError::Domain("Logarithmus nur für positive Zahlen"));
            }
            Self::Ln => x.ln(),
            Self::Log => x.log10(),
            Self::Log2 => x.log2(),
            Self::Exp => x.exp(),
            Self::Abs => x.abs(),
            Self::Floor => x.floor(),
            Self::Ceil => x.ceil(),
            Self::Round => x.round(),
        };
        Ok(r)
    }
}

/// Rundet winzige Rundungsfehler bei Winkelfunktionen weg (z. B. sin(180°)).
fn clean_trig(v: f64) -> f64 {
    if v.abs() < 1e-15 { 0.0 } else { v }
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Num(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Bang,
    LParen,
    RParen,
}

impl Token {
    fn describe(&self) -> String {
        match self {
            Self::Num(n) => n.to_string(),
            Self::Ident(s) => s.clone(),
            Self::Plus => "+".into(),
            Self::Minus => "-".into(),
            Self::Star => "*".into(),
            Self::Slash => "/".into(),
            Self::Percent => "%".into(),
            Self::Caret => "^".into(),
            Self::Bang => "!".into(),
            Self::LParen => "(".into(),
            Self::RParen => ")".into(),
        }
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let chars: Vec<char> = input.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while let Some(&c) = chars.get(i) {
        match c {
            c if c.is_whitespace() => i += 1,
            '0'..='9' | '.' | ',' => {
                let start = i;
                let mut text = String::new();
                let mut seen_dot = false;
                while let Some(&d) = chars.get(i) {
                    match d {
                        '0'..='9' => text.push(d),
                        // Komma und Punkt sind beide als Dezimaltrennzeichen erlaubt.
                        '.' | ',' if !seen_dot => {
                            seen_dot = true;
                            text.push('.');
                        }
                        _ => break,
                    }
                    i += 1;
                }
                // Wissenschaftliche Schreibweise, z. B. 1.5e-3
                if matches!(chars.get(i), Some('e' | 'E')) {
                    let mut j = i + 1;
                    let mut exp = String::from("e");
                    if let Some(&s @ ('+' | '-')) = chars.get(j) {
                        exp.push(s);
                        j += 1;
                    }
                    let digits_start = j;
                    while let Some(&d @ '0'..='9') = chars.get(j) {
                        exp.push(d);
                        j += 1;
                    }
                    if j > digits_start {
                        text.push_str(&exp);
                        i = j;
                    }
                }
                if text == "." {
                    return Err(CalcError::UnexpectedChar(chars[start]));
                }
                let n: f64 = text
                    .parse()
                    .map_err(|_| CalcError::UnexpectedChar(chars[start]))?;
                tokens.push(Token::Num(n));
            }
            c if c.is_alphabetic() => {
                let mut name = String::new();
                while let Some(&d) = chars.get(i) {
                    if d.is_alphanumeric() {
                        name.extend(d.to_lowercase());
                        i += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(name));
            }
            'π' => {
                tokens.push(Token::Ident("pi".into()));
                i += 1;
            }
            '√' => {
                tokens.push(Token::Ident("sqrt".into()));
                i += 1;
            }
            _ => {
                let t = match c {
                    '+' => Token::Plus,
                    '-' | '−' => Token::Minus,
                    '*' | '×' | '·' => Token::Star,
                    '/' | '÷' | ':' => Token::Slash,
                    '%' => Token::Percent,
                    '^' => Token::Caret,
                    '!' => Token::Bang,
                    '(' | '[' => Token::LParen,
                    ')' | ']' => Token::RParen,
                    other => return Err(CalcError::UnexpectedChar(other)),
                };
                tokens.push(t);
                i += 1;
            }
        }
    }
    Ok(tokens)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    depth: usize,
    mode: AngleMode,
    ans: f64,
}

impl Parser<'_> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn enter(&mut self) -> Result<(), CalcError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            Err(CalcError::TooDeep)
        } else {
            Ok(())
        }
    }

    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    fn expression(&mut self) -> Result<f64, CalcError> {
        self.enter()?;
        let mut value = self.term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.pos += 1;
                    value += self.term()?;
                }
                Some(Token::Minus) => {
                    self.pos += 1;
                    value -= self.term()?;
                }
                _ => break,
            }
        }
        self.leave();
        Ok(value)
    }

    fn term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.unary()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.pos += 1;
                    value *= self.unary()?;
                }
                Some(Token::Slash) => {
                    self.pos += 1;
                    let rhs = self.unary()?;
                    if rhs == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    value /= rhs;
                }
                Some(Token::Percent) => {
                    // "a % b" ist der Rest der Division (Modulo).
                    self.pos += 1;
                    let rhs = self.unary()?;
                    if rhs == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }
                    value %= rhs;
                }
                // Implizite Multiplikation: 2pi, 3(4+1), (1+2)(3+4), 2sin(30)
                Some(Token::Num(_) | Token::Ident(_) | Token::LParen) => {
                    value *= self.power_level()?;
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn unary(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.pos += 1;
                self.enter()?;
                let v = -self.unary()?;
                self.leave();
                Ok(v)
            }
            Some(Token::Plus) => {
                self.pos += 1;
                self.enter()?;
                let v = self.unary()?;
                self.leave();
                Ok(v)
            }
            _ => self.power_level(),
        }
    }

    fn power_level(&mut self) -> Result<f64, CalcError> {
        let base = self.postfix()?;
        if matches!(self.peek(), Some(Token::Caret)) {
            self.pos += 1;
            self.enter()?;
            // Rechtsassoziativ: 2^3^2 = 2^(3^2); -2^2 = -(2^2), 2^-1 erlaubt.
            let exponent = self.unary()?;
            self.leave();
            let r = base.powf(exponent);
            if r.is_nan() {
                return Err(CalcError::Domain("Potenz ist nicht reell"));
            }
            return Ok(r);
        }
        Ok(base)
    }

    fn postfix(&mut self) -> Result<f64, CalcError> {
        let mut value = self.primary()?;
        while matches!(self.peek(), Some(Token::Bang)) {
            self.pos += 1;
            value = factorial(value)?;
        }
        Ok(value)
    }

    fn primary(&mut self) -> Result<f64, CalcError> {
        match self.next() {
            Some(Token::Num(n)) => Ok(n),
            Some(Token::LParen) => self.parenthesized_rest(),
            Some(Token::Ident(name)) => match name.as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                "ans" => Ok(self.ans),
                _ => {
                    let func = Func::from_name(&name).ok_or(CalcError::UnknownIdentifier(name))?;
                    // Funktionsaufruf: Klammer ist Pflicht, z. B. sin(30)
                    match self.next() {
                        Some(Token::LParen) => {
                            let arg = self.parenthesized_rest()?;
                            func.apply(arg, self.mode)
                        }
                        Some(t) => Err(CalcError::UnexpectedToken(t.describe())),
                        None => Err(CalcError::UnexpectedEnd),
                    }
                }
            },
            Some(t) => Err(CalcError::UnexpectedToken(t.describe())),
            None => Err(CalcError::UnexpectedEnd),
        }
    }

    /// Wertet den Inhalt nach einer öffnenden Klammer aus. Eine fehlende
    /// schließende Klammer am Ende der Eingabe wird toleriert, wie bei
    /// vielen Taschenrechnern üblich.
    fn parenthesized_rest(&mut self) -> Result<f64, CalcError> {
        let v = self.expression()?;
        match self.next() {
            Some(Token::RParen) | None => Ok(v),
            Some(t) => Err(CalcError::UnexpectedToken(t.describe())),
        }
    }
}

fn factorial(x: f64) -> Result<f64, CalcError> {
    if x < 0.0 || x.fract() != 0.0 {
        return Err(CalcError::Domain("Fakultät nur für natürliche Zahlen"));
    }
    if x > MAX_FACTORIAL {
        return Err(CalcError::Overflow);
    }
    // x ist hier eine ganze Zahl zwischen 0 und 170, die Schleife ist also kurz.
    let mut r = 1.0;
    let mut k = 2.0;
    while k <= x {
        r *= k;
        k += 1.0;
    }
    Ok(r)
}

/// Wertet einen mathematischen Ausdruck aus.
///
/// `ans` ist das vorherige Ergebnis und kann in der Eingabe mit `ans`
/// verwendet werden.
pub fn evaluate(input: &str, mode: AngleMode, ans: f64) -> Result<f64, CalcError> {
    if input.chars().count() > MAX_INPUT_LEN {
        return Err(CalcError::TooLong);
    }
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err(CalcError::Empty);
    }
    let mut parser = Parser {
        tokens: &tokens,
        pos: 0,
        depth: 0,
        mode,
        ans,
    };
    let value = parser.expression()?;
    if let Some(t) = parser.peek() {
        return Err(match t {
            Token::RParen => CalcError::UnexpectedToken(")".into()),
            other => CalcError::UnexpectedToken(other.describe()),
        });
    }
    if value.is_nan() {
        return Err(CalcError::Domain("Ergebnis ist keine Zahl"));
    }
    if value.is_infinite() {
        return Err(CalcError::Overflow);
    }
    Ok(value)
}

/// Formatiert ein Ergebnis gut lesbar (ohne störende Rundungsreste).
pub fn format_number(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let abs = v.abs();
    if !(1e-9..1e15).contains(&abs) {
        // Wissenschaftliche Schreibweise für sehr große oder kleine Zahlen.
        let s = format!("{v:.10e}");
        if let Some((mantissa, exp)) = s.split_once('e') {
            let m = trim_zeros(mantissa);
            return format!("{m}e{exp}");
        }
        return s;
    }
    // 12 signifikante Nachkommastellen reichen und verbergen f64-Rauschen.
    let decimals = (11 - abs.log10().floor() as i32).clamp(0, 15) as usize;
    trim_zeros(&format!("{v:.decimals$}"))
}

fn trim_zeros(s: &str) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(s: &str) -> Result<f64, CalcError> {
        evaluate(s, AngleMode::Deg, 0.0)
    }

    fn approx(s: &str, expected: f64) {
        let v = eval(s).unwrap_or_else(|e| panic!("{s}: {e}"));
        assert!(
            (v - expected).abs() < 1e-9 * expected.abs().max(1.0),
            "{s} = {v}, erwartet {expected}"
        );
    }

    #[test]
    fn grundrechenarten() {
        approx("1+2*3", 7.0);
        approx("(1+2)*3", 9.0);
        approx("10/4", 2.5);
        approx("2-3-4", -5.0);
        approx("7 % 3", 1.0);
        approx("1,5 + 1.5", 3.0);
        approx("2 × 3 ÷ 4", 1.5);
    }

    #[test]
    fn potenzen_und_vorzeichen() {
        approx("2^10", 1024.0);
        approx("2^3^2", 512.0);
        approx("-2^2", -4.0);
        approx("(-2)^2", 4.0);
        approx("2^-1", 0.5);
        approx("--3", 3.0);
        approx("1.5e3", 1500.0);
        approx("2e-3", 0.002);
    }

    #[test]
    fn funktionen_und_konstanten() {
        approx("sin(30)", 0.5);
        approx("cos(60)", 0.5);
        approx("tan(45)", 1.0);
        approx("asin(1)", 90.0);
        approx("sqrt(16)", 4.0);
        approx("√(16)", 4.0);
        approx("log(1000)", 3.0);
        approx("ln(e)", 1.0);
        approx("2pi", 2.0 * std::f64::consts::PI);
        approx("2(3+4)", 14.0);
        approx("5!", 120.0);
        approx("3!!", 720.0);
        approx("abs(-3)", 3.0);
        approx("SIN(90)", 1.0);
        assert_eq!(eval("sin(180)"), Ok(0.0));
    }

    #[test]
    fn radiant() {
        let v = evaluate("sin(pi/2)", AngleMode::Rad, 0.0);
        assert_eq!(v, Ok(1.0));
    }

    #[test]
    fn ans() {
        assert_eq!(evaluate("ans*2", AngleMode::Deg, 21.0), Ok(42.0));
    }

    #[test]
    fn offene_klammer_wird_toleriert() {
        approx("sqrt(9", 3.0);
        approx("(1+2", 3.0);
    }

    #[test]
    fn fehler() {
        assert_eq!(eval(""), Err(CalcError::Empty));
        assert_eq!(eval("1/0"), Err(CalcError::DivisionByZero));
        assert!(matches!(eval("sqrt(-1)"), Err(CalcError::Domain(_))));
        assert!(matches!(eval("ln(0)"), Err(CalcError::Domain(_))));
        assert!(matches!(eval("tan(90)"), Err(CalcError::Domain(_))));
        assert!(matches!(eval("2.5!"), Err(CalcError::Domain(_))));
        assert_eq!(eval("171!"), Err(CalcError::Overflow));
        assert_eq!(eval("10^400"), Err(CalcError::Overflow));
        assert!(matches!(
            eval("foo(1)"),
            Err(CalcError::UnknownIdentifier(_))
        ));
        assert!(matches!(eval("1+"), Err(CalcError::UnexpectedEnd)));
        assert!(matches!(eval("1)"), Err(CalcError::UnexpectedToken(_))));
        assert!(matches!(eval("1 $ 2"), Err(CalcError::UnexpectedChar('$'))));
        assert!(matches!(eval("."), Err(CalcError::UnexpectedChar('.'))));
        assert!(matches!(eval("sin 30"), Err(CalcError::UnexpectedToken(_))));
    }

    #[test]
    fn schutz_gegen_boesartige_eingaben() {
        // Sehr tiefe Verschachtelung darf keinen Stack-Überlauf auslösen.
        let deep = "(".repeat(MAX_INPUT_LEN - 1) + "1";
        assert_eq!(eval(&deep), Err(CalcError::TooDeep));
        let minus = "-".repeat(MAX_INPUT_LEN - 1) + "1";
        assert_eq!(eval(&minus), Err(CalcError::TooDeep));
        let pow = "2^".repeat(MAX_INPUT_LEN / 2 - 1) + "1";
        assert_eq!(eval(&pow), Err(CalcError::TooDeep));
        // Zu lange Eingaben werden abgewiesen.
        assert_eq!(
            eval(&"1".repeat(MAX_INPUT_LEN + 1)),
            Err(CalcError::TooLong)
        );
        // Riesige Fakultät blockiert nicht.
        assert_eq!(eval("1e300!"), Err(CalcError::Overflow));
    }

    #[test]
    fn formatierung() {
        assert_eq!(format_number(0.1 + 0.2), "0.3");
        assert_eq!(format_number(2.0), "2");
        assert_eq!(format_number(-1.25), "-1.25");
        assert_eq!(format_number(1e20), "1e20");
        assert_eq!(format_number(1.5e-12), "1.5e-12");
    }
}
