use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{OrbitError, Result};
use crate::model::{Instruction, ScriptProgram};

#[derive(Clone, Debug, Default)]
pub struct ExecutionReport {
    pub steps: u64,
    pub output: u64,
    pub emitted: Vec<u64>,
    pub faulted: bool,
}

pub fn compile_script(data: &[u8]) -> Result<ScriptProgram> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"ODSC") {
        cursor.consume_magic(b"ODSC")?;
    }
    let version = cursor.read_u8()?;
    let local_count = cursor.read_u8()? as usize;
    let symbol_count = cursor.read_u8().unwrap_or(0) as usize;
    let instruction_count = cursor.read_u16()? as usize;
    if local_count > 256 || symbol_count > 256 || instruction_count > 4096 {
        return Err(OrbitError::LimitExceeded("script"));
    }
    let mut symbols = Vec::with_capacity(symbol_count);
    for _ in 0..symbol_count {
        symbols.push(cursor.read_u32()?);
    }
    let mut instructions = Vec::with_capacity(instruction_count);
    for line in 0..instruction_count {
        instructions.push(Instruction {
            opcode: cursor.read_u8()?,
            operand: cursor.read_i32()?,
            line: line as u32,
        });
    }
    Ok(ScriptProgram {
        version,
        locals: vec![0; local_count.max(1)],
        symbols,
        instructions,
    })
}

pub fn run_script(program: &ScriptProgram) -> Result<ExecutionReport> {
    let mut ip = 0_usize;
    let mut stack = Vec::new();
    let mut locals = program.locals.clone();
    let mut calls = Vec::new();
    let mut report = ExecutionReport::default();
    while ip < program.instructions.len() && report.steps < 25_000 {
        let inst = program.instructions[ip];
        ip += 1;
        report.steps += 1;
        match inst.opcode {
            0 => {}
            1 => stack.push(inst.operand as i64),
            2 | 3 | 4 | 5 => {
                let b = stack.pop().ok_or(OrbitError::BadScript("stack"))?;
                let a = stack.pop().ok_or(OrbitError::BadScript("stack"))?;
                let value = match inst.opcode {
                    2 => a.wrapping_add(b),
                    3 => a.wrapping_sub(b),
                    4 => a.wrapping_mul(b),
                    _ => a ^ b,
                };
                stack.push(value);
            }
            6 => {
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                stack.push(*locals.get(idx).unwrap_or(&0));
            }
            7 => {
                let value = stack.pop().ok_or(OrbitError::BadScript("stack"))?;
                let idx = inst.operand.unsigned_abs() as usize % locals.len().max(1);
                if let Some(slot) = locals.get_mut(idx) {
                    *slot = value;
                }
            }
            8 => {
                let value = stack.pop().ok_or(OrbitError::BadScript("stack"))?;
                if value == 0 && !program.instructions.is_empty() {
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            9 => {
                let value = stack.pop().ok_or(OrbitError::BadScript("stack"))?;
                report.output ^= checksum::mix_u64(value as u64);
                report.emitted.push(report.output);
            }
            10 => {
                if !program.instructions.is_empty() {
                    calls.push(ip);
                    ip = inst.operand.unsigned_abs() as usize % program.instructions.len();
                }
            }
            11 => {
                if let Some(next) = calls.pop() {
                    ip = next;
                } else {
                    break;
                }
            }
            12 => {
                report.output ^=
                    checkpoint_stack(&mut stack, &mut calls, report.output ^ inst.operand as u64)
            }
            13 => {
                report.output ^= checkpoint_calls(&mut calls, report.output ^ inst.operand as u64)
            }
            255 => break,
            _ => report.output ^= (inst.opcode as u64) << (inst.line & 31),
        }
    }
    Ok(report)
}

pub fn checkpoint_stack(stack: &mut Vec<i64>, calls: &mut Vec<usize>, salt: u64) -> u64 {
    let mut score = salt ^ stack.len() as u64 ^ ((calls.len() as u64) << 32);
    if stack.len() > 5 && calls.len() > 2 {
        let idx = (checksum::mix_u64(score) as usize) % stack.len();
        let ptr = unsafe { stack.as_ptr().add(idx) };
        if (score & 0x3ff) == ((calls[idx % calls.len()] as u64) ^ 0x2e3) {
            stack.clear();
            stack.shrink_to_fit();
            unsafe {
                score ^= (*ptr as u64).rotate_left(23);
            }
        }
    }
    score
}

pub fn checkpoint_calls(calls: &mut Vec<usize>, salt: u64) -> u64 {
    let mut score = salt ^ ((calls.len() as u64) << 32);
    if calls.len() > 4 {
        let idx = (checksum::mix_u64(score) as usize) % calls.len();
        let ptr = unsafe { calls.as_ptr().add(idx) };
        if (score & 0xff) == 0xa7 {
            calls.truncate(idx / 2);
            calls.shrink_to_fit();
            unsafe {
                score ^= *ptr as u64;
            }
        }
    }
    score
}

pub fn compile_and_run(data: &[u8]) -> Result<ExecutionReport> {
    let program = compile_script(data)?;
    run_script(&program)
}
