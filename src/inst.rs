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
fn op_004a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_c(v);

    (4, 1)
}

// ld c,e
#[allow(unused_variables)]
fn op_004b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_c(v);

    (4, 1)
}

// ld c,h
#[allow(unused_variables)]
fn op_004c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_c(v);

    (4, 1)
}

// ld c,l
#[allow(unused_variables)]
fn op_004d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_c(v);

    (4, 1)
}

// ld c,a
#[allow(unused_variables)]
fn op_004f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
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
fn op_005a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_e(v);

    (4, 1)
}

// ld e,e
#[allow(unused_variables)]
fn op_005b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_e(v);

    (4, 1)
}

// ld e,h
#[allow(unused_variables)]
fn op_005c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_e(v);

    (4, 1)
}

// ld e,l
#[allow(unused_variables)]
fn op_005d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_e(v);

    (4, 1)
}

// ld e,a
#[allow(unused_variables)]
fn op_005f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
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
fn op_006a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_l(v);

    (4, 1)
}

// ld l,e
#[allow(unused_variables)]
fn op_006b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_l(v);

    (4, 1)
}

// ld l,h
#[allow(unused_variables)]
fn op_006c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_l(v);

    (4, 1)
}

// ld l,l
#[allow(unused_variables)]
fn op_006d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_l(v);

    (4, 1)
}

// ld l,a
#[allow(unused_variables)]
fn op_006f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
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
fn op_007a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_d();
    cpu.set_a(v);

    (4, 1)
}

// ld a,e
#[allow(unused_variables)]
fn op_007b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_e();
    cpu.set_a(v);

    (4, 1)
}

// ld a,h
#[allow(unused_variables)]
fn op_007c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_h();
    cpu.set_a(v);

    (4, 1)
}

// ld a,l
#[allow(unused_variables)]
fn op_007d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_l();
    cpu.set_a(v);

    (4, 1)
}

// ld a,a
#[allow(unused_variables)]
fn op_007f(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let v = cpu.get_a();
    cpu.set_a(v);

    (4, 1)
}

// ld a,(bc)
#[allow(unused_variables)]
fn op_000a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_bc());
    cpu.set_a(v);

    (8, 1)
}

// ld a,(de)
#[allow(unused_variables)]
fn op_001a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_de());
    cpu.set_a(v);

    (8, 1)
}

// ld b,(hl)
#[allow(unused_variables)]
fn op_0046(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_b(v);

    (8, 1)
}

// ld c,(hl)
#[allow(unused_variables)]
fn op_004e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_c(v);

    (8, 1)
}

// ld d,(hl)
#[allow(unused_variables)]
fn op_0056(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_d(v);

    (8, 1)
}

// ld e,(hl)
#[allow(unused_variables)]
fn op_005e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_e(v);

    (8, 1)
}

// ld h,(hl)
#[allow(unused_variables)]
fn op_0066(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_h(v);

    (8, 1)
}

// ld l,(hl)
#[allow(unused_variables)]
fn op_006e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_l(v);

    (8, 1)
}

// ld a,(hl)
#[allow(unused_variables)]
fn op_007e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    (8, 1)
}

// ld (bc),a
#[allow(unused_variables)]
fn op_0002(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

// ld (de),a
#[allow(unused_variables)]
fn op_0012(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_de(), v);

    (8, 1)
}

// ld (hl),b
#[allow(unused_variables)]
fn op_0070(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_b();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),c
#[allow(unused_variables)]
fn op_0071(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_c();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),d
#[allow(unused_variables)]
fn op_0072(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_d();
    mmu.set8(cpu.get_bc(), v);

    (8, 1)
}

// ld (hl),e
#[allow(unused_variables)]
fn op_0073(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_e();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),h
#[allow(unused_variables)]
fn op_0074(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_h();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),l
#[allow(unused_variables)]
fn op_0075(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_l();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ld (hl),a
#[allow(unused_variables)]
fn op_0077(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    (8, 1)
}

// ldi (hl),a
#[allow(unused_variables)]
fn op_0022(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

// ldd (hl),a
#[allow(unused_variables)]
fn op_0022(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(cpu.get_hl(), v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

// ldi a,(hl)
#[allow(unused_variables)]
fn op_002a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_add(1));

    (8, 1)
}

// ldd a,(hl)
#[allow(unused_variables)]
fn op_003a(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    cpu.set_a(v);

    cpu.set_hl(cpu.get_hl().wrapping_sub(1));

    (8, 1)
}

// ld (0xff00+c),a
#[allow(unused_variables)]
fn op_00e2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(0xff00 + cpu.get_c() as u16, v);

    (8, 1)
}

// ld a,(0xff00+c)
#[allow(unused_variables)]
fn op_00e2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(0xff00 + cpu.get_c() as u16);
    cpu.set_a(v);

    (8, 1)
}

// ld b,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_b(v);

    (8, 2)
}

// ld c,d8
#[allow(unused_variables)]
fn op_000e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_c(v);

    (8, 2)
}

// ld d,d8
#[allow(unused_variables)]
fn op_0016(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_d(v);

    (8, 2)
}

// ld e,d8
#[allow(unused_variables)]
fn op_001e(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_e(v);

    (8, 2)
}

// ld h,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_h(v);

    (8, 2)
}

// ld l,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_l(v);

    (8, 2)
}

// ld a,d8
#[allow(unused_variables)]
fn op_0006(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    cpu.set_a(v);

    (8, 2)
}

// ld (hl),d8
#[allow(unused_variables)]
fn op_0036(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_pc().wrapping_add(arg));
    mmu.set8(cpu.get_hl(), v);

    (12, 2)
}

// ld (0xff00+a8),a
#[allow(unused_variables)]
fn op_00e0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_a();
    mmu.set8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16, v);

    (12, 2)
}

// ld a,(0xff00+a8)
#[allow(unused_variables)]
fn op_00f0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(0xff00 + mmu.get8(cpu.get_pc().wrapping_add(arg)) as u16);
    cpu.set_a(v);

    (12, 2)
}

// ld bc,d16
#[allow(unused_variables)]
fn op_0001(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_bc(v);

    (12, 3)
}

// ld de,d16
#[allow(unused_variables)]
fn op_0011(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_de(v);

    (12, 3)
}

// ld hl,d16
#[allow(unused_variables)]
fn op_0021(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_hl(v);

    (12, 3)
}

// ld sp,d16
#[allow(unused_variables)]
fn op_0031(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_sp(v);

    (12, 3)
}

// ld (a16),sp
#[allow(unused_variables)]
fn op_0008(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp();
    mmu.set16(mmu.ge16(cpu.get_pc().wrapping_add(arg)), v);

    (20, 3)
}

// ld (a16),a
#[allow(unused_variables)]
fn op_00ea(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp();
    mmu.set8(mmu.get16(cpu.get_pc().wrapping_add(arg)), v);

    (16, 3)
}

// ld a,(a16)
#[allow(unused_variables)]
fn op_00fa(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(mmu.get16(cpu.get_pc().wrapping_add(arg)));
    cpu.set_a();

    (16, 3)
}

// ld sp,hl
#[allow(unused_variables)]
fn op_00f9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_hl();
    cpu.set_sp(v);

    (8, 1)
}

// ldhl sp,r8
#[allow(unused_variables)]
fn op_00f9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_sp();
    let q = mmuget.8(cpu_get_pc().wrapping_add(arg));
    let (v, h, c, z) = alu::add16e(p, q, false);
    cpu.set_hl(v);
    cpu.set_zf(false);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (12, 2)
}

// cp b
#[allow(unused_variables)]
fn op_00b8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp c
#[allow(unused_variables)]
fn op_00b9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp d
#[allow(unused_variables)]
fn op_00ba(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp e
#[allow(unused_variables)]
fn op_00bb(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp h
#[allow(unused_variables)]
fn op_00bc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp l
#[allow(unused_variables)]
fn op_00bd(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp a
#[allow(unused_variables)]
fn op_00bf(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

// cp (hl)
#[allow(unused_variables)]
fn op_00be(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_hl());
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 1)
}

// cp d8
#[allow(unused_variables)]
fn op_00be(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = cpu.get_a();
    let q = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let (_, h, c, z) = alu::sub8(p, q, false);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (8, 2)
}

// jp a16
#[allow(unused_variables)]
fn op_00c3(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
    cpu.set_pc(pc.wrapping_sub(3));
    //????
    (16, 3)
}

// jp nz,a16
#[allow(unused_variables)]
fn op_00c2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp z,a16
#[allow(unused_variables)]
fn op_00ca(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp nc,a16
#[allow(unused_variables)]
fn op_00d2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp c,a16
#[allow(unused_variables)]
fn op_00c2(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        let pc = mmu.get16(cpu.get_pc().wrapping_add(arg));
        cpu.set_pc(pc);
        return (16, 0)
    }

    (12, 3)
}

// jp hl
#[allow(unused_variables)]
fn op_00e9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.get_hl();
    cpu.set_pc(pc.wrapping_sub(1));

    (4, 1)
}

// jr r8
#[allow(unused_variables)]
fn op_0018(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
    let pc = cpu.get_pc().wrapping_add(alu::signed(p));
    cpu.set_pc(pc);

    (12, 2)
}

// jr z,r8
#[allow(unused_variables)]
fn op_0028(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = cpu.get_zf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr nz,r8
#[allow(unused_variables)]
fn op_0020(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = !cpu.get_zf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr c,r8
#[allow(unused_variables)]
fn op_0038(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = cpu.get_cf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// jr nc,r8
#[allow(unused_variables)]
fn op_0030(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flag = !cpu.get_cf();
    if flg{
        let p = mmu.get8(cpu.get_pc().wrapping_add(arg));
        let pc = cpu.getpc().wrapping_add(alu::signed(p));
        cpu.set_pc(pc);
        return (12, 2);
    }

    (8, 2)
}

// push bc
#[allow(unused_variables)]
fn op_00c5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_bc());

    (16, 1)
}

// push de
#[allow(unused_variables)]
fn op_00d5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_de());

    (16, 1)
}

// push hl
#[allow(unused_variables)]
fn op_00e5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_hl());

    (16, 1)
}

// push af
#[allow(unused_variables)]
fn op_00f5(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_af());

    (16, 1)
}

// pop bc
#[allow(unused_variables)]
fn op_00c1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_bc(v);

    (12, 1)
}

// pop de
#[allow(unused_variables)]
fn op_00d1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_de(v);

    (12, 1)
}

// pop hl
#[allow(unused_variables)]
fn op_00e1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_hl(v);

    (12, 1)
}

// pop af
#[allow(unused_variables)]
fn op_00f1(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.pop(mmu);
    cpu.set_af(v);

    (12, 1)
}

// call a16
#[allow(unused_variables)]
fn op_00cd(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(3));
    cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)).wrapping_sub(3));

    (24, 3)
}

// call nz,a16
#[allow(unused_variables)]
fn op_00c4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call z,a16
#[allow(unused_variables)]
fn op_00cc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call nc,a16
#[allow(unused_variables)]
fn op_00d4(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// call c,a16
#[allow(unused_variables)]
fn op_00dc(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_cf();
    if flg{
        cpu.push(mmu, cpu.get_pc().wrapping_add(3));
        cpu.set_pc(mmu.get16(cpu.get_pc().wrapping_add(arg)));
        return (24, 0);
    }
    

    (12, 3)
}

// ret
#[allow(unused_variables)]
fn op_00c9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);

    (16, 1)
}

// ret nz
#[allow(unused_variables)]
fn op_00c0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }

    (8, 1)
}

// ret z
#[allow(unused_variables)]
fn op_00c8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_zf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// ret nc
#[allow(unused_variables)]
fn op_00d0(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = !cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// ret c
#[allow(unused_variables)]
fn op_00d8(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let flg = cpu.get_cf();
    if flg {
        let pc = cpu.pop(mmu);
        cpu.set_pc(pc);
        return (20, 0);
    }
    
    (8, 1)
}

// reti
#[allow(unused_variables)]
fn op_00c9(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let pc = cpu.pop(mmu).wrapping_sub(1);
    cpu.set_pc(pc);
    cpu.enable_interrupt();

    (16, 1)
}

// rst 0x00
#[allow(unused_variables)]
fn op_00c7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x00u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x08
#[allow(unused_variables)]
fn op_00cf(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x08u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x10
#[allow(unused_variables)]
fn op_00d7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x10u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x18
#[allow(unused_variables)]
fn op_00df(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x18u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x20
#[allow(unused_variables)]
fn op_00e7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x20u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x28
#[allow(unused_variables)]
fn op_00ef(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x28u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x30
#[allow(unused_variables)]
fn op_00f7(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x30u16.wrapping_sub(1));

    (16, 1)
}

// rst 0x38
#[allow(unused_variables)]
fn op_00ff(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    cpu.push(mmu, cpu.get_pc().wrapping_add(1));
    cpu.set_pc(0x38u16.wrapping_sub(1));

    (16, 1)
}

// inc b
#[allow(unused_variables)]
fn op_0004(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_b();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc c
#[allow(unused_variables)]
fn op_000c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_c();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc d
#[allow(unused_variables)]
fn op_0014(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_d();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc e
#[allow(unused_variables)]
fn op_001c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_e();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc h
#[allow(unused_variables)]
fn op_0024(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_h();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc l
#[allow(unused_variables)]
fn op_002c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_l();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc a
#[allow(unused_variables)]
fn op_003c(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_a();
    let (v, h, c, z) = alu::add8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (4, 1)
}

// inc bc
#[allow(unused_variables)]
fn op_0003(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_bc().wrapping_add(1);
    cpu.set_bc(v);

    (8, 1)
}

// inc de
#[allow(unused_variables)]
fn op_0013(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_de().wrapping_add(1);
    cpu.set_de(v);

    (8, 1)
}

// inc hl
#[allow(unused_variables)]
fn op_0023(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_hl().wrapping_add(1);
    cpu.set_hl(v);

    (8, 1)
}

// inc sp
#[allow(unused_variables)]
fn op_0033(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_sp().wrapping_add(1);
    cpu.set_sp(v);

    (8, 1)
}

// dec b
#[allow(unused_variables)]
fn op_0005(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_b();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec c
#[allow(unused_variables)]
fn op_000d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_c();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec d
#[allow(unused_variables)]
fn op_0015(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_d();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec e
#[allow(unused_variables)]
fn op_001d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_e();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec h
#[allow(unused_variables)]
fn op_0025(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_h();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec l
#[allow(unused_variables)]
fn op_002d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_l();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// dec a
#[allow(unused_variables)]
fn op_003d(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu_get_a();
    let (v, h, c, z) = alu::sub8(v, 1, false);
    cpu.set_b(v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (4, 1)
}

// inc (hl)
#[allow(unused_variables)]
fn op_0034(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::add8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);

    (12, 1)
}

// dec (hl)
#[allow(unused_variables)]
fn op_0035(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = mmu.get8(cpu.get_hl());
    let (v, h, c, z) = alu::sub8(v, 1, false);
    mmu.set8(cpu.get_hl(), v);
    cpu.set_zf(z);
    cpu.set_nf(true);
    cpu.set_hf(h);

    (12, 1)
}

// dec bc
#[allow(unused_variables)]
fn op_000b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_bc().wrapping_sub(1);
    cpu.set_bc(v);

    (8, 1)
}

// dec de
#[allow(unused_variables)]
fn op_001b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_de().wrapping_sub(1);
    cpu.set_de(v);

    (8, 1)
}

// dec hl
#[allow(unused_variables)]
fn op_002b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_hl().wrapping_sub(1);
    cpu.set_hl(v);

    (8, 1)
}

// dec sp
#[allow(unused_variables)]
fn op_003b(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize){
    let v = cpu.get_sp().wrapping_sub(1);
    cpu.set_sp(v);

    (8, 1)
}

/// add a,b
#[allow(unused_variables)]
fn op_0080(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_b();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,c
#[allow(unused_variables)]
fn op_0081(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_c();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,d
#[allow(unused_variables)]
fn op_0082(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_d();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,e
#[allow(unused_variables)]
fn op_0083(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_e();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,h
#[allow(unused_variables)]
fn op_0084(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_h();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,l
#[allow(unused_variables)]
fn op_0085(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_l();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}

/// add a,a
#[allow(unused_variables)]
fn op_0087(arg : u16, cpu : &mut Cpu, mmu : &mut Mmu) -> (usize, usize) {
    let p = cpu.get_a();
    let q = cpu.get_a();
    let (v, h, c, z) = alu::add8(p, q, false);
    cpu.set_a(v);
    cpu.set_zf(z);
    cpu.set_nf(false);
    cpu.set_hf(h);
    cpu.set_cf(c);

    (4, 1)
}