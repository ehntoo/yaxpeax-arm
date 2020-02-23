use yaxpeax_arch::{Arch, Decoder, LengthedInstruction};
use yaxpeax_arm::armv7::{ARMv7, Instruction, ConditionCode, Operand, Opcode, Reg, RegShift};

fn test_decode(data: [u8; 4], expected: Instruction) {
    let instr = <ARMv7 as Arch>::Decoder::default().decode(data.to_vec()).unwrap();
    assert!(
        instr == expected,
        "decode error for {:02x}{:02x}{:02x}{:02x}:\n  decoded: {:?}\n expected: {:?}\n",
        data[0], data[1], data[2], data[3],
        instr, expected
    );
}

fn test_display(data: [u8; 4], expected: &'static str) {
    let instr = <ARMv7 as Arch>::Decoder::default().decode(data.to_vec()).unwrap();
    let text = format!("{}", instr);
    assert!(
        text == expected,
        "display error for {:02x}{:02x}{:02x}{:02x}:\n  decoded: {:?}\n displayed: {}\n expected: {}\n",
        data[0], data[1], data[2], data[3],
        instr,
        text, expected
    );
}

#[test]
fn test_decode_str_ldr() {
    test_decode(
        [0x24, 0xc0, 0x9f, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::LDR(true, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(12)),
                Operand::RegDisp(Reg::from_u8(15), 0x24),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x10, 0x00, 0x9f, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::LDR(true, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(0)),
                Operand::RegDisp(Reg::from_u8(15), 0x10),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x04, 0x20, 0x2d, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::STR(false, true, true),
            operands: [
                Operand::Reg(Reg::from_u8(2)),
                Operand::RegDerefPostindexOffset(Reg::from_u8(13), 4, false),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x04, 0x00, 0x2d, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::STR(false, true, true),
            operands: [
                Operand::Reg(Reg::from_u8(0)),
                Operand::RegDerefPostindexOffset(Reg::from_u8(13), 4, false),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x14, 0x30, 0x9f, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::LDR(true, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(3)),
                Operand::RegDisp(Reg::from_u8(15), 0x14),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x14, 0x20, 0x9f, 0xe5],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::LDR(true, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(2)),
                Operand::RegDisp(Reg::from_u8(15), 0x14),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
}

#[test]
fn test_synchronization() {
    test_display(
        [0x94, 0x8f, 0x8a, 0xe1],
        "strex r8, r4, [r10]"
    );
    test_display(
        [0x9f, 0x8f, 0x9a, 0xe1],
        "ldrex r8, [r10]"
    );
    test_display(
        [0x94, 0x2f, 0xa4, 0xe1],
        "strexd r2, r4, r5, [r4]"
    );
    test_display(
        [0x9f, 0x2f, 0xb4, 0xe1],
        "ldrexd r2, r3, [r4]"
    );
    test_display(
        [0x9f, 0x2f, 0xc4, 0xe1],
        "strexb r2, pc, [r4]"
    );
    test_display(
        [0x9f, 0x2f, 0xd4, 0xe1],
        "ldrexb r2, [r4]"
    );
    test_display(
        [0x9f, 0x2f, 0xe4, 0xe1],
        "strexh r2, pc, [r4]"
    );
    test_display(
        [0x9f, 0x2f, 0xf4, 0xe1],
        "ldrexh r2, [r4]"
    );
}

#[test]
fn test_str() {
    test_display(
        [0xb5, 0x53, 0x68, 0xe0],
        "strht r5, [r8], -0x35"
    );
}

#[test]
fn test_data_imm() {
    test_display(
        [0x12, 0x34, 0xa0, 0xe3],
        "mov r3, 0x12000000"
    );
    test_display(
        [0x12, 0x44, 0x9c, 0xe3],
        "orrs r4, ip, 0x12000000"
    );
}

#[test]
fn test_decode_misc() {
    test_display(
        [0x13, 0x5f, 0x6f, 0xe1],
        "clz r5, r3"
    );
    test_display(
        [0xc8, 0xac, 0x0b, 0xe1],
        "smlabt fp, r8, ip, r10"
    );
    test_display(
        [0x32, 0xff, 0x2f, 0xe1],
        "blx r2"
    );
    test_display(
        [0x02, 0x00, 0xa0, 0xe3],
        "mov r0, 0x2"
    );
    test_display(
        [0xe8, 0x10, 0x9f, 0xe5],
        "ldr r1, [pc, #0x3a0]"
    );
}

#[test]
fn test_decode_pop() {
    test_decode(
        [0x04, 0x10, 0x9d, 0xe4],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::LDR(true, false, false),
            operands: [
                Operand::Reg(Reg::from_u8(1)),
                Operand::RegDerefPostindexOffset(Reg::from_u8(13), 0x4, true),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_display(
        [0x04, 0x10, 0x9d, 0xe4],
        "pop {r1}"
    );
    test_decode(
        [0xf0, 0x40, 0x2d, 0xe9],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::STM(false, true, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(13)),
                Operand::RegList(16624),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_display(
        [0xf0, 0x40, 0x2d, 0xe9],
        "push {r4, r5, r6, r7, lr}"
    );
    test_decode(
        [0xf0, 0x80, 0xbd, 0x18],
        Instruction {
            condition: ConditionCode::NE,
            opcode: Opcode::LDM(true, false, true, false),
            operands: [
                Operand::Reg(Reg::from_u8(13)),
                Operand::RegList(33008),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_display(
        [0xf0, 0x80, 0xbd, 0x18],
        "popne {r4, r5, r6, r7, pc}"
    );
}

#[test]
fn test_decode_mov() {
    test_decode(
        [0x0d, 0x20, 0xa0, 0xe1],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::MOV,
            operands: [
                Operand::Reg(Reg::from_u8(2)),
                Operand::Reg(Reg::from_u8(13)),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x00, 0xb0, 0xa0, 0xe3],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::MOV,
            operands: [
                Operand::Reg(Reg::from_u8(11)),
                Operand::Imm32(0),
                Operand::Nothing,
                Operand::Nothing,
            ],
            s: false
        }
    );
}

#[test]
fn test_decode_arithmetic() {
    test_decode(
        [0x18, 0x1d, 0x00, 0x00],
        Instruction {
            condition: ConditionCode::EQ,
            opcode: Opcode::AND,
            operands: [Operand::Reg(Reg::from_u8(1)), Operand::Reg(Reg::from_u8(0)), Operand::RegShift(RegShift::from_raw(0xd18)), Operand::Nothing],
            s: false
        }
    );
    test_display(
        [0x18, 0x1d, 0x00, 0x00],
        "andeq r1, r0, r8, lsl sp",
    );
    test_decode(
        [0x03, 0x30, 0x8f, 0xe0],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::ADD,
            operands: [Operand::Reg(Reg::from_u8(3)), Operand::Reg(Reg::from_u8(15)), Operand::Reg(Reg::from_u8(3)), Operand::Nothing],
            s: false
        }
    );
    test_decode(
        [0x03, 0x30, 0x66, 0xe0],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::RSB,
            operands: [Operand::Reg(Reg::from_u8(3)), Operand::Reg(Reg::from_u8(6)), Operand::Reg(Reg::from_u8(3)), Operand::Nothing],
            s: false
        }
    );
    test_decode(
        [0x43, 0x31, 0xa0, 0xe1],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::MOV,
            operands: [Operand::Reg(Reg::from_u8(3)), Operand::Reg(Reg::from_u8(0)), Operand::RegShift(RegShift::from_raw(0x143)), Operand::Nothing],
            s: false
        }
    );
    test_decode(
        [0x01, 0x50, 0x43, 0xe2],
        Instruction {
            condition: ConditionCode::AL,
            opcode: Opcode::SUB,
            operands: [Operand::Reg(Reg::from_u8(5)), Operand::Reg(Reg::from_u8(3)), Operand::Imm32(1), Operand::Nothing],
            s: false
        }
    );
}

#[test]
fn test_decode_mul() {
    test_decode(
        [0x9c, 0x7d, 0x0b, 0x00],
        Instruction {
            condition: ConditionCode::EQ,
            opcode: Opcode::MUL,
            operands: [
                Operand::Reg(Reg::from_u8(11)),
                Operand::Reg(Reg::from_u8(12)),
                Operand::Reg(Reg::from_u8(13)),
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x90, 0x79, 0x09, 0x00],
        Instruction {
            condition: ConditionCode::EQ,
            opcode: Opcode::MUL,
            operands: [
                Operand::Reg(Reg::from_u8(9)),
                Operand::Reg(Reg::from_u8(0)),
                Operand::Reg(Reg::from_u8(9)),
                Operand::Nothing,
            ],
            s: false
        }
    );
    test_decode(
        [0x94, 0x79, 0x09, 0x00],
        Instruction {
            condition: ConditionCode::EQ,
            opcode: Opcode::MUL,
            operands: [
                Operand::Reg(Reg::from_u8(9)),
                Operand::Reg(Reg::from_u8(4)),
                Operand::Reg(Reg::from_u8(9)),
                Operand::Nothing,
            ],
            s: false
        }
    );
}

static INSTRUCTION_BYTES: [u8; 4 * 60] = [
        0x24, 0xc0, 0x9f, 0xe5,
        0x00, 0xb0, 0xa0, 0xe3,
        0x04, 0x10, 0x9d, 0xe4,
        0x0d, 0x20, 0xa0, 0xe1,
        0x04, 0x20, 0x2d, 0xe5,
        0x04, 0x00, 0x2d, 0xe5,
        0x10, 0x00, 0x9f, 0xe5,
        0x10, 0x30, 0x9f, 0xe5,
        0x04, 0xc0, 0x2d, 0xe5,
        0x4b, 0xfe, 0xff, 0xeb,
        0xd5, 0xfd, 0xff, 0xeb,
        0x90, 0x79, 0x09, 0x00,
        0x64, 0xd0, 0x01, 0x00,
        0x94, 0x79, 0x09, 0x00,
        0x14, 0x30, 0x9f, 0xe5,
        0x14, 0x20, 0x9f, 0xe5,
        0x03, 0x30, 0x8f, 0xe0,
        0x02, 0x10, 0x93, 0xe7,
        0x00, 0x00, 0x51, 0xe3,
        0x0e, 0xf0, 0xa0, 0x01,
        0x01, 0xfe, 0xff, 0xea,
        0x58, 0x75, 0x09, 0x00,
        0xec, 0x02, 0x00, 0x00,
        0xf0, 0x40, 0x2d, 0xe9,
        0x54, 0x70, 0x9f, 0xe5,
        0x00, 0x30, 0xd7, 0xe5,
        0x00, 0x00, 0x53, 0xe3,
        0xf0, 0x80, 0xbd, 0x18,
        0x48, 0x60, 0x9f, 0xe5,
        0x48, 0x30, 0x9f, 0xe5,
        0x48, 0x40, 0x9f, 0xe5,
        0x03, 0x30, 0x66, 0xe0,
        0x43, 0x31, 0xa0, 0xe1,
        0x00, 0x20, 0x94, 0xe5,
        0x01, 0x50, 0x43, 0xe2,
        0x05, 0x00, 0x52, 0xe1,
        0x06, 0x00, 0x00, 0x2a,
        0x01, 0x30, 0x82, 0xe2,
        0x00, 0x30, 0x84, 0xe5,
        0x0f, 0xe0, 0xa0, 0xe1,
        0x03, 0xf1, 0x96, 0xe7,
        0x00, 0x20, 0x94, 0xe5,
        0x05, 0x00, 0x52, 0xe1,
        0xf8, 0xff, 0xff, 0x3a,
        0x01, 0x30, 0xa0, 0xe3,
        0x00, 0x30, 0xc7, 0xe5,
        0xf0, 0x80, 0xbd, 0xe8,
        0x9c, 0x7d, 0x0b, 0x00,
        0xa0, 0x33, 0x0b, 0x00,
        0xa4, 0x33, 0x0b, 0x00,
        0xa0, 0x7d, 0x0b, 0x00,
        0x04, 0xe0, 0x2d, 0xe5,
        0x04, 0xf0, 0x9d, 0xe4,
        0x24, 0x00, 0x9f, 0xe5,
        0x00, 0x30, 0x90, 0xe5,
        0x00, 0x00, 0x53, 0xe3,
        0x04, 0xe0, 0x2d, 0xe5,
        0x04, 0xf0, 0x9d, 0x04,
        0x14, 0x30, 0x9f, 0xe5,
        0x00, 0x00, 0x53, 0xe3
    ];


#[test]
fn test_decode_span() {
    let mut i = 0u32;
    while i < INSTRUCTION_BYTES.len() as u32 {
        let instr = <ARMv7 as Arch>::Decoder::default().decode(INSTRUCTION_BYTES[(i as usize)..].iter().cloned()).unwrap();
        println!(
            "Decoded {:02x}{:02x}{:02x}{:02x}: {}", //{:?}\n  {}",
            INSTRUCTION_BYTES[i as usize],
            INSTRUCTION_BYTES[i as usize + 1],
            INSTRUCTION_BYTES[i as usize + 2],
            INSTRUCTION_BYTES[i as usize + 3],
//            instr,
            instr);
        i += instr.len();
    }
//    panic!("done");
}
/*
 * from debian 5.0.10 bash 3.2-4_arm
 *   0x0001bee4      24c09fe5       ldr ip, sym.__libc_csu_fini
 *   0x0001bee8      00b0a0e3       mov fp, 0
 *   0x0001beec      04109de4       pop {r1}
 *   0x0001bef0      0d20a0e1       mov r2, sp
 *   0x0001bef4      04202de5       str r2, [sp, -4]!
 *   0x0001bef8      04002de5       str r0, [sp, -4]!
 *   0x0001befc      10009fe5       ldr r0, sym.main
 *   0x0001bf00      10309fe5       ldr r3, sym.__libc_csu_init
 *   0x0001bf04      04c02de5       str ip, [sp, -4]!
 *   0x0001bf08      4bfeffeb       bl sym.imp.__libc_start_main
 *   0x0001bf0c      d5fdffeb       bl sym.imp.abort
 *   0x0001bf10      90790900       muleq sb, r0, sb
 *   0x0001bf14      64d00100       andeq sp, r1, r4, rrx
 *   0x0001bf18      94790900       muleq sb, r4, sb
 *   0x0001bf1c      14309fe5       ldr r3, [0x0001bf38]
 *   0x0001bf20      14209fe5       ldr r2, [0x0001bf3c]
 *   0x0001bf24      03308fe0       add r3, pc, r3
 *   0x0001bf28      021093e7       ldr r1, [r3, r2]
 *   0x0001bf2c      000051e3       cmp r1, 0
 *   0x0001bf30      0ef0a001       moveq pc, lr
 *   0x0001bf34      01feffea       b loc.imp.__gmon_start__
 *   0x0001bf38      58750900       andeq r7, sb, r8, asr r5
 *   0x0001bf3c      ec020000       andeq r0, r0, ip, ror 5
 *   0x0001bf40      f0402de9       push {r4, r5, r6, r7, lr}
 *   0x0001bf44      54709fe5       ldr r7, [0x0001bfa0]
 *   0x0001bf48      0030d7e5       ldrb r3, [r7]
 *   0x0001bf4c      000053e3       cmp r3, 0
 *   0x0001bf50      f080bd18       popne {r4, r5, r6, r7, pc}
 *   0x0001bf54      48609fe5       ldr r6, [0x0001bfa4]
 *   0x0001bf58      48309fe5       ldr r3, [0x0001bfa8]
 *   0x0001bf5c      48409fe5       ldr r4, [0x0001bfac]
 *   0x0001bf60      033066e0       rsb r3, r6, r3
 *   0x0001bf64      4331a0e1       asr r3, r3, 2
 *   0x0001bf68      002094e5       ldr r2, [r4]
 *   0x0001bf6c      015043e2       sub r5, r3, 1
 *   0x0001bf70      050052e1       cmp r2, r5
 *   0x0001bf74      0600002a       bhs 0x1bf94
 *   0x0001bf78      013082e2       add r3, r2, 1
 *   0x0001bf7c      003084e5       str r3, [r4]
 *   0x0001bf80      0fe0a0e1       mov lr, pc
 *   0x0001bf84      03f196e7       ldr pc, [r6, r3, lsl 2]
 *   0x0001bf88      002094e5       ldr r2, [r4]
 *   0x0001bf8c      050052e1       cmp r2, r5
 *   0x0001bf90      f8ffff3a       blo 0x1bf78
 *   0x0001bf94      0130a0e3       mov r3, 1
 *   0x0001bf98      0030c7e5       strb r3, [r7]
 *   0x0001bf9c      f080bde8       pop {r4, r5, r6, r7, pc}
 *   0x0001bfa0      9c7d0b00       muleq fp, ip, sp
 *   0x0001bfa4      a0330b00       andeq r3, fp, r0, lsr 7
 *   0x0001bfa8      a4330b00       andeq r3, fp, r4, lsr 7
 *   0x0001bfac      a07d0b00       andeq r7, fp, r0, lsr 27
 *   0x0001bfb0      04e02de5       str lr, [sp, -4]!
 *   0x0001bfb4      04f09de4       pop {pc}
 *   0x0001bfb8      24009fe5       ldr r0, [0x0001bfe4]
 *   0x0001bfbc      003090e5       ldr r3, [r0]
 *   0x0001bfc0      000053e3       cmp r3, 0
 *   0x0001bfc4      04e02de5       str lr, [sp, -4]!
 *   0x0001bfc8      04f09d04       popeq {pc}
 *   0x0001bfcc      14309fe5       ldr r3, [0x0001bfe8]
 *   0x0001bfd0      000053e3       cmp r3, 0
 */

/*
use test::Bencher;
#[bench]
pub fn bench_60000_instrs(b: &mut Bencher) {
    b.iter(|| {
        for i in (0..1000) {
            let mut iter = INSTRUCTION_BYTES.iter().map(|x| *x);
            let decoder = <ARMv7 as Arch>::Decoder::default();
            let mut result = Instruction::default();
            loop {
                match decoder.decode_into(&mut result, &mut iter) {
                    Ok(result) => {
                        test::black_box(&result);
                    },
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    });
}
*/
