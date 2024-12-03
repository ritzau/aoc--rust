use crate::cache::AocCache;
use crate::input::InputFetcher;
use crate::s15::e07::Operand::{Value, Wire};
use crate::s15::e07::Operation::{And, Forward, LShift, Not, Or, RShift};
use crate::s15::YEAR;
use crate::{head, Day, PuzzleResult};
use regex::{Match, Regex};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

const DAY: Day = Day(7);

pub fn some_assembly_required(aoc: &AocCache) -> PuzzleResult<bool> {
    head(YEAR, DAY, "Some Assembly Required");
    let input = aoc.get_input(YEAR, DAY)?;

    #[cfg(feature = "EXCLUDE_SLOW_SOLUTIONS")]
    {
        println!("Skipping...");
        return Ok(true);
    }

    #[cfg(not(feature = "EXCLUDE_SLOW_SOLUTIONS"))]
    {
        let mut circuit = Circuit::new();

        for line in input.lines()? {
            let line = line;
            let gate = Gate::parse(&line);
            circuit.add_gate(gate);
        }

        let a1 = match circuit.eval("a") {
            Some(result) => {
                println!("aoc15e07a: {}", result);
                result
            }
            None => return Ok(false),
        };

        let mut circuit = Circuit::new();

        for line in input.lines()? {
            let line = line;
            let mut gate = Gate::parse(&line);
            if gate.operation == Forward && gate.output == "b" {
                gate.inputs = vec![Value(a1)]
            }
            circuit.add_gate(gate);
        }

        let a2 = match circuit.eval("a") {
            Some(result) => {
                println!("aoc15e07b: {}", result);
                result
            }
            None => return Ok(false),
        };

        Ok(a1 == 16076 && a2 == 2797)
    }
}

type WireValue = u16;

#[derive(Debug, PartialEq)]
enum Operand {
    Wire(String),
    Value(WireValue),
}

impl<T: AsRef<str>> From<T> for Operand {
    fn from(value: T) -> Self {
        let s = value.as_ref();

        if let Ok(value1) = s.parse::<WireValue>() {
            Value(value1)
        } else {
            Wire(s.into())
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Forward,
    Not,
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Debug)]
struct Gate {
    operation: Operation,
    inputs: Vec<Operand>,
    output: String,
}

impl Gate {
    fn new(operation: Operation, inputs: Vec<Operand>, output: String) -> Self {
        Gate {
            operation,
            inputs,
            output,
        }
    }

    fn parse(s: &str) -> Self {
        fn match_as_str(m: Option<Match>) -> &str {
            m.unwrap().as_str()
        }

        let direct_pattern = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
        let unary_pattern = Regex::new(r"^(NOT) (\w+) -> (\w+)$").unwrap();
        let binary_pattern = Regex::new(r"^(\w+) (AND|OR|LSHIFT|RSHIFT) (\w+) -> (\w+)$").unwrap();

        let s = s.as_ref();

        if let Some(cs) = direct_pattern.captures(s) {
            let operand = match_as_str(cs.get(1)).into();
            let wire = match_as_str(cs.get(2)).into();
            Self::new(Forward, vec![operand], wire)
        } else if let Some(cs) = unary_pattern.captures(s) {
            let operation = match_as_str(cs.get(1));
            let operation = match operation {
                "NOT" => Not,
                _ => panic!("Invalid unary operation: {operation}"),
            };
            let operand = match_as_str(cs.get(2)).into();
            let wire = match_as_str(cs.get(3)).into();
            return Self::new(operation, vec![operand], wire);
        } else if let Some(cs) = binary_pattern.captures(s) {
            let operand_a = match_as_str(cs.get(1)).into();
            let operand_b = match_as_str(cs.get(3)).into();

            let operation = match_as_str(cs.get(2));
            let operation = match operation {
                "AND" => And,
                "OR" => Or,
                "LSHIFT" => LShift,
                "RSHIFT" => RShift,
                _ => panic!("Invalid unary operation: {operation}"),
            };

            let wire = match_as_str(cs.get(4)).into();

            return Self::new(operation, vec![operand_a, operand_b], wire);
        } else {
            panic!("Can't parse instruction: {s}");
        }
    }

    fn compute(&self, wires: &HashMap<String, WireValue>) -> Option<WireValue> {
        let inputs: Vec<_> = self
            .inputs
            .iter()
            .map(|i| match i {
                Value(v) => Some(v),
                Wire(id) => wires.get(id),
            })
            .collect();

        if inputs.iter().all(|v| v.is_some()) {
            let inputs: Vec<_> = inputs.iter().map(|v| v.unwrap()).collect();
            Some(match self.operation {
                Forward => *inputs[0],
                Not => !*inputs[0],
                And => *inputs[0] & *inputs[1],
                Or => *inputs[0] | *inputs[1],
                LShift => *inputs[0] << *inputs[1],
                RShift => *inputs[0] >> *inputs[1],
            })
        } else {
            None
        }
    }
}

struct Circuit {
    wires: HashMap<String, WireValue>,
    inputs_to_gates: HashMap<String, Vec<Rc<Gate>>>,
    outputs_to_gates: HashMap<String, Rc<Gate>>,
}

impl Circuit {
    fn new() -> Self {
        Circuit {
            wires: HashMap::new(),
            inputs_to_gates: HashMap::new(),
            outputs_to_gates: HashMap::new(),
        }
    }

    fn add_gate(&mut self, gate: Gate) {
        let gate = Rc::new(gate);
        for input in &gate.inputs {
            if let Wire(name) = input {
                self.inputs_to_gates
                    .entry(name.clone())
                    .or_default()
                    .push(gate.clone());
            }
        }
        let old = self
            .outputs_to_gates
            .insert(gate.output.clone(), gate.clone());
        assert!(old.is_none());
    }

    fn eval(&mut self, wire_id: &str) -> Option<WireValue> {
        if let Some(value) = self.wires.get(wire_id) {
            return Some(*value);
        }

        let gate = self.outputs_to_gates.get(wire_id).cloned();
        if let Some(gate) = gate {
            for input in &gate.inputs {
                if let Wire(name) = input {
                    self.eval(name);
                }
            }

            if let Some(output) = gate.compute(&self.wires) {
                self.wires.insert(wire_id.to_owned(), output);
                return Some(output);
            }
        }

        None
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Circuit: {{")?;
        for (key, value) in &self.wires {
            writeln!(f, "  {}: {},", key, value)?;
        }
        write!(f, "}}")
    }
}
