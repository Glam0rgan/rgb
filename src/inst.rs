use crate::cpu::Cpu;
use crate::mmu::Mmu;

// ld b,b
#[allow(unused_variables)]
fn op_0040(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_b(v);

    (4, 1)
}

// ld b,c
#[allow(unused_variables)]
fn op_0041(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_b(v);

    (4, 1)
}

// ld b,d
#[allow(unused_variables)]
fn op_0042(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_b(v);

    (4, 1)
}

// ld b,e
#[allow(unused_variables)]
fn op_0043(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_b(v);

    (4, 1)
}

// ld b,h
#[allow(unused_variables)]
fn op_0044(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_b(v);

    (4, 1)
}

// ld b,l
#[allow(unused_variables)]
fn op_0045(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_b(v);

    (4, 1)
}

// ld b,a
#[allow(unused_variables)]
fn op_0047(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_b(v);

    (4, 1)
}

// ld c,b
#[allow(unused_variables)]
fn op_0048(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_c(v);

    (4, 1)
}

// ld c,c
#[allow(unused_variables)]
fn op_0049(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_c(v);

    (4, 1)
}

// ld c,d
#[allow(unused_variables)]
fn op_004A(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_c(v);

    (4, 1)
}

// ld c,e
#[allow(unused_variables)]
fn op_004B(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_c(v);

    (4, 1)
}

// ld c,h
#[allow(unused_variables)]
fn op_004C(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_c(v);

    (4, 1)
}

// ld c,l
#[allow(unused_variables)]
fn op_004D(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_c(v);

    (4, 1)
}

// ld c,a
#[allow(unused_variables)]
fn op_004F(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_c(v);

    (4, 1)
}

// ld d,b
#[allow(unused_variables)]
fn op_0050(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_d(v);

    (4, 1)
}

// ld d,c
#[allow(unused_variables)]
fn op_0051(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_d(v);

    (4, 1)
}

// ld d,d
#[allow(unused_variables)]
fn op_0052(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_d(v);

    (4, 1)
}

// ld d,e
#[allow(unused_variables)]
fn op_0053(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_d(v);

    (4, 1)
}

// ld d,h
#[allow(unused_variables)]
fn op_0054(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_d(v);

    (4, 1)
}

// ld d,l
#[allow(unused_variables)]
fn op_0055(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_d(v);

    (4, 1)
}

// ld d,a
#[allow(unused_variables)]
fn op_0057(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_d(v);

    (4, 1)
}

// ld e,b
#[allow(unused_variables)]
fn op_0058(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_e(v);

    (4, 1)
}

// ld e,c
#[allow(unused_variables)]
fn op_0059(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_e(v);

    (4, 1)
}

// ld e,d
#[allow(unused_variables)]
fn op_005A(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_e(v);

    (4, 1)
}

// ld e,e
#[allow(unused_variables)]
fn op_005B(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_e(v);

    (4, 1)
}

// ld e,h
#[allow(unused_variables)]
fn op_005C(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_e(v);

    (4, 1)
}

// ld e,l
#[allow(unused_variables)]
fn op_005D(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_e(v);

    (4, 1)
}

// ld e,a
#[allow(unused_variables)]
fn op_005F(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_e(v);

    (4, 1)
}

// ld h,b
#[allow(unused_variables)]
fn op_0060(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_h(v);

    (4, 1)
}

// ld h,c
#[allow(unused_variables)]
fn op_0061(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_h(v);

    (4, 1)
}

// ld h,d
#[allow(unused_variables)]
fn op_0062(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_h(v);

    (4, 1)
}

// ld h,e
#[allow(unused_variables)]
fn op_0063(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_h(v);

    (4, 1)
}

// ld h,h
#[allow(unused_variables)]
fn op_0064(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_h(v);

    (4, 1)
}

// ld h,l
#[allow(unused_variables)]
fn op_0065(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_h(v);

    (4, 1)
}

// ld h,a
#[allow(unused_variables)]
fn op_0067(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_h(v);

    (4, 1)
}

// ld l,b
#[allow(unused_variables)]
fn op_0068(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_l(v);

    (4, 1)
}

// ld l,c
#[allow(unused_variables)]
fn op_0069(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_l(v);

    (4, 1)
}

// ld l,d
#[allow(unused_variables)]
fn op_006A(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_l(v);

    (4, 1)
}

// ld l,e
#[allow(unused_variables)]
fn op_006B(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_l(v);

    (4, 1)
}

// ld l,h
#[allow(unused_variables)]
fn op_006C(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_l(v);

    (4, 1)
}

// ld l,l
#[allow(unused_variables)]
fn op_006D(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_l(v);

    (4, 1)
}

// ld l,a
#[allow(unused_variables)]
fn op_006F(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_l(v);

    (4, 1)
}

// ld a,b
#[allow(unused_variables)]
fn op_0078(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_b();
    cpu.set_a(v);

    (4, 1)
}

// ld a,c
#[allow(unused_variables)]
fn op_0079(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_c();
    cpu.set_a(v);

    (4, 1)
}

// ld a,d
#[allow(unused_variables)]
fn op_007A(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_a(v);

    (4, 1)
}

// ld a,e
#[allow(unused_variables)]
fn op_007B(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_a(v);

    (4, 1)
}

// ld a,h
#[allow(unused_variables)]
fn op_007C(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_a(v);

    (4, 1)
}

// ld a,l
#[allow(unused_variables)]
fn op_007D(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_a(v);

    (4, 1)
}

// ld a,a
#[allow(unused_variables)]
fn op_007F(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_a(v);

    (4, 1)
}