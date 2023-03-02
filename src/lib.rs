use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum Terminal {
    Integer(u32),
    Dice,
    Remove,
    Higher,
    Lower,
    Add,
    Subtract,
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: Terminal,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: Terminal::Dice,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Lexeme {
    Dice(char, usize),
    Remove(char, usize),
    Direction(char, usize),
    Op(char, usize),
    Int(u32, usize),
}

fn lex(input: &str) -> Result<Vec<Lexeme>, String> {
    let mut result = Vec::new();

    let mut iter = input.chars().enumerate().peekable();

    while let Some(chr) = iter.peek() {
        let c = chr.1;
        let idx = chr.0;
        let mut skip_next = false;

        match c {
            '0'..='9' => {
                let n = lex_number(&mut iter);
                result.push(Lexeme::Int(n, idx));
                skip_next = true;
            }
            '+' => result.push(Lexeme::Op(c, idx)),
            '-' => result.push(Lexeme::Op(c, idx)),
            'd' => result.push(Lexeme::Dice(c, idx)),
            'r' => result.push(Lexeme::Remove(c, idx)),
            'l' => result.push(Lexeme::Direction(c, idx)),
            'h' => result.push(Lexeme::Direction(c, idx)),
            c if c.is_whitespace() => {}
            _ => return Err(format!("Unexpected character {} @ index {}", c, idx))
        }

        if !skip_next {
            iter.next();
        }
    }

    Ok(result)
}

fn lex_number<T: Iterator<Item=(usize, char)>>(iter: &mut Peekable<T>) -> u32 {
    let mut number: u32 = 0;

    while let Some(Ok(digit)) = iter.peek().map(|chr| chr.1.to_digit(10).ok_or("Not a digit!")) {
        number = number * 10 + digit;
        iter.next();
    }

    number
}


pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;

    parse_expr(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        Ok(n)
    } else {
        Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    })
}

fn parse_expr(tokens: &Vec<Lexeme>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (previous_node, next_pos) = parse_term(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&Lexeme::Op(o, op_idx)) => {
            let mut op_node = ParseNode::new();
            match o {
                '+' => op_node.entry = Terminal::Add,
                '-' => op_node.entry = Terminal::Subtract,
                _ => return Err(format!("Unexpected operation {o} in op token @ {op_idx}"))
            }
            op_node.children.push(previous_node);
            let (right_node, i) = parse_expr(tokens, next_pos + 1)?;
            op_node.children.push(right_node);

            Ok((op_node, i))
        }
        _ => {
            Ok((previous_node, next_pos))
        }
    }
}

fn parse_term(tokens: &Vec<Lexeme>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (previous_node, next_pos) = parse_dice_or_int(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&Lexeme::Remove(_, remv_idx)) => {
            let mut remove_node = ParseNode::new();
            remove_node.entry = Terminal::Remove;
            remove_node.children.push(previous_node);

            let c2 = tokens.get(next_pos + 1);
            match c2 {
                Some(&Lexeme::Int(n, c_idx)) => {
                    let mut count_node = ParseNode::new();
                    count_node.entry = Terminal::Integer(n);
                    remove_node.children.push(count_node);
                    if let &Lexeme::Direction(d, _) = tokens.get(next_pos + 2).ok_or(String::from(format!("Unexpected end of remove declaration @ {}, expected remove direction", c_idx)))? {
                        let mut direction_node = ParseNode::new();
                        match d {
                            'l' => direction_node.entry = Terminal::Lower,
                            'h' => direction_node.entry = Terminal::Higher,
                            _ => return Err(format!("Unexpected direction value {d}. This should never happen!"))
                        }
                        remove_node.children.push(direction_node);

                        Ok((remove_node, next_pos + 3))
                    } else {
                        return Err(format!("Unexpected token following removal count @ {c_idx}"))
                    }
                }
                Some(&Lexeme::Direction(d, _)) => {
                    let mut count_node = ParseNode::new();
                    count_node.entry = Terminal::Integer(1);
                    remove_node.children.push(count_node);
                    let mut direction_node = ParseNode::new();
                    match d {
                        'l' => direction_node.entry = Terminal::Lower,
                        'h' => direction_node.entry = Terminal::Higher,
                        _ => return Err(format!("Unexpected direction value {d}. This should never happen!"))
                    }
                    remove_node.children.push(direction_node);

                    Ok((remove_node, next_pos + 2))
                }
                _ => {
                    Err(format!("Unexpected token following removal indicator @ {remv_idx}"))
                }
            }
        }
        _ => {
            Ok((previous_node, next_pos))
        }
    }
}

fn parse_dice_or_int(tokens: &Vec<Lexeme>, pos: usize) -> Result<(ParseNode, usize), String> {
    let c: &Lexeme = tokens.get(pos).ok_or(String::from("Unexpected end of input, expected dice count"))?;

    match c {
        &Lexeme::Int(n, idx) => {
            let mut count_node = ParseNode::new();
            count_node.entry = Terminal::Integer(n);

            let c2 = tokens.get(pos + 1);

            return match c2 {
                Some(&Lexeme::Dice(_, idx2)) => {
                    let mut dice_node = ParseNode::new();
                    dice_node.entry = Terminal::Dice;
                    dice_node.children.push(count_node);
                    let mut size_node = ParseNode::new();
                    if let &Lexeme::Int(n, _) = tokens.get(pos + 2).ok_or(String::from(format!("Unexpected end of dice declaration @ {}, expected dice size", idx2)))? {
                        size_node.entry = Terminal::Integer(n);
                        dice_node.children.push(size_node);

                        Ok((dice_node, pos + 3))
                    } else {
                        Err(format!("Unexpected token following dice indicator @ {idx}"))
                    }
                }
                _ => {
                    Ok((count_node, pos + 1))
                }
            };
        }
        &Lexeme::Dice(_, idx) => {
            let mut dice_node = ParseNode::new();
            dice_node.entry = Terminal::Dice;
            let mut count_node = ParseNode::new();
            count_node.entry = Terminal::Integer(1);
            dice_node.children.push(count_node);
            if let &Lexeme::Int(n, _) = tokens.get(pos + 1).ok_or(String::from(format!("Unexpected end of dice declaration @ {}, expected dice size", idx)))? {
                let mut size_node = ParseNode::new();
                size_node.entry = Terminal::Integer(n);
                dice_node.children.push(size_node);

                Ok((dice_node, pos + 2))
            } else {
                Err(format!("Unexpected token following dice indicator @ {idx}"))
            }
        }
        _ => {
            Err(String::from("How did you even get here?"))
        }
    }
}

pub fn print(tree: &ParseNode) -> String {
    match tree.entry {
        Terminal::Higher => format!("h"),
        Terminal::Lower => format!("l"),
        Terminal::Integer(n) => format!("{n}"),
        Terminal::Dice => {
            format!("{}d{}",
                    print(tree.children.get(0).expect("dice need 2 children")),
                    print(tree.children.get(1).expect("dice need 2 children"))
            )
        }
        Terminal::Remove => {
            format!("{}r{}{}",
                    print(tree.children.get(0).expect("remove needs 3 children")),
                    print(tree.children.get(1).expect("remove needs 3 children")),
                    print(tree.children.get(2).expect("remove needs 3 children"))
            )
        }
        Terminal::Add => {
            format!("{} + {}",
                    print(tree.children.get(0).expect("add needs 2 children")),
                    print(tree.children.get(1).expect("add needs 2 children"))
            )
        }
        Terminal::Subtract => {
            format!("{} - {}",
                    print(tree.children.get(0).expect("subtract needs 2 children")),
                    print(tree.children.get(1).expect("subtract needs 2 children"))
            )
        }
    }
}



// #########
// #TESTING#
// #########

#[cfg(test)]
mod tests {
    use crate::{parse, print};

    #[test]
    fn parse_solitary_int() {
        let input = String::from("1");
        let output = print(&parse(&input).expect("Failed to parse `1` for test"));
        assert_eq!(input, output)
    }

    #[test]
    fn parse_solitary_dice_no_count() {
        let input = String::from("d20");
        let output = print(&parse(&input).expect("Failed to parse `d20` for test"));
        assert_eq!("1d20", output)
    }

    #[test]
    fn parse_solitary_dice_with_count() {
        let input = String::from("3d6");
        let output = print(&parse(&input).expect("Failed to parse `3d6` for test"));
        assert_eq!(input, output)
    }

    #[test]
    fn parse_solitary_remv_with_no_count_no_dice_count() {
        let input = String::from("d20rl");
        let output = print(&parse(&input).expect("Failed to parse `d20rl` for test"));
        assert_eq!("1d20r1l", output)
    }

    #[test]
    fn parse_solitary_remv_with_count_no_dice_count() {
        let input = String::from("d20r2l");
        let output = print(&parse(&input).expect("Failed to parse `d20r2l` for test"));
        assert_eq!("1d20r2l", output)
    }

    #[test]
    fn parse_solitary_remv_with_no_count_with_dice_count() {
        let input = String::from("2d20rl");
        let output = print(&parse(&input).expect("Failed to parse `2d20rl` for test"));
        assert_eq!("2d20r1l", output)
    }

    #[test]
    fn parse_solitary_remv_with_count_with_dice_count() {
        let input = String::from("3d20r2l");
        let output = print(&parse(&input).expect("Failed to parse `3d20r2l` for test"));
        assert_eq!(input, output)
    }

    #[test]
    fn parse_simple_add() {
        let input = String::from("2 + 6");
        let output = print(&parse(&input).expect("Failed to parse `2 + 6` for test"));
        assert_eq!(input, output)
    }

    #[test]
    fn parse_simple_subtract() {
        let input = String::from("6 - 2");
        let output = print(&parse(&input).expect("Failed to parse `6 - 2` for test"));
        assert_eq!(input, output)
    }

    #[test]
    fn parse_complex() {
        let input = String::from("15d10r3l-2d4rl+d3+7d7+66-2d4");
        let output = print(&parse(&input).expect("Failed to parse `15d10r3l - 2d4rl + d3 + 7d7 + 66 - 2d4` for test"));
        assert_eq!("15d10r3l - 2d4r1l + 1d3 + 7d7 + 66 - 2d4", output)
    }
}